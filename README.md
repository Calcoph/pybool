# PyBool

Write `or`, `and` and `not` instead of `||`, `&&` and `!` in rust!

This is the same as in C:
```C
#define and &&
#define or ||
#define not !
```

# Usage
```rust
use pybool::pybool;
pybool!(true or true) == true
pybool!(true or false) == true
pybool!(false or true) == true
pybool!(false or false) == false

pybool!(true and true) == true
pybool!(true and false) == false
pybool!(false and true) == false
pybool!(false and false) == false

pybool!(not true) == false
pybool!(not false) == true

// You probably want to use a shorter name
use pybool::pybool as b;
b!((false or !(false or true)) and !false) == false
```