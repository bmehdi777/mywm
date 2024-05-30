BIN_NAME=mywm

.PHONY: run clean release stop

run:
	cargo build
	unset XDG_SEAT
	Xephyr -br -ac -sw-cursor -screen 800x600 :1 2> /dev/null &
	clear
	sudo DISPLAY=:1 ./target/debug/${BIN_NAME}

release: 
	cargo build --release
	unset XDG_SEAT
	Xephyr -br -ac -noreset -screen 800x600 :1 &
	clear
	sudo DISPLAY=:1 ./target/release/${BIN_NAME}

stop:
	pkill Xephyr
	while pgrep -x Xephyr >/dev/null; do sleep 1; done

clean:
	rm -r ./target
