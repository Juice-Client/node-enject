const path = require("path")

const binaryName = "node-enject.node"
const binaryPath = path.join(__dirname, binaryName)

module.exports = require(binaryPath)