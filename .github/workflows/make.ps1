#!/usr/bin/env pwsh

#-------------------------------------------------------------------------------
# FILTERS AND FUNCTIONS
#-------------------------------------------------------------------------------

# Colored logging split into explicit variants instead of inferring severity from
# $LastExitCode (that global is easy to leave stale between calls, and PowerShell
# exceptions never set it in the first place, so a caught error used to print in
# "info" yellow instead of red).

Filter Write-Info {
    "$(Get-Date -uformat '%y-%m-%d_%T')$([char]27)[33m {0}$([char]27)[0m" -f $_ | Out-Host
}

Filter Write-Err {
    "$(Get-Date -uformat '%y-%m-%d_%T')$([char]27)[31m {0}$([char]27)[0m" -f $_ | Out-Host
}

# For logging right after a native command: reports the actual exit code, colored
# green/red accordingly. Only meaningful immediately after a native (non-cmdlet)
# invocation -- $LastExitCode is untouched by cmdlets, so don't use this for
# arbitrary messages elsewhere.
Filter Out-Log {
    $(
        If ($LastExitCode -eq 0) {
            "$(Get-Date -uformat '%y-%m-%d_%T')$([char]27)[32m {0}$([char]27)[0m" -f $_
        } Else {
            "$(Get-Date -uformat '%y-%m-%d_%T')$([char]27)[31m [{0}]`t{1}$([char]27)[0m" -f $LastExitCode, $_
        }
    ) | Out-Host
}

Filter Save-Installer {
    $OutFile = Join-Path ([System.IO.Path]::GetTempPath()) ('{0}_{1}' -f [guid]::NewGuid(), (Split-Path -Path $_ -Leaf).Split('?')[0])
    Invoke-WebRequest -OutFile $OutFile -Uri $_
    'Downloaded {0} to {1}' -f $_, $OutFile | Write-Info
    $OutFile
}

# Refreshes the current process's PATH from the registry (Machine + User) so that
# tools installed by a child process (e.g. the VS installer) are visible without
# starting a new shell.
function Update-SessionPath {
    $machine = [System.Environment]::GetEnvironmentVariable('PATH', 'Machine')
    $user = [System.Environment]::GetEnvironmentVariable('PATH', 'User')
    $env:PATH = @($machine, $user) -join ';'
}

# Resolve the native build tools from the selected Visual Studio instance so the
# build does not depend on a specific edition or major-version directory.
function Initialize-VsBuildTools {
    $vswhere = Join-Path ([Environment]::GetFolderPath('ProgramFilesX86')) 'Microsoft Visual Studio\Installer\vswhere.exe'
    if (-not (Test-Path -LiteralPath $vswhere -PathType Leaf)) {
        throw 'vswhere.exe was not found; install Visual Studio Installer and the C++ build tools.'
    }

    $components = @(
        'Microsoft.VisualStudio.Component.VC.Tools.x86.x64',
        'Microsoft.VisualStudio.Component.VC.Llvm.Clang',
        'Microsoft.VisualStudio.Component.VC.CMake.Project'
    )
    $libclang = @(& $vswhere -latest -products '*' -requires $components -find 'VC\Tools\Llvm\x64\lib\libclang.lib')
    if ($LASTEXITCODE -ne 0 -or $libclang.Count -eq 0) {
        throw 'Visual Studio with the x64 C++ tools and LLVM/Clang was not found; run make.ps1 setup.'
    }
    $ninja = @(& $vswhere -latest -products '*' -requires $components -find 'Common7\IDE\CommonExtensions\Microsoft\CMake\Ninja\ninja.exe')
    if ($LASTEXITCODE -ne 0 -or $ninja.Count -eq 0) {
        throw 'Ninja was not found in the selected Visual Studio installation; run make.ps1 setup.'
    }

    $env:LIBCLANG_PATH = Split-Path -Parent $libclang[0]
    $ninjaDirectory = Split-Path -Parent $ninja[0]
    $env:PATH = @($ninjaDirectory, $env:PATH) -join ';'
    'Using libclang at {0}' -f $env:LIBCLANG_PATH | Write-Info
    'Using Ninja at {0}' -f (Join-Path $ninjaDirectory 'ninja.exe') | Write-Info
}

