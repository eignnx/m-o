# m-o
A command line tool for parsing Python [dataclass](https://docs.python.org/3/library/dataclasses.html) [reprs](https://docs.python.org/3/library/functions.html#repr) and pretty-printing them. The name is based on the [WALL-E character M-O](https://disney.fandom.com/wiki/M-O), a robot who liked to clean things up.

## Installation
```shell
$ cargo install m-o
```

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
$ python my_data.py | m-o --columns 20
Dog(
    name="Pip",
    age=7,
    friends=[
        "Quincy",
        "Digger",
    ],
)
```

## CLI Options

```
USAGE:
    m-o [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --columns <columns>    Specifies the width of the terminal or file that
                               the results will be printed to. If unspecified,
                               `m-o` will try to use the width of the current
                               terminal window. Defaults to 80 columns.
    -i, --indent <indent>      The number of spaces used for a single
                               indentation in the output. [default: 4]
```

## Motivation
Python's `pprint.pprint` function allows common values (tuples, lists, dicts, etc.) to be pretty-printed, but does not know how to format Python 3.7's [`dataclasses`](https://docs.python.org/3/library/dataclasses.html). I use dataclasses pretty frequently, and often need to debug deeply nested trees of dataclasses. The `__repr__` method on dataclasses displays everything on one line which is difficult to read.

Rather than implementing `pprint.pprint` for your dataclasses (who even knows how to do this anyway?), just print out the value and pipe it into this tool. The data structure will be pretty-printed to your terminal.

## (Planned/Current) Features
- [x] Parse string escape characters (0.1.5)
- [x] Use "Wadler-style" pretty-printing algorithm (0.1.6)
- [x] Use stable version of [`pretty.rs`](https://github.com/Marwes/pretty.rs) (1.0.0)
- [x] Add command-line options for:
  - [x] Indentation level (currently 4 spaces) (0.1.7)
  - [x] Target width (number of columns) (0.1.7)
- [ ] Better error messages when parsing fails
- [x] Allow non-keyword arguments in constructors (ex: `Dog('Pip', age=7)`) (0.1.7)
- [x] Support multi-identifier paths in symbols (ex: `datetime.datetime`) (0.1.7)
