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

Filter Install-VsBuildTools {
    # VS 2022 is installer/component channel "17". Component IDs are pinned for
    # reproducibility; see https://aka.ms/vs/workloads for the current catalog if
    # any of these ever get dropped by the installer.
    $components = @(
        'Microsoft.VisualStudio.Workload.VCTools',
        'Microsoft.VisualStudio.Component.VC.ATL',
        'Microsoft.VisualStudio.Component.VC.ATLMFC',
        'Microsoft.VisualStudio.Component.VC.CMake.Project',
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
    $env:LIBCLANG_PATH = '{0}\Microsoft Visual Studio\2022\Community\VC\Tools\Llvm\x64\lib' -f $Env:PROGRAMFILES

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
Set-PSDebug -Strict #-Trace 1

if (Get-Module -ListAvailable -Name PSScriptAnalyzer) {
    Invoke-ScriptAnalyzer -EnableExit -Path $PSCommandPath
} else {
    'PSScriptAnalyzer not installed -- skipping lint pass.' | Write-Info
}

If ($args.count -gt 0) {
    Switch ($args[0]) {
        'setup' {
            try {
                'Found cmake at {0}' -f (Get-Command 'cmake').Source | Write-Info
            }
            catch {
                'cmake not found: {0}' -f $_ | Write-Err
                @(
                    'https://aka.ms/vs/17/release/vs_community.exe'
                ) | Save-Installer | Install-VsBuildTools
            }
        }
        'build' {
            & cargo clippy --features="all" --quiet --examples | Out-Log
            If ($LastExitCode -ne 0) {
                'clippy failed, skipping build' | Write-Err
                Exit($LastExitCode)
            }
            & cargo build --features="all" --release --examples | Out-Log
        }
        default {
            "Usage: make.ps1 {setup|build}" | Write-Info
        }
    }
}

Exit($(If ($null -eq $LastExitCode) { 0 } Else { $LastExitCode }))
