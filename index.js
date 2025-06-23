const path = require("path")
const { platform, arch } = process

const map = {
    win32: {
        x64: "node-enject-x86_64-pc-windows-msvc.node",
        ia32: "node-enject-i686-pc-windows-msvc.node",
        arm64: "node-enject-aarch64-pc-windows-msvc.node"
    }
}

const loadAddon = () => {
    const binaries = map[platform]
    if (!binaries) throw new Error(`Unsupported platform: ${platform}`)

    const binaryName = binaries[arch]
    if (!binaryName) throw new Error(`Unsupported architecture: ${arch} on ${platform}`)

    const binaryPath = path.join(__dirname, "dist", binaryName)
    return require(binaryPath)
}

module.exports = loadAddon()