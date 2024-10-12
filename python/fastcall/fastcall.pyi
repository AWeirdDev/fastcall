from typing import Any

def decode(data: str) -> Any: ...

class Message:
    jsonrpc: str
    id: str
    method: str
    params: Any
    def __init__(self) -> None: ...
    def __repr__(self) -> str: ...
