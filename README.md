# ☎️ fastcall
Fastcall is an RPC server/client implementation written in Rust for Python. It's highly inspired by `ucall` and `fastapi` — I want the speed, typing, docstrings and that sweet developer experience (which google doesn't have one).

```python
from fastcall FastCall

app = FastCall()

@app
def add(a: int, b: int) -> int:
    """Adds two numbers."""
    return a + b

app.run()
```

## ⚡️ Features
Fastcall is fast. It's just that fast (so that no one can blame the loading is our fault).

1. 🍛 **Seamless integration**. Give us the rice and we'll add the sauce. Dang, that's some delicious curry rice.

Fastcall supports:

- Python built-ins *(default)*
- Dataclasses *(default)*
- Custom types *(default)*
- Pydantic models *(`fastcall-pydantic`)*
- Full page control *(`fastcall-web`)*

...and more to come!

```python
@app
def make_pizza(*ingredients: str) -> Pizza:
    order = make_pizza(ingredients)
    return Receipt(id=order.id, cost=order.cost)
```
