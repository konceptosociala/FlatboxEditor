ifeq ($(OS),Windows_NT)
	EXT = dll
else
	EXT = so
endif

FFI = target/release/libeditorffi.$(EXT)
DESTDIR = /usr/bin/

all : clean build

build:
	mkdir build -p
	cd SonjaEditor.FFI/lib/ && cargo build --release && cp $(FFI) ../../build/
	dotnet build -o build/

run: build
	./build/SonjaEditor

clean:
	rm -rf build/
	rm -rf obj/
	rm -rf bin/

purge: clean
	cd SonjaEditor.FFI/lib/ && cargo clean
