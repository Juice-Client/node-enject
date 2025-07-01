![Version](https://img.shields.io/npm/v/@juice-client/node-enject)
![License](https://img.shields.io/github/license/Juice-Client/node-enject)
![GitHub stars](https://img.shields.io/github/stars/Juice-Client/node-enject)
![GitHub forks](https://img.shields.io/github/forks/Juice-Client/node-enject)
![GitHub last commit](https://img.shields.io/github/last-commit/Juice-Client/node-enject)
![Languages](https://img.shields.io/github/languages/top/Juice-Client/node-enject)

# node-enject
Electron injector that intercepts Chromium's render widget window to fix a Chromium bug.  
In Chromium versions past ~97 holding and dragging the cursor freezes any current websocket connection if running with the flag `--disable-frame-rate-limit`.  

Supports `x64`, `ia32 (x86)`, and `arm64` architectures for Windows only.

## Build locally
```bash
npm i -g @napi-rs/cli@latest # Install napi
./build.ps1 # Build artifacts in /dist
npm pack
```

## Usage
```bash
npm i @juice-client/node-enject
```

```js
import enject from "@juice-client/node-enject"

const win = new BrowserWindow({ // from electron
    show: false
})

win.once("ready-to-show", () => {
    const handleBuffer = win.getNativeWindowHandle()
    let hwnd

    if (process.arch === "x64" || process.arch === "arm64") {
        hwnd = Number(handleBuffer.readBigUInt64LE(0))
    } else {
        hwnd = handleBuffer.readUInt32LE(0)
    }

    enject.startHook(hwnd)
    win.show()
})
```

Built for [Juice Client](https://github.com/irrvlo/juice-client). [Community Server](https://discord.gg/FjzAAdSjng)