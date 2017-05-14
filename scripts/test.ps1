Get-ChildItem "examples" -Filter *.log | Foreach-Object {
  cp $_.FullName src/test.rs
  cargo run
}
