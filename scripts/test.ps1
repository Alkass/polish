Get-ChildItem "examples\" -Filter *.rs | Foreach-Object {
  echo $_.FullName
  cp $_.FullName src/test.rs
  rustc $_.FullName
}
