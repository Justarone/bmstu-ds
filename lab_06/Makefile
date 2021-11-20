run: decompressed.toml

build: ./target/release/huffman
	cargo build --release

compress: compressed.b

compressed.b: build
	echo "\n\n\n"
	./target/release/huffman -i ./target/release/huffman -o compressed.b

decompressed.toml: compressed.b
	echo "\n\n\n"
	./target/release/huffman -i compressed.b -o decompressed --decompress

clean:
	rm -f decompressed compressed.b
