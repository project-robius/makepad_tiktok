# makepad_tiktok

TikTok-like (Douyin) application implemented with Makepad

The user interface and functionality are not complete. Main functionality is to demonstrate Android video playing.

Known Issues:
* Currently supports Android only
* No audio yet

# Build Instructions


## 1. Setup Makepad

### Clone the Makepad repository
```
git clone git@github.com:makepad/makepad.git
```
OR
```
git clone https://github.com/makepad/makepad.git
```

### Change to latest 'rik' branch
```
git branch rik
```

### Install makepad subcommand for cargo
```
cd ~/makepad
cargo install --path ./tools/cargo_makepad
```

## 2. Get Project

### Clone the makepad_tiktok repo
```
git clone https://github.com/project-robius/makepad_tiktok.git
```

## 3. Android Build

### Install Android toolchain (First time)
```
rustup toolchain install nightly
cargo makepad android install-toolchain
```

### Install app on Android devivce or Android emulator
Open either the Android emulator or connect to a real Android device
use `adb` command to make sure there's a device connected properly
```
cd ~/makepad_tiktok
cargo makepad android run -p makepad_tiktok --release
```
