wget https://github.com/SimonKagstrom/kcov/archive/master.tar.gz
tar xzf master.tar.gz && mkdir kcov-master/build && cd kcov-master/build
cmake ..
make
sudo make install
cd ../..
kcov --coveralls-id=$1 --exclude-pattern=/.cargo target/kcov target/debug/polish*
