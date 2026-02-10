====================================
    Handy - Portable Edition
====================================

Thank you for downloading Handy!

This is the portable version of Handy, which means you can run it directly
without installing it on your system. All app data and settings are stored
in the same directory as the executable, making it truly portable.


WHAT'S IN THIS PACKAGE
-----------------------
- Handy.exe           : The main application
- .portable           : Marker file that enables portable mode
- data/               : Directory for all app data (auto-created)
- resources/          : Required application resources (models, icons, sounds)
- PORTABLE_README.txt : This file


HOW PORTABLE MODE WORKS
-----------------------
The .portable file next to the executable enables portable mode, which means:

1. All settings are stored in: data/settings_store.json
2. Transcription history database: data/history.db
3. Downloaded models: data/models/
4. Recorded audio files: data/recordings/
5. Application logs: data/logs/

You can move the entire Handy folder anywhere (USB drive, different computer)
and all your settings and data will move with it!


HOW TO USE
----------
1. Extract all files from this ZIP archive to a folder of your choice
2. Double-click Handy.exe to start the application
3. That's it! No installation required.


SYSTEM REQUIREMENTS
-------------------
- Windows 10 or Windows 11 (x64 or ARM64)
- Microsoft Edge WebView2 Runtime (usually pre-installed on Windows 11)
  - If WebView2 is not installed, download it from:
    https://go.microsoft.com/fwlink/p/?LinkId=2124703


FIRST-TIME SETUP
----------------
On first run, Handy will:
1. Create the data directory and initialize settings
2. Download the speech recognition model (Small, Medium, Turbo, or Large)
3. Configure your preferred keyboard shortcut
4. Select your microphone

All settings and data are stored in the data/ folder next to the executable.


FEATURES
--------
- Local speech-to-text transcription using Whisper AI
- Global keyboard shortcuts for quick access
- Support for multiple languages
- Automatic text insertion into any application
- Privacy-focused: all processing happens on your device


SWITCHING BETWEEN PORTABLE AND INSTALLED MODE
----------------------------------------------
- To use portable mode: Keep the .portable file
- To use normal installed mode: Delete the .portable file

Without the .portable file, Handy will use the standard Windows app data
folder (%APPDATA%\handy\) instead of the local data/ directory.


TROUBLESHOOTING
---------------
If Handy doesn't start:
1. Make sure WebView2 is installed (see System Requirements)
2. Check if your antivirus is blocking the application
3. Try running as administrator (right-click > Run as administrator)
4. Check the logs in: data/logs/

If settings or models aren't being saved:
1. Make sure the .portable file exists next to Handy.exe
2. Verify you have write permissions to the Handy folder
3. Check if the data/ folder was created successfully


UPDATES
-------
This portable version does not auto-update. To update:
1. Download the latest portable version
2. Extract to a new folder
3. Copy your data/ folder from the old version to the new version
4. Your settings and models will be preserved


MORE INFORMATION
----------------
- Website: https://handy.computer
- GitHub: https://github.com/cjpais/Handy
- Issues: https://github.com/cjpais/Handy/issues


LICENSE
-------
MIT License - See https://github.com/cjpais/Handy/blob/main/LICENSE


====================================
        Enjoy using Handy!
====================================
