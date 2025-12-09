; --- Configuration ---
#define MyAppName "Monitor VS"
#define MyAppVersion "0.3"
#define MyAppPublisher "SathyaSubrahmanya"
#define MyAppExeName "monitor_vs.exe"
#define MyAppId "{{A5D3E8C1-4F2B-4A1D-9E8F-7C6B5A4D3E2F}"

[Setup]
; NOTE: The value of AppId uniquely identifies this application.
AppId={#MyAppId}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}

; Install into Program Files (64-bit)
DefaultDirName={autopf}\{#MyAppName}
; Important: Marks this as a 64-bit installer
ArchitecturesInstallIn64BitMode=x64

DisableProgramGroupPage=yes
; We need Admin rights to set permissions
PrivilegesRequired=admin

; Output settings
OutputDir=.
OutputBaseFilename=ProductivityMonitor_Setup_v{#MyAppVersion}
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern

; Icon for the installer (optional, remove if you don't have one)
; SetupIconFile=myicon.ico

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Tasks]
Name: "desktopicon"; Description: "{cm:CreateDesktopIcon}"; GroupDescription: "{cm:AdditionalIcons}"; Flags: unchecked

[Dirs]
; CRITICAL: Create the directory and grant "Users" write permission.
; Without this, your app will crash when trying to write logs or screenshots in Program Files.
Name: "{app}"; Permissions: users-modify
Name: "{app}\screenshots"; Permissions: users-modify

[Files]
; The Main Executable
Source: "target\release\{#MyAppExeName}"; DestDir: "{app}"; Flags: ignoreversion

; NOTE: We do NOT include session.key or logs here. 
; The app generates them, and we want to preserve them if the user updates.

[Icons]
; 1. The Monitor Shortcut (Hidden Mode / Standard)
Name: "{autoprograms}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Parameters: "monitor"; WorkingDir: "{app}"
Name: "{autodesktop}\{#MyAppName}"; Filename: "{app}\{#MyAppExeName}"; Parameters: "monitor"; Tasks: desktopicon; WorkingDir: "{app}"

; 2. The Verify Shortcut (Interactive Mode)
; We use 'cmd /k' to keep the window open so the user can read the result.
Name: "{autoprograms}\Verify Logs"; Filename: "cmd.exe"; Parameters: "/k ""{app}\{#MyAppExeName}"" verify"; IconFilename: "{app}\{#MyAppExeName}"; WorkingDir: "{app}"

; 3. Uninstall Shortcut
Name: "{autoprograms}\Uninstall {#MyAppName}"; Filename: "{uninstallexe}"

[Run]
; Option to run immediately after install
Filename: "{app}\{#MyAppExeName}"; Parameters: "monitor"; Description: "{cm:LaunchProgram,{#MyAppName}}"; Flags: nowait postinstall skipifsilent

[UninstallDelete]
; Cleanup logs and keys when uninstalled? 
; Uncomment the lines below if you want a "Clean" uninstall that deletes data.
; Type: files; Name: "{app}\productivity_log.csv"
; Type: files; Name: "{app}\session.key"
; Type: filesandordirs; Name: "{app}\screenshots"