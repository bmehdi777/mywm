BIN_NAME=mywm

.PHONY: run clean release stop

run:
	unset XDG_SEAT
	Xephyr -br -ac -noreset -screen 800x600 :1 &
	cargo build
	clear
	DISPLAY=:1 ./target/debug/${BIN_NAME}

release: 
	unset XDG_SEAT
	Xephyr -br -ac -noreset -screen 800x600 :1 &
	cargo build --release
	clear
	DISPLAY=:1 ./target/release/${BIN_NAME}

stop:
	pkill Xephyr
	while pgrep -x Xephyr >/dev/null; do sleep 1; done

clean:
	rm -r ./target
