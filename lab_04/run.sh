set -e

cargo build --release
./target/release/rsa --generate_keys -o id_rsa
./target/release/rsa -i Cargo.toml -o cipher.txt -k id_rsa.pub
./target/release/rsa -i cipher.txt -o new.toml -k id_rsa --decode
echo "BEFORE"
cat Cargo.toml
echo -e "\n\nAFTER"
cat new.toml
rm -f cipher.txt id_rsa id_rsa.pub new.toml
