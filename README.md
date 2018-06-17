# Instructions

```
rustup override set nightly
python3 setup.py install
python3 -c "import pyo3typeerror; pyo3typeerror.loads(b'"\xc3\xa5"')"
Traceback (most recent call last):
  File "<string>", line 1, in <module>
TypeError
```