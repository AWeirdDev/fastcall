# ☎️ fastcall
Fastcall is an RPC server/client implementation written in Rust for Python. It's highly inspired by `ucall` and `fastapi` — I want the speed, typing, docstrings and that sweet developer experience (which google doesn't have one).

<table>

<thead>
<tr>
<th>🍰 Server</th>
<th>🥄 Client</th>
</tr>
</thead>

<tbody>

</tbody>
<td>

```python
from fastcall FastCall

app = FastCall()

@app
def add(a: int, b: int) -> int:
    """Adds two numbers."""
    return a + b
```

</td>
<td>

```python
# sum_client.py is generated using:
# $ fastcall gen server.py --python
from sum_client import Client

client = Client("http://localhost:8787")

# This function is typed!
print(client.add(6 * 9, 6 + 9))
```

</td>
</table>

## ⚡️ Features
Fastcall is fast. It's just that fast (so that no one can blame the loading is our fault).

🍛 **Seamless integration**. Give us the rice and we'll add the sauce. Dang, that's some delicious curry rice.

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
