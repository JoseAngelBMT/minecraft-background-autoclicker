# Minecraft Background Autoclicker

A lightweight **background autoclicker** for **Minecraft** written in Rust.  
It allows you to **auto-click inside Minecraft without needing the window in focus**, making it perfect for **AFK farming** or idle actions while you do other tasks.

---

## 🧩 Features
- Works **in the background** – Minecraft doesn’t need to be the active window.
- Automatically detects any Minecraft window (e.g. `Minecraft 1.21.10`, `Minecraft 1.20.1`, etc).
- Simple and lightweight – no external dependencies or overlays.
- Interval input on startup (choose how often clicks happen).
- Safe and easy to stop anytime with `Ctrl + C`.

---

## ⚙️ Requirements

1. **Minecraft must not pause when losing focus.**  
   By default, Minecraft stops the game when you switch to another window.  
   You need to disable that behavior:

    - Open your Minecraft configuration file:
      ```
      %appdata%\.minecraft\options.txt
      ```
    - Find the line:
      ```
      pauseOnLostFocus:true
      ```
    - Change it to:
      ```
      pauseOnLostFocus:false
      ```
    - Save and close the file.

   > This setting allows the game to keep running while you’re using other applications.

2. Windows operating system (this program uses the Win32 API).
3. The game window title must include “Minecraft” followed by its version (e.g. `Minecraft 1.21.10 - Singleplayer`).

---

## 🚀 How to Use

1. **Download** the latest release from the [Releases page](https://github.com/<your_username>/minecraft_background_autoclicker/releases).
2. **Run the `.exe` file.**
3. The program will:
    - Detect your Minecraft window automatically.
    - Ask you for a click interval in seconds (e.g., `0.25`).
    - Wait 5 seconds, then start sending left-clicks directly to the Minecraft window — even while it’s in the background.

Example output:
```txt
Minecraft window detected.
Enter click interval in seconds: 0.25
Starting in 5 seconds...
Auto-clicking every 0.25 seconds.
Press Ctrl+C to stop.
Stopped.
```

---

## 🧱 Build from Source (optional)

If you have [Rust](https://www.rust-lang.org/tools/install) installed:

```bash
git clone https://github.com/<your_username>/minecraft_background_autoclicker.git
cd minecraft_background_autoclicker
cargo build --release
```

You’ll find the compiled executable in:

target\release\minecraft_background_autoclicker.exe

---
## ⚠️ Notes

This tool is for personal use only.

Using autoclickers on multiplayer servers may violate their terms of service.

It does not move the mouse or take focus — all clicks are sent directly to the Minecraft window via background messages.


