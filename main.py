from fastcall import FastCall

fc = FastCall()


@fc
def bake():
    """Literally bakes."""

    print("baking")
    print("baked.")


bake.message()
