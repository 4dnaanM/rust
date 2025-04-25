PROJECT_DIR := spreadsheet
CARGO := cargo

.PHONY: all build run clean

all: build 

build:
	make clean
	$(CARGO) build --release

run:
	make build
	@read -p "Enter number of rows: " rows; \
	 read -p "Enter number of columns: " cols; \
	 cargo run --release -- $$rows $$cols

clean:
	$(CARGO) clean

test:
	rm -rf vcs*
	$(CARGO) test

coverage:
	rm -rf vcs*
	$(CARGO) tarpaulin

docs:
	$(CARGO) doc

ext1:
	@echo "Choose mode:"
	@echo "1. Load from vcs_dir"
	@echo "2. Create new with rows and cols"
	@read -p "Enter choice (1 or 2): " choice; \
	if [ "$$choice" = "1" ]; then \
		read -p "Enter vcs_dir path: " vcs_dir; \
		cargo run -- --vcs --vcs_dir $$vcs_dir; \
	elif [ "$$choice" = "2" ]; then \
		read -p "Enter number of rows: " rows; \
		read -p "Enter number of cols: " cols; \
		cargo run -- --vcs --rows $$rows --cols $$cols; \
	else \
		echo "Invalid choice"; \
		exit 1; \
	fi
