from json import load
from sys import argv

proof_path = argv[1]

print("generating groth16-verifier/proof.h")

def str_quote(s):
    return f"\"{s}\""

def str_g1(g1, name):
    return f"const char* {name} = \"1 {g1[0]} {g1[1]}\";"

def str_g2(g2, name):
    return f"const char* {name} = \"1 {g2[0][0]} {g2[0][1]} {g2[1][0]} {g2[1][1]}\";"

def mcl_bytes(decimal_str_x, decimal_str_y):
    lst = list(int(decimal_str_x).to_bytes(32, "little"))
    if int(decimal_str_y) % 2 == 1:
        lst[-1] += 0x80
    return lst

def mcl_bytes_g1(g1):
    byts = list(int(g1[0]).to_bytes(32, "little"))
    if int(g1[1]) % 2 == 1:
        byts[-1] += 0x80
    return byts

def mcl_bytes_g2(g2):
    bytes_1 = list(int(g2[0][0]).to_bytes(32, "little"))
    bytes_2 = list(int(g2[0][1]).to_bytes(32, "little"))
    if int(g2[1][0]) % 2 == 1:
        bytes_2[-1] += 0x80
    return bytes_1 + bytes_2

def str_bytes(byts, name):
    return f"const unsigned char bytes_{name}[{len(byts)}] = {{{', '.join(map(str, byts))}}};"

lst = []

with open(proof_path, "r+") as file:
    proof = load(file)
    lst.append(str_bytes(mcl_bytes_g1(proof["pi_a"]), "proof_a"))
    lst.append(str_bytes(mcl_bytes_g2(proof["pi_b"]), "proof_b"))
    lst.append(str_bytes(mcl_bytes_g1(proof["pi_c"]), "proof_c"))

with open("groth16-verifier/proof.h", "w+") as file:
    file.write("\n".join(lst))
