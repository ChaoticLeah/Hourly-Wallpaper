# Hourly-Wallpaper

Is a small rust app that lets you configure what you want your wallpaper to be, and it will take that and give you a new wallpaper as many times per seconds/minutes/hours based on the prompts.
This will make sure you are never board of your wallpaper.

## How to use

Go to the releases of this repo, and download the file. Then unzip the file and to run it in the background run "Start.vbs"(This allows it to run in the background). To end the program run "Stop.bat"
To configure what kind of wallpapers you get, and other settings you can open "config.yaml"

MAKE SURE TO KEEP ALL THE FILES IN THE SAME DIRECTORY.

### Running the program on startup

1. Press WINDOWS KEY + R
2. Type `shell:startup` and hit ok
3. Copy the "Start.vbs" into this Startup folder
4. Edit the copied "Start.vbs" and change `objShell.Run "hourly_wallpaper.exe", 0, True` to `objShell.Run "C:/path/to/file/hourly_wallpaper.exe", 0, True`. Make sure to give it a full path.
5. Run this file and check to make sure your background changes. If it fails run the exe and see if it works. If it works try this over again, if not try to detail the bug if you are going to submit a bug report.

## Building or Testing the project

rename "config.example.yaml" to "config.yaml" and place it in target/debug/config.yaml
