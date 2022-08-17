build:
	cargo build
	mv target/debug/stick .
	python3 -m pip install --upgrade stickpy/
	rm -rf build dist
