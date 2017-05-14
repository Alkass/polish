Get-ChildItem "examples\" -Filter *.rs | Foreach-Object {
  echo $_.FullName
  # TODO: run test cases
}
