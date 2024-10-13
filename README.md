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
from fastcall import FastCall

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

***

## 🧃 Examples
Get started with a few examples.

<details>
<summary><b>🌦️ Weather forecast</b></summary>

```python
from dataclasses import dataclass
from fastcall import FastCall

app = FastCall()

@dataclass
class Forecast:
  description: str
  precipitation: str
  high_temp: float
  low_temp: float

@app
def get_forecast(place: str) -> Forecast:
    """Gets the weather forecast for a place."""
    data = fake_api.get_weather(place)
    return Forecast(data['desc'], data['prec'], data['high'], data['low'])
```

</details>

## 📚 Reference
All the essentials. Technical:tm:.

### RPCs and thoughts
Fastcall supports:

- JSON-RPC
- FC-RPC *(custom)*

Currently, there are great RPCs out there like gRPC (uses Protobuf), JSON-RPC (widely used and known) and more. I was wondering if we could make a custom one that's both **fast** and **light**? The answer is definitely a yes, and it involves Rust's `bincode`.

Using a custom RPC schema, we could parse number-based data faster and more accurately. You don't want a long JSON array of ints and floats, that's just disgusting. 🤮

To set the RPC implementation, simply add the `impl` parameter for the server constructor:

```python
# JSON-RPC (default)
app = FastCall(impl="json")

# FC-RPC (custom)
app = FastCall(impl="fc")
```

