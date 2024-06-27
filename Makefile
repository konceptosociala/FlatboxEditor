ifeq ($(OS),Windows_NT)
	EXT = dll
else
	EXT = so
endif

FFI = target/debug/libnative.$(EXT)
DESTDIR = /usr/bin/

all: clean buildRelease

build:
	mkdir build -p
	cd FlatboxEditor.FFI/ && cargo build && cp $(FFI) ../build/
	dotnet build -o build/

buildRelease:
	mkdir build -p
	cd FlatboxEditor.FFI/ && cargo build && cp $(FFI) ../build/
	dotnet build -o build/ --configuration Release

run: build
	./build/FlatboxEditor

clean:
	rm -rf build/
	rm -rf obj/
	rm -rf bin/

purge: clean
	cd FlatboxEditor.FFI/ && cargo clean

cls:
	clear

debug: cls clean build run

doc:
	cd FlatboxEditor.FFI/ && cargo doc --open --no-deps