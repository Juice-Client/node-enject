const https = require("https")
const fs = require("fs")
const path = require("path")

const version = require("../package.json").version

const binaryName = "node-enject.node"
const downloadUrl = `https://github.com/your-username/Juice-Client/node-enject/download/v${version}/${binaryName}`
const outPath = path.join(__dirname, "..", "bin", binaryName)

fs.mkdirSync(path.dirname(outPath), { recursive: true })

https
    .get(downloadUrl, (res) => {
        if (res.statusCode !== 200) {
            console.error(`Failed to download ${binaryName}: ${res.statusCode}`)
            process.exit(1)
        }

        const file = fs.createWriteStream(outPath)
        res.pipe(file)

        file.on("finish", () => {
            file.close()
            console.log(`Downloaded ${binaryName}`)
        })
    })
    .on("error", error => {
        console.error(`Error downloading ${binaryName}: ${error.message}`)
        process.exit(1)
    })