# Variables for directories
PROG_DIR = program
SCRIPT_DIR = script

# Build targets
all: build-elf build-script build-game

.PHONY: all build-elf build-script build-game

build-elf:
	@echo "Building ELF files"
	cd $(PROG_DIR) && cargo prove build && aligned get-vk-commitment --verification_key_file  elf/riscv32im-succinct-zkvm-elf  --proving_system SP1 2> elf/commitment

build-script:
	@echo "Building SCRIPT files"
	cd $(SCRIPT_DIR) && cargo build --release

demo:
	@echo "Running demo"
	cd $(SCRIPT_DIR) cargo r --release -- --board '{"w":["Ke1","Qf3","Bc4","Pd2","Pe4"],"b":["Kf8","Pf7","Pg7","Ph7","Pe7"]}' --mode interactive --moves '[]' --keystore-path ~/.foundry/keystores/fio

clean:
	@echo "Cleaning all builds"
	cd $(PROG_DIR) && cargo clean
	cd $(SCRIPT_DIR) && cargo clean
