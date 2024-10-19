import inspect
import json
from types import NoneType
from typing import Callable, Generic, List, ParamSpec, TypeVar, Union

from .fastcall import Message

AnyType = Union[str, int, float, bool, list, dict, NoneType]

P = ParamSpec("P")
AnyT = TypeVar("AnyT", bound=AnyType)


class FastCall:
    """☎️ JSON-RPC implemented in Rust & Python.

    Example:

    .. code-block:: python

        from fastcall import FastCall

        app = FastCall()

        @app
        def add(a: int, b: int) -> int:
            \"\"\"Adds two numbers.\"\"\"
            return a + b

        app.run()

    Attributes:
        fns (List[Callable[..., AnyType]]): Function calls.
        dev (bool): Whether to run in development mode.
    """

    __slots__ = ("fns", "dev")
    fns: List["Function"]
    dev: bool

    def __init__(self, *, dev: bool = True):
        self.fns = []
        self.dev = dev

    def __call__(self, fn: Callable[P, AnyT]) -> "Function[P, AnyT]":
        params = inspect.signature(fn).parameters

        for name, param in params.items():
            if param.annotation == inspect.Parameter.empty:
                e = TypeError(f"Parameter '{name}' must be annotated with a type")
                e.add_note(
                    f"Hint: Add an explicit type annotation like `def {fn.__name__}(…, {name}: str)`"
                )
                raise e

            if param.annotation not in (str, int, float, bool, list, dict):
                raise TypeError(
                    f"Parameter '{name}' must be one of type str, int, float, bool, list or dict"
                )

        fnc = Function(fn)
        self.fns.append(fnc)

        return fnc


class Function(Generic[P, AnyT]):
    fn: Callable[P, AnyT]

    def __init__(self, fn: Callable[P, AnyT]):
        self.fn = fn

    def message(self, id: str, *args: P.args, **kwargs: P.kwargs) -> Message:
        if args:
            msg = Message(id, kw=False)
            for itm in args:
                msg.set_param("args", json.dumps(itm))

        elif kwargs:
            msg = Message(id, kw=True)
            for k, v in kwargs.items():
                msg.set_param(k, json.dumps(v))

        else:
            msg = Message(id)

        return msg

    def __call__(self, *args: P.args, **kwargs: P.kwargs) -> AnyT:
        return self.fn(*args, **kwargs)
