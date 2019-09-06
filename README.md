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
$ cargo install --git https://github.com/eignnx/m-o
```
### Why not `cargo install m-o`?
Currently, `m-o` depends on an unreleased alpha version of `pretty.rs`. This means `cargo install m-o` doesn't work because the crate cannot be published on `crates.io`.

If you don't mind using an older version of the crate, you can install version 0.1.5 (which has a less sophisticated layout algorithm) from `crates.io` with this command:
```shell
$ cargo install m-o
```


## (Planned/Current) Features
- [x] Parse string escape characters (0.1.5)
- [x] Use "Wadler-style" pretty-printing algorithm (0.1.6)
- [ ] Use stable version of [`pretty.rs`](https://github.com/Marwes/pretty.rs)
- [ ] Add command-line options for:
  - [ ] Indentation level (currently 4 spaces)
  - [ ] Target width (number of columns)
- [ ] Better error messages when parsing fails
