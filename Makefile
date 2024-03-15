# compiler setup
CC?=cc
CXX?=c++
CLANG?=clang
RISCV_GCC?=riscv32-unknown-elf-gcc

# directory setup
OBJ_DIR?=obj
BIN_DIR?=bin
WORK_DIR?=work_dir
GROTH16_DIR?=groth16-verifier
MCL_DIR?=mcl
MCL_INCLUDE := $(MCL_DIR)/include
MCL_LIB_DIR := $(MCL_DIR)/lib

MCL_MAKEFILE = Makefile.cross
MCL_LIBRARY?=$(MCL_LIB_DIR)/libmclbn384_256.a

BIT=32
TARGET=riscv32

RV32I_FLAGS?=-mcmodel=medany -march=rv32i -mabi=ilp32
RISCV_FLAGS?=-O3 -T linkers/link.ld -static -fvisibility=hidden -nostdlib -nostartfiles -Wl,--wrap=malloc,--wrap=free -lgcc $(RV32I_FLAGS)

.PHONY: mcl

$(MCL_LIBRARY):
	$(MAKE) -C $(MCL_DIR) -f $(MCL_MAKEFILE) BIT=$(BIT) TARGET=$(TARGET)

$(OBJ_DIR)/%.o: $(GROTH16_DIR)/%.c
	@echo "\033[92mCompiling $< to $@\033[0m"
	$(CLANG) -O3 -c $< -o $@ -I $(MCL_INCLUDE) -I $(MCL_DIR)/src -I $(GROTH16_DIR) $(RV32I_FLAGS) --target=$(TARGET)

zkverifier: $(MCL_LIBRARY) $(OBJ_DIR)/main.o $(OBJ_DIR)/sha256.o
	@echo "\033[92mLink the object files along with the start file and static library into the final executable \033[0m"
	$(RISCV_GCC) start.S $(OBJ_DIR)/main.o $(OBJ_DIR)/sha256.o $(MCL_LIBRARY) -o $(BIN_DIR)/zkverifier $(RISCV_FLAGS)

clean:
	$(RM) $(OBJ_DIR)/main.o
	$(RM) $(OBJ_DIR)/sha256.o
	$(RM) $(BIN_DIR)/zkverifier
