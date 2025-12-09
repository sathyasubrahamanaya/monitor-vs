# User Guide: Productivity Monitor

## 1. Installation
Because this software was built internally and not purchased from a store, Windows might treat it as "unrecognized." Follow these steps to install it successfully.

1.  **Download & Extract:** Download the `ProductivityMonitor_Setup.zip` file and extract it.
2.  **Run the Installer:** Double-click `ProductivityMonitor_Setup.exe`.
3.  **Bypass SmartScreen:**
    * If you see a blue window saying *"Windows protected your PC"*:
    * Click **More Info**.
    * Click the **Run Anyway** button.
4.  **Finish Setup:** Follow the installation prompts. We recommend keeping the "Create Desktop Shortcut" option checked.

---

## 2. How to Start Monitoring
Once installed, the application is designed to run silently in the background.

1.  Double-click the **Productivity Monitor** icon on your Desktop.
2.  **That’s it!** The program is now running.
    * It does **not** open a visible window (to stay out of your way).
    * It will automatically log activity and take screenshots based on the default settings (every 60 seconds).

**To Stop Monitoring:**
* Locate the window (if you ran it via command line) and press `Ctrl+C`.
* OR, use Task Manager to end the `productivity_monitor.exe` process.

---

## 3. How to Verify Log Integrity
This tool ensures that work logs have not been tampered with. To check if a log file is valid:

1.  Open the **Start Menu**.
2.  Go to the **Productivity Monitor** folder.
3.  Click on **Verify Logs**.
4.  A black window will appear and scan your data.

**Understanding the Results:**
* **✅ LOG INTEGRITY VERIFIED:** The file is authentic. No data has been changed.
* **❌ INTEGRITY COMPROMISED:** Someone has manually edited the CSV file (changed timestamps, status, etc.). The line number of the fake entry will be shown.

---

## 4. Where is my Data?
By default, all data is stored in the installation folder.

**Location:** `C:\Program Files\Productivity Monitor`

You will find:
* `productivity_log.csv`: The spreadsheet containing timestamps, active apps, and window titles.
* `screenshots/`: A folder containing the images of your screen.
* `session.key`: **IMPORTANT** - This is the security key used to sign your logs.

> **⚠️ WARNING:** Never delete or lose `session.key`. If this file is lost, you will be unable to verify the integrity of your previous logs.

---

## 5. Advanced Settings (Changing Intervals)
The default setting is to log and screenshot every **60 seconds**. To change this:

1.  Right-click the **Productivity Monitor** shortcut on your Desktop.
2.  Select **Properties**.
3.  Find the **Target** box. It will look like this:
    `"C:\...\productivity_monitor.exe" monitor`
4.  Add your custom settings to the end.
    * *Example (Log every 30s, Screenshot every 5 mins):*
        `...productivity_monitor.exe" monitor --log-interval 30 --screenshot-interval 300`
5.  Click **Apply** and **OK**.
6.  Restart the application for changes to take effect.

---

## 6. Troubleshooting

**Q: The app crashes immediately upon opening.**
* **Fix:** Ensure you are not trying to run it from inside the Zip file. Extract it first.
* **Fix:** If installed in Program Files, ensure you used the official Installer (which sets the correct write permissions).

**Q: Screenshots are black.**
* **Fix:** Some secure applications (like banking websites or Netflix) block screenshots. This is normal behavior.
* **Fix:** If the computer is on the Lock Screen, Windows prevents screenshots for security reasons.

**Q: My Antivirus deleted the file.**
* **Fix:** Open your Antivirus settings and add an **Exclusion** for `C:\Program Files\Productivity Monitor`. This happens because the tool records keystrokes and screens, which mimics the behavior of surveillance software.