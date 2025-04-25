PROJECT_DIR := spreadsheet
CARGO := cargo

.PHONY: all build run clean

all: build 

build:
	make clean
	$(CARGO) build --release

run:
	make clean
	$(CARGO) run --release

clean:
	$(CARGO) clean

test:
	$(CARGO) test

coverage:
	$(CARGO) tarpaulin

ext1:
	$(CARGO) run --release

docs:
	$(CARGO) doc
