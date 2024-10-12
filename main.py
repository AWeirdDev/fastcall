from fastcall import decode

r = decode("""{
    "jsonrpc": "2.0",
    "id": "awbqw-9q9q9",
    "method": "drink",
    "params": {"hot": "coffee", "cold": "milk"}
}""")
print(r)
