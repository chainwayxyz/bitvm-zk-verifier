CC?=cc
CXX?=c++

WORK_DIR?=work_dir

OPTIONS1?=-g3 -Wall -Wextra -Wformat=2 -Wcast-qual -Wcast-align -Wwrite-strings -Wfloat-equal -Wpointer-arith -Wundef
OPTIONS2?=-fomit-frame-pointer -DNDEBUG -fno-stack-protector -O3 -fpic
OPTIONS3?=-DMCL_USE_LLVM=1 -DMCL_BINT_ASM=1 -DMCL_BINT_ASM_X64=0 

MCL?=mcl
MCL_INCLUDE?=$(MCL)/include
MCL_LIB?=$(MCL)/lib
BN256_LIBRARY?=$(MCL_LIB)/libmclbn256.a
MCL_LIBRARY?=$(MCL_LIB)/libmcl.a

CFLAGS?=$(OPTIONS1) -I $(MCL_INCLUDE) $(OPTIONS2) $(OPTIONS3)

groth16:
	$(CC) $(CFLAGS) -c groth16-verifier/main.c -o groth16-verifier/main.o -MMD -MP -MF groth16-verifier/main.d
	rm groth16-verifier/main.d
	gcc -c groth16-verifier/sha256.c -I groth16-verifier/ -o groth16-verifier/sha256.o
	$(CXX) groth16-verifier/sha256.o groth16-verifier/main.o -o bin/groth16_verifier $(BN256_LIBRARY) $(MCL_LIBRARY)
	rm groth16-verifier/main.o
	rm groth16-verifier/sha256.o

transpiler:
	clang -Ofast -c src/zkverifier.c -o execs/zkverifier.o  -I ./mcl/include -I ./mcl/src -mcmodel=medany -march=rv32i -mabi=ilp32 --target=riscv32
	riscv32-unknown-elf-gcc -Ofast -static -mcmodel=medany -fvisibility=hidden -nostdlib -nostartfiles -T linkers/link.ld ./start.S execs/zkverifier.o mcl/lib/libmclbn384_256.a -o execs/zkverifier -Wl,--wrap=malloc,--wrap=free  -march=rv32i -mabi=ilp32 -lgcc
	# riscv32-unknown-elf-objcopy -O binary execs/zkverifier execs/out_binary.bin

	# echo "Running emu-rv32i"
	# ./emu-rv32i execs/zkverifier
	# riscv64-unknown-elf-objdump -Mno-aliases -d execs/zkverifier  > dumps/zkverifier.dump
	# npx ts-node --files main.ts ./execs/zkverifier 

	# ./emu-rv32i execs/zkverifier > dumps/emulator_execution.dump
	echo "Running bitvm"
	cd rv32i-to-bitvm
	npx ts-node --files main.ts ../execs/zkverifier > ../dumps/bitvm_execution.dump
	cd ..

clean:
	rm bin/*
