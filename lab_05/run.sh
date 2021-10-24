set -e

cargo build --release
./target/release/sign --generate_keys -k id_rsa
./target/release/sign -i Cargo.toml -k id_rsa -s my.sign
./target/release/sign -i Cargo.toml -k id_rsa.pub -s my.sign --verify
rm -f id_rsa id_rsa.pub my.sign
