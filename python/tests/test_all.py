import pytest
import fastcall


def test_sum_as_string():
    assert fastcall.sum_as_string(1, 1) == "2"