function Test-CMakeBuildCache([string]$BuildDirectory) {
    Initialize-VsBuildTools

    if ([string]::IsNullOrWhiteSpace($BuildDirectory)) {
        throw 'Cargo did not provide the release foxtk-sys CMake build directory.'
    }
    $buildDirectory = $BuildDirectory.Trim()
    if (-not (Test-Path -LiteralPath (Join-Path $buildDirectory 'build.ninja') -PathType Leaf)) {
        throw "The recorded release Ninja build does not exist: $buildDirectory"
    }

    $ninja = Get-Command 'ninja.exe' -ErrorAction Stop
    $cmakeLists = Join-Path $PWD 'foxtk-sys\src\CMakeLists.txt'

    $output = (& $ninja.Source -C $buildDirectory 2>&1 | Out-String)
    if ($LASTEXITCODE -ne 0) {
        throw "Ninja no-op build failed: $output"
    }
    if ($output -match 'Re-running CMake') {
        throw 'Ninja unexpectedly reran CMake with unchanged inputs.'
    }

    $original = [System.IO.File]::ReadAllBytes($cmakeLists)
    try {
        $marker = [System.Text.Encoding]::UTF8.GetBytes("`n# CI cache regeneration check`n")
        $mutated = [byte[]]::new($original.Length + $marker.Length)
        [Array]::Copy($original, $mutated, $original.Length)
        [Array]::Copy($marker, 0, $mutated, $original.Length, $marker.Length)
        [System.IO.File]::WriteAllBytes($cmakeLists, $mutated)

        $output = (& $ninja.Source -C $buildDirectory 2>&1 | Out-String)
        if ($LASTEXITCODE -ne 0) {
            throw "Ninja failed after CMakeLists.txt changed: $output"
        }
        if ($output -notmatch 'Re-running CMake') {
            throw "Ninja did not regenerate CMake after CMakeLists.txt changed: $output"
        }
    }
    finally {
        [System.IO.File]::WriteAllBytes($cmakeLists, $original)
        & $ninja.Source -C $buildDirectory 2>&1 | Out-Host
        if ($LASTEXITCODE -ne 0) {
            throw 'Ninja failed to regenerate after restoring CMakeLists.txt.'
        }
    }

    'Ninja reused its existing configuration and regenerated it after CMakeLists.txt changed.' | Write-Info
}

Filter Install-VsBuildTools {
    # VS 2022 is installer/component channel "17". Component IDs are pinned for
    # reproducibility; see https://aka.ms/vs/workloads for the current catalog if
    # any of these ever get dropped by the installer.
    $components = @(
        'Microsoft.VisualStudio.Workload.VCTools',
        'Microsoft.VisualStudio.Component.VC.ATL',
        'Microsoft.VisualStudio.Component.VC.ATLMFC',
        'Microsoft.VisualStudio.Component.VC.CMake.Project',
        'Microsoft.VisualStudio.Component.VC.Llvm.Clang',
        'Microsoft.VisualStudio.Component.VC.CoreBuildTools',
        'Microsoft.VisualStudio.Component.VC.Redist.14.Latest',
        'Microsoft.VisualStudio.Component.VC.Tools.x86.x64',
        'Microsoft.VisualStudio.Component.Windows11SDK.23000'
    )

    $arguments = @(
        '--quiet',
        '--wait',
        '--norestart',
        '--includeRecommended'
    )
    ForEach ($component in $components) {
        # Each --add and its ID must be separate array elements: Start-Process
        # quotes every element independently, so "--add $id" as one element
        # reaches the installer as a single token instead of two, and it's
        # silently ignored.
        $arguments += '--add', $component
    }

    'Starting Visual Studio 2022 Community installer...' | Write-Info
    Start-Process -FilePath $_ -ArgumentList $arguments -Wait -NoNewWindow
    Remove-Item $_

    Update-SessionPath

    try {
        'Found cmake at {0}' -f (Get-Command 'cmake').Source | Write-Info
    } catch {
        'cmake still not found on PATH after install -- open a new shell and re-run setup.' | Write-Err
        Exit 1
    }
}

#-------------------------------------------------------------------------------
# MAIN ENDPOINT
#-------------------------------------------------------------------------------

$ErrorActionPreference = 'stop'
$LastExitCode = 0 # Set-PSDebug -Strict below throws on reading $LastExitCode before any native command sets it
Set-PSDebug -Strict #-Trace 1

if (Get-Module -ListAvailable -Name PSScriptAnalyzer) {
    Invoke-ScriptAnalyzer -EnableExit -Path $PSCommandPath
} else {
    'PSScriptAnalyzer not installed -- skipping lint pass.' | Write-Info
}

