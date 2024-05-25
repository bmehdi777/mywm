BIN_NAME=mywm

.PHONY: run clean release

run:
	cargo build
	Xephyr -screen 800x600 :1
	DISPLAY=:1 ./target/debug/${BIN_NAME}

release: 
	cargo build --release
	Xephyr -screen 800x600 :1
	DISPLAY=:1 ./target/release/${BIN_NAME}

clean:
	rm ./target
	echo "Folder ./target/ removed."
