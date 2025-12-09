
# Productivity Monitor

A secure, lightweight tool to track computer activity. It logs active window titles, counts keystrokes/mouse events, and takes smart, deduplicated screenshots. All logs are cryptographically signed to prevent tampering.

## 1\. Installation

1.  **Download** the latest release (Zip or Installer).
2.  **Run** `Monitor_VS_Setup_v0.5.1.exe`.
3.  **SmartScreen Warning:** If Windows says "Protected your PC", click **More Info** → **Run Anyway**.
4.  Follow the prompts to install to `C:\Program Files\Monitor VS`.

## 2\. How to Run

### Basic Mode (Default)

Simply double-click the **Productivity Monitor** shortcut on your Desktop.

  * **Log Interval:** Every 60 seconds.
  * **Screenshot Interval:** Every 300 seconds.
  * **Note:** The app runs silently in the background. To stop it, use Task Manager.

### Advanced Mode (Custom Intervals)

To change the settings, you must run the application from the Command Line (CMD or PowerShell).

1.  Open **Command Prompt** or **PowerShell**.
2.  Navigate to the installation folder:
    ```powershell
    cd "C:\Program Files\Monitor VS"
    ```
3.  Run the monitor command with your desired flags:
    ```powershell
    .\monitor_vs.exe monitor --log-interval 30 --screenshot-interval 300
    ```
    *(This example logs every 30 seconds and takes a screenshot every 5 minutes.)*
    
## 3\. Verifying Logs

To ensure `productivity_log.csv` has not been edited manually, use the verify command via the terminal.

1.  Open **Command Prompt** or **PowerShell**.
2.  Navigate to the installation folder (if not already there):
    ```powershell
    cd "C:\Program Files\Monitor VS"
    ```
3.  Run the verification command:
    ```powershell
    .\monitor_vs.exe verify
    ```
4.  The application will scan the log file and the session key.

**Output Meanings:**

  * `✅ LOG INTEGRITY VERIFIED`: The file is authentic.
  * `❌ INTEGRITY COMPROMISED`: The file has been tampered with. The specific line number of the fake entry will be displayed.

## 4\. Output Files

All data is stored in the installation directory:

  * **`productivity_log.csv`**: The main activity log.
  * **`screenshots/`**: Folder containing compressed JPEG screenshots.
  * **`session.key`**: The cryptographic key used for signing logs.
      * **⚠️ IMPORTANT:** Do not delete `session.key`, or you will lose the ability to verify past logs.

## 5\. Troubleshooting

  * **Antivirus:** If the file is deleted automatically, add an **Exclusion** in Windows Defender for `C:\Program Files\Productivity Monitor`.
  * **Black Screenshots:** Banking apps, Netflix, and the Windows Lock Screen prevent screen capture for security reasons.
