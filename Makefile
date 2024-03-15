# compiler setup
CC?=cc
CXX?=c++
CLANG?=~/_install/bin/clang
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

BIT = 32
TARGET = riscv32

OPTIONS1?=-g3 -Wall -Wextra -Wformat=2 -Wcast-qual -Wcast-align -Wwrite-strings -Wfloat-equal -Wpointer-arith -Wundef
OPTIONS2?=-fomit-frame-pointer -DNDEBUG -fno-stack-protector -O3 -fpic
OPTIONS3?=-DMCL_USE_LLVM=1 -DMCL_BINT_ASM=1 -DMCL_BINT_ASM_X64=0 

CFLAGS?=$(OPTIONS1) -I $(MCL_INCLUDE) $(OPTIONS2) $(OPTIONS3)
RV32I_FLAGS?=-mcmodel=medany -march=rv32i -mabi=ilp32
RISCV_FLAGS?=-O3 -static -fvisibility=hidden -nostdlib -nostartfiles -T linkers/link.ld start.S -Wl,--wrap=malloc,--wrap=free -lgcc $(RV32I_FLAGS)

.PHONY: mcl

# Always run the make command for mcl, as mcl is a phony target
mcl:
	$(MAKE) -C $(MCL_DIR) -f $(MCL_MAKEFILE) BIT=$(BIT) TARGET=$(TARGET)

zkverifier:
	@echo "\033[92mCompile each source file to an individual object file \033[0m"
	$(CLANG) -O3 -c $(GROTH16_DIR)/main.c -o $(OBJ_DIR)/main.o -I $(MCL_INCLUDE) -I $(MCL_DIR)/src -I $(GROTH16_DIR) -mcmodel=medany -march=rv32i -mabi=ilp32 --target=riscv32
	$(CLANG) -O3 -c $(GROTH16_DIR)/sha256.c -o $(OBJ_DIR)/sha256.o -I $(MCL_INCLUDE) -I $(MCL_DIR)/src -I $(GROTH16_DIR) -mcmodel=medany -march=rv32i -mabi=ilp32 --target=riscv32

	@echo "\033[92mLink the object files along with the start file and static library into the final executable \033[0m"
	$(RISCV_GCC) -O3 -static -mcmodel=medany -fvisibility=hidden -nostdlib -nostartfiles -T linkers/link.ld ./start.S $(OBJ_DIR)/main.o $(OBJ_DIR)/sha256.o $(MCL_LIBRARY) -o $(BIN_DIR)/zkverifier -Wl,--wrap=malloc,--wrap=free -march=rv32i -mabi=ilp32 -lgcc

clean:
	$(RM) $(OBJ_DIR)/main.o
	$(RM) $(OBJ_DIR)/sha256.o
	$(RM) $(BIN_DIR)/zkverifier
