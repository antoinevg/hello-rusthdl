YOSYS := ~/GreatScott/toolchain/oss-cad-suite

ENVIRONMENT := source $(YOSYS)/environment

top:
	cargo run --bin blinky
	$(ENVIRONMENT) && python gateware/top.py

clean:
	rm -rf *.svg *.v *.vcd target/
