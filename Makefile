CC?=cc
CXX?=c++
CLANG?=clang

WORK_DIR?=work_dir

OPTIONS1?=-g3 -Wall -Wextra -Wformat=2 -Wcast-qual -Wcast-align -Wwrite-strings -Wfloat-equal -Wpointer-arith -Wundef
OPTIONS2?=-fomit-frame-pointer -DNDEBUG -fno-stack-protector -O3 -fpic
OPTIONS3?=-DMCL_USE_LLVM=1 -DMCL_BINT_ASM=1 -DMCL_BINT_ASM_X64=0 

MCL_DIR?=mcl
MCL_MAKEFILE = Makefile.cross
MCL_INCLUDE?=$(MCL_DIR)/include
MCL_LIB?=$(MCL_DIR)/lib
# BN256_LIBRARY?=$(MCL_LIB)/libmclbn384_256.a
MCL_LIBRARY?=$(MCL_LIB)/libmclbn384_256.a
BIT = 32
TARGET = riscv32

OBJ_DIR?=obj
BIN_DIR?=bin

CFLAGS?=$(OPTIONS1) -I $(MCL_INCLUDE) $(OPTIONS2) $(OPTIONS3)

.PHONY: mcl

# Always run the make command for mcl, as mcl is a phony target
mcl:
	$(MAKE) -C $(MCL_DIR) -f $(MCL_MAKEFILE) BIT=$(BIT) TARGET=$(TARGET)

zkverifier:
	@echo "\033[92mCompile each source file to an individual object file \033[0m"
	$(CLANG) -O3 -c groth16-verifier/main.c -o $(OBJ_DIR)/main.o -I $(MCL_INCLUDE) -I $(MCL_DIR)/src -I groth16-verifier -mcmodel=medany -march=rv32i -mabi=ilp32 --target=riscv32
	$(CLANG) -O3 -c groth16-verifier/sha256.c -o $(OBJ_DIR)/sha256.o -I $(MCL_INCLUDE) -I $(MCL_DIR)/src -I groth16-verifier -mcmodel=medany -march=rv32i -mabi=ilp32 --target=riscv32


	@echo "\033[92mLink the object files along with the start file and static library into the final executable \033[0m"
	riscv32-unknown-elf-gcc -O3 -static -mcmodel=medany -fvisibility=hidden -nostdlib -nostartfiles -T linkers/link.ld ./start.S $(OBJ_DIR)/main.o $(OBJ_DIR)/sha256.o mcl/lib/libmclbn384_256.a -o $(BIN_DIR)/zkverifier -Wl,--wrap=malloc,--wrap=free -march=rv32i -mabi=ilp32 -lgcc

clean:
	$(RM) $(OBJ_DIR)/main.o
	$(RM) $(OBJ_DIR)/sha256.o
	$(RM) $(BIN_DIR)/zkverifier
