all: build

build: bin/program bin/patcher

clean:
	rm -rf bin

run:
	cd bin && ./patcher -i program -o a.out && ./a.out

bin/program: program/src/main.rs
	mkdir -p bin
	cd program && cargo build && mv target/debug/program ../bin

bin/patcher: patcher/src/main.rs
	mkdir -p bin
	cd patcher && cargo build && mv target/debug/patcher ../bin
