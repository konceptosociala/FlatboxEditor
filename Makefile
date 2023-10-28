ifeq ($(OS),Windows_NT)
	EXT = dll
else
	EXT = so
endif

FFI = target/release/libnative.$(EXT)
DESTDIR = /usr/bin/

all : clean build

build:
	mkdir build -p
	cd FlatboxEditor.FFI/ && cargo build --release && cp $(FFI) ../build/
	dotnet build -o build/

run: build
	./build/FlatboxEditor

clean:
	rm -rf build/
	rm -rf obj/
	rm -rf bin/

purge: clean
	cd FlatboxEditor.FFI/ && cargo clean
