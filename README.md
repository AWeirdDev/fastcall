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
# sum_client module is generated with cli!

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
@dataclass
class Pizza:
  name: str
  ingredients: list[str]

@app
def make_pizza(*ingredients: str) -> Pizza:
    return Pizza(name="Custom", ingredients=ingredients)
```

<br />

**🍩 Sugary typing for everything**. Say no more to raw JSON handling for the client, they're just ugly. Create bindings across different languages with just one server file.

```sh
$ fastcall gen server.py:app --bindings python
```

Don't worry. Type hints are everywhere, including docstrings.

