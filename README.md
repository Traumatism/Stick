# Stick

An OSINT tool similar as [Squiz](https://github.com/traumatism/squiz) but in... Rust !

Yes... the cute crab seduced me... but i'm not quitting Python at all (at least now ^^): indeed, Stick use Python 3 as a scripting language to write modules !

## Usage

1. build it from source and move the binary to our current location

`cargo build && mv target/debug/stick $(pwd)`

2. if it's not present, create the modules folder

`mkdir modules/`

2. (b) create a hello world module

`python3 create_module hello`

3. run stick on the target of your choice

`./stick 1.1.1.1`

`./stick domain.tld`

`./stick user@domain.tld`

`./stick 1.1.1.1:80`

`./stick https://1.1.1.1:80`
