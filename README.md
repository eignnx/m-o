# m-o
A command line tool for parsing Python [dataclass](https://docs.python.org/3/library/dataclasses.html) [reprs](https://docs.python.org/3/library/functions.html#repr) and pretty-printing them. The name is based on the [WALL-E character M-O](https://disney.fandom.com/wiki/M-O), a robot who liked to clean things up.

## Example

```python
# my_data.py

@dataclass
class Dog:
    name: str
    age: int
    friends: List[str]

pip = Dog("Pip", 7, ["Quincy", "Digger"])
print(pip)
```

```shell
$ # without m-o:
$ python my_data.py
Dog(name="Pip", age=7, friends=["Quincy", "Digger"])

$ # with m-o:
$ python my_data.py | m-o
Dog(
    name="Pip",
    age=7,
    friends=[
        "Quincy",
        "Digger",
    ],
)

```

## why tho?
Python's `pprint.pprint` function allows common values (tuples, lists, dicts, etc.) to be pretty-printed, but does not know how to format Python 3.7's [`dataclasses`](https://docs.python.org/3/library/dataclasses.html). I use dataclasses pretty frequently, and often need to debug deeply nested trees of dataclasses. The `__repr__` method on dataclasses displays everything on one line which is difficult to read.

Rather than implementing `pprint.pprint` for your dataclasses (who even knows how to do this anyway?), just print out the value and pipe it into this tool. The data structure will be pretty-printed to your terminal.

## Installation
```shell
$ cargo install m-o
```

## (Planned/Current) Features
- [x] Parse string escape characters (0.1.5)
- [ ] Use "Wadler-style" pretty-printing algorithm
- [ ] Better error messages when parsing fails
