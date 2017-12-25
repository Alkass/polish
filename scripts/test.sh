for example in examples/*.rs; do
  echo $example
  cp $example src/main.rs
  cargo run
  rv=$?
  if [ $rv -ne 0 ]; then
    echo returning $rv
    return $rv
  fi
done
rm src/main.rs
