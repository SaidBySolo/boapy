# boapy

> A JavaScript interpreter that can be used in Python using the boa engine.

## Install

```sh
pip install boapy
```

## Example

```py
from boa import execute

execute("function add(a,b){ return a+b}")
result = execute("add(1,2)")
print(result)
```
