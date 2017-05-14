if ($env:PLATFORM -eq "x86") {
    $arch = "i686"
}
else {
    $arch = "x86_64"
}

$rust_version = $env:RUST_VERSION
$rust_install = "rust-$rust_version-$arch-pc-windows-gnu.msi"
Start-FileDownload "https://static.rust-lang.org/dist/$rust_install" -FileName $rust_install
Start-Process -FilePath msiexec -ArgumentList /i, $rust_install, /quiet, INSTALLDIR="C:\Rust" -Wait
$env:Path = $env:Path + ";C:\Rust\bin"
rustc -vV

Get-ChildItem "examples" -Filter *.log | Foreach-Object {
  cp $_.FullName src/test.rs
  cargo run
}
