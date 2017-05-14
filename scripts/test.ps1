echo running tests
Get-ChildItem "examples" -Filter *.log | Foreach-Object {
  echo $_.FullName
  cp $_.FullName src/test.rs
  cargo run
}
