#!/usr/bin/env pwsh

#-------------------------------------------------------------------------------
# FILTERS AND FUNCTIONS
#-------------------------------------------------------------------------------

Filter Out-Log {
    $(
        If (! (Test-Path -Path Variable:LastExitCode)) {
            "$(Get-Date -uformat '%y-%m-%d_%T')$([char]27)[33m {0}$([char]27)[0m" -f $_
        } ElseIf ($LastExitCode -eq 0) {
            "$(Get-Date -uformat '%y-%m-%d_%T')$([char]27)[32m {0}$([char]27)[0m" -f $_
        } Else {
            "$(Get-Date -uformat '%y-%m-%d_%T')$([char]27)[31m [{0}]`t{1}$([char]27)[0m" -f $LastExitCode, $_
        }
    ) | Out-Host
}

Filter Get-Packages {
    "Downloading Visual Studio 18 Community installer to $installerPath..." | Out-Log
    Invoke-WebRequest $_
    $_.OutFile
}

Filter Install-Packages {
    $arguments = @(
        '--quiet',
        '--wait',
        '--norestart',
        '--includeRecommended'
    )
    @(
        'Microsoft.VisualStudio.Workload.NativeDesktop',
        'Microsoft.VisualStudio.Workload.VCTools',
        'Microsoft.VisualStudio.Component.VC.Tools.x86.x64',
        'Microsoft.VisualStudio.Component.Windows11SDK.23000',
        'Microsoft.VisualStudio.Component.VC.CMake.Project',
        'Microsoft.VisualStudio.Component.VC.Redist.14.Latest',
        'Microsoft.VisualStudio.Component.VC.ATL',
        'Microsoft.VisualStudio.Component.VC.ATLMFC',
        'Microsoft.VisualStudio.Component.VC.CoreBuildTools'
    ) | ForEach-Object { $arguments += "--add $_" }
    "Starting Visual Studio 18 Community installer..."  | Out-Log
    Start-Process -FilePath $_ -ArgumentList $arguments -Wait -NoNewWindow
    Remove-Item $_
    $env:LIBCLANG_PATH='{0}\Microsoft Visual Studio\18\Community\VC\Tools\Llvm\x64\lib' -f $Env:PROGRAMFILES
}

#-------------------------------------------------------------------------------
# MAIN ENDPOINT
#-------------------------------------------------------------------------------

$ErrorActionPreference = 'stop'
Set-PSDebug -Strict #-Trace 1
@{
    Uri = 'https://aka.ms/vs/17/release/vs_community.exe'
    OutFile = (New-TemporaryFile).FullName
} | Get-Packages | Install-Packages
& cargo clippy --quiet --example simple | Out-Log
Exit($LastExitCode)
