#!/usr/bin/env pwsh

[CmdletBinding()]
param (
    [Parameter(Mandatory=$true)]
    [ValidateSet('setup', 'build')]
    [string]$Action
)

$ErrorActionPreference = 'Stop'
Set-PSDebug -Strict

Function Write-Log {
    process {
        $timestamp = Get-Date -uformat '%y-%m-%d_%T'
        Write-Host "$timestamp $_" -ForegroundColor Cyan
    }
}

Function Install-VSBuildTools {
    $installerUrl = 'https://aka.ms/vs/17/release/vs_community.exe'
    $outFile = '{0}.exe' -f (New-TemporaryFile).FullName
    
    "Downloading VS Installer to $outFile..." | Write-Log
    Invoke-WebRequest -Uri $installerUrl -OutFile $outFile

    $arguments = @(
        '--quiet', '--wait', '--norestart', '--includeRecommended',
        '--add', 'Microsoft.VisualStudio.Workload.NativeDesktop',
        '--add', 'Microsoft.VisualStudio.Workload.VCTools',
        '--add', 'Microsoft.VisualStudio.Component.VC.CMake.Project',
        '--add', 'Microsoft.VisualStudio.Component.VC.Tools.x86.x64',
        '--add', 'Microsoft.VisualStudio.Component.Windows11SDK.23000'
    )
    
    "Starting Visual Studio 17 Community installer..." | Write-Log
    Start-Process -FilePath $outFile -ArgumentList $arguments -Wait -NoNewWindow
    Remove-Item $outFile

    $env:LIBCLANG_PATH = "$env:PROGRAMFILES\Microsoft Visual Studio\2022\Community\VC\Tools\Llvm\x64\lib"
}

Switch ($Action) {
    'setup' {
        if (-not (Get-Command 'cmake' -ErrorAction Ignore)) {
            "CMake not found. Initiating setup..." | Write-Log
            Install-VSBuildTools
        } else {
            "CMake already installed: $((Get-Command 'cmake').Source)" | Write-Log
        }
    }
    'build' {
        "Running Cargo Clippy..." | Write-Log
        & cargo clippy --features="all" --quiet --examples
        
        "Building Cargo project..." | Write-Log
        & cargo build --features="all" --release --examples
    }
}

Exit $LASTEXITCODE
