from fastcall import FastCall, decode

app = FastCall(dev=True)


@app
def add(a: int, b: int) -> str:
    """Adds two numbers."""
    return str(a + b)
