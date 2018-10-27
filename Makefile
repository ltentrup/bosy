.PHONY: default clean

default:
	mkdir -p external
	cd external ; cmake ..
	make -C external

clean:
	-rm -rf external
