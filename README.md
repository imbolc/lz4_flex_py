lz4_flex python wrapper
=======================

Usage
-----
```python
>>> import lz4_flex
>>> compressed = lz4_flex.compress_prepend_size(b"foo")
>>> lz4_flex.decompress_size_prepended(compressed)
b'foo'
```

Development
-----------

Preparations:
```sh
pip install maturin
python -m venv .venv
source .venv/bin/activate
```
Install the package:
```sh
maturin develop
```

Test it:
```sh
python -c'import lz4_flex'
```

Publishing to pypi:
```sh
maturin publish
```
