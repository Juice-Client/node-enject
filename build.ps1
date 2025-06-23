$targets = @(
  "x86_64-pc-windows-msvc",
  "i686-pc-windows-msvc",
  "aarch64-pc-windows-msvc"
)

if (Test-Path -Path "dist") {
    Remove-Item -Path "dist\*" -Recurse -Force
} else {
    New-Item -ItemType Directory -Path "dist" | Out-Null
}

foreach ($target in $targets) {
    $env:CARGO_BUILD_TARGET = $target
    npx napi build --release --target $target

    $sourcePath = "index.node"
    $destPath = "dist/node-enject-$target.node"

    Move-Item -Path $sourcePath -Destination $destPath
}