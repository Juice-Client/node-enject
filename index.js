const path = require("path")

const binaryName = "node-enject.node"
const binaryPath = path.join(__dirname, "bin", binaryName)

module.exports = require(binaryPath)