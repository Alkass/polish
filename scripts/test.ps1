for example in examples/*
do
  echo $example
  cp $example src/main.rs
  cargo run
  cd ..
done
rm src/main.rs
