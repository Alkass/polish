for example in examples/*; do
  echo $example
  cp $example src/main.rs
  cargo run
done
rm src/main.rs
