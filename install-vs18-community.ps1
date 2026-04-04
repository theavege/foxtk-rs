<##
.SYNOPSIS
Download and install Visual Studio 18 Community with the components needed to build the example.

.DESCRIPTION
This script downloads the Visual Studio Community installer and runs it with the recommended
C++ build components required for building the Rust/FOX example on Windows.

.NOTES
Update the installer URL if Microsoft changes the Visual Studio 18 Community release link.
#>

$installerUrl = 'https://aka.ms/vs/18/release/vs_community.exe'
$installerPath = Join-Path -Path $env:TEMP -ChildPath 'vs_community_18.exe'

Write-Host "Downloading Visual Studio 18 Community installer to $installerPath..."
Invoke-WebRequest -Uri $installerUrl -OutFile $installerPath

$addComponents = @(
    'Microsoft.VisualStudio.Workload.NativeDesktop',
    'Microsoft.VisualStudio.Workload.VCTools',
    'Microsoft.VisualStudio.Component.VC.Tools.x86.x64',
    'Microsoft.VisualStudio.Component.Windows11SDK.23000',
    'Microsoft.VisualStudio.Component.VC.CMake.Project',
    'Microsoft.VisualStudio.Component.VC.Redist.14.Latest',
    'Microsoft.VisualStudio.Component.VC.ATL',
    'Microsoft.VisualStudio.Component.VC.ATLMFC',
    'Microsoft.VisualStudio.Component.VC.CoreBuildTools'
)

$arguments = @(
    '--quiet',
    '--wait',
    '--norestart',
    '--includeRecommended'
)

$addComponents | ForEach-Object { $arguments += "--add $_" }

Write-Host "Starting Visual Studio 18 Community installer..."
Start-Process -FilePath $installerPath -ArgumentList $arguments -Wait -NoNewWindow

Write-Host 'Visual Studio 18 Community installation completed.'
Write-Host 'If installation fails, rerun the script with administrative privileges.'