If ($args.count -gt 0) {
    Switch ($args[0]) {
        'setup' {
            $toolsReady = $false
            try {
                'Found cmake at {0}' -f (Get-Command 'cmake').Source | Write-Info
                Initialize-VsBuildTools
                $toolsReady = $true
            }
            catch {
                'Required Visual Studio build tools not found: {0}' -f $_ | Write-Err
            }
            if (-not $toolsReady) {
                @(
                    'https://aka.ms/vs/17/release/vs_community.exe'
                ) | Save-Installer | Install-VsBuildTools
                Initialize-VsBuildTools
            }
        }
        'build' {
            Initialize-VsBuildTools
            & cargo clippy --quiet --examples | Out-Log
            If ($LastExitCode -ne 0) {
                'clippy failed, skipping build' | Write-Err
                Exit($LastExitCode)
            }
            $previousCacheCheck = $env:FOXTK_CMAKE_CACHE_CHECK
            $env:FOXTK_CMAKE_CACHE_CHECK = [guid]::NewGuid().ToString()
            $buildOutput = & cargo build --release --examples --verbose 2>&1
            $buildStatus = $LASTEXITCODE
            if ($null -eq $previousCacheCheck) {
                Remove-Item Env:FOXTK_CMAKE_CACHE_CHECK -ErrorAction SilentlyContinue
            } else {
                $env:FOXTK_CMAKE_CACHE_CHECK = $previousCacheCheck
            }
            $LastExitCode = $buildStatus
            If ($buildStatus -ne 0) {
                $buildOutput | Out-Log
                'build failed, skipping documentation checks' | Write-Err
                Exit($buildStatus)
            }

            $buildLog = $buildOutput -join "`n"
            $buildDirectoryMatch = [regex]::Match(
                $buildLog,
                'foxtk-sys-cmake-build-dir=(?<path>[^\r\n]+)'
            )
            if (-not $buildDirectoryMatch.Success) {
                $buildOutput | Out-Log
                throw 'Could not find foxtk-sys CMake build directory in Cargo verbose output.'
            }
            $buildOutput |
                ForEach-Object { "$_" } |
                Where-Object { $_ -match 'foxtk-sys-cmake-build-dir=|Finished .*release.*profile' } |
                Out-Log
            $buildDirectory = $buildDirectoryMatch.Groups['path'].Value.Trim()
            $buildDirectoryOutput = if ($env:RUNNER_TEMP) {
                Join-Path $env:RUNNER_TEMP 'foxtk-sys-cmake-build-dir.txt'
            } else {
                Join-Path ([System.IO.Path]::GetTempPath()) 'foxtk-sys-cmake-build-dir.txt'
            }
            [System.IO.File]::WriteAllText($buildDirectoryOutput, $buildDirectory)
            'Captured release CMake build directory: {0}' -f $buildDirectory | Write-Info

            if ($env:GITHUB_OUTPUT) {
                [System.IO.File]::AppendAllText(
                    $env:GITHUB_OUTPUT,
                    "foxtk_cmake_build_dir=$buildDirectory`n",
                    [System.Text.UTF8Encoding]::new($false)
                )
            }
            & cargo doc --no-deps | Out-Log
            If ($LastExitCode -ne 0) {
                'documentation build failed' | Write-Err
                Exit($LastExitCode)
            }
            & cargo fmt --check --all | Out-Log
            If ($LastExitCode -ne 0) {
                'format check failed' | Write-Err
                Exit($LastExitCode)
            }
        }
        'cmake-cache-check' {
            $buildDirectory = if ($args.Count -gt 1) {
                $args[1]
            } elseif ($env:RUNNER_TEMP) {
                [System.IO.File]::ReadAllText((Join-Path $env:RUNNER_TEMP 'foxtk-sys-cmake-build-dir.txt'))
            } else {
                [System.IO.File]::ReadAllText((Join-Path ([System.IO.Path]::GetTempPath()) 'foxtk-sys-cmake-build-dir.txt'))
            }
            Test-CMakeBuildCache $buildDirectory
        }
        default {
            "Usage: make.ps1 {setup|build|cmake-cache-check}" | Write-Info
        }
    }
}

Exit($(If ($null -eq $LastExitCode) { 0 } Else { $LastExitCode }))
