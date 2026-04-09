$ErrorActionPreference = 'stop'
Set-PSDebug -Strict #-Trace 1
$installerUrl = 'https://aka.ms/vs/17/release/vs_community.exe'
$installerPath = Join-Path -Path $env:TEMP -ChildPath 'vs_community_18.exe'
Write-Host "Downloading Visual Studio 18 Community installer to $installerPath..."
Invoke-WebRequest -Uri $installerUrl -OutFile $installerPath
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
Write-Host "Starting Visual Studio 18 Community installer..."
Start-Process -FilePath $installerPath -ArgumentList $arguments -Wait -NoNewWindow
$env:LIBCLANG_PATH='{0}\Microsoft Visual Studio\18\Community\VC\Tools\Llvm\x64\lib' -f $Env:PROGRAMFILES
