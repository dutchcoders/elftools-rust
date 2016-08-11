# Elftools for rust. Remco Verhoef <remco@honeytrap.io>
ifndef BUILDDIR
OBJDIR = ./build
endif

.PHONY: gen_const install clean test

#gen_const:
#	cd .. && python gen.py > 
#	cargo fmt

install:
	cargo build

clean:
	rm -rf target/

test:
	cargo test --verbose

