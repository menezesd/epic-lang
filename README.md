# epic-lang
Interpreter for Python-like programming language with braces

## Installing ANTLR

It is recommended to use Linux to solve this problem. If you don't have Linux, you may either use WSL (that is, Windows Subsystem for Linux), or experiment on Windows somehow (I just didn't try to run it on Windows and don't know whether it's easy).

To install ANTLR, use the following command:

```
pip install antlr4-tools antlr4-python3-runtime
```

The commands above may download a Java runtime if you haven't installed it before.

## Building

To build the grammar, use the following command:

```
antlr4 EpicLang.g4 -Dlanguage=Python3 -visitor
```

When you rebuilt the grammar after its modification, you can just run the interpreter, which is located in `interp.py`

## Syntax

### Types

The language contains the values of the following types, which fully correspond to some of the types in Python:
- integers (`1`, `2`, `-100`, ...)
- booleans (`true`, `false`)
- `none` (corresponding to `None` in Python)
- lists (`[1, 2]`, `[]`, `[[1, 2], true, 5]`, ...)

The language is dynamically typed, so the variable types are checked while running the program. In principle, you may just use plain Python values to store the contents of the variables.

Note that, just like in Python, lists are reference types, so if you copy the list, only the reference is copied:

```
func main() {
    a = [1,2,3];
    print a; // prints [1,2,3]
    b = a;
    a[1] = 4;
    print a; // prints [1,4,3]
    print b; // also prints [1,4,3], as `a` and `b` refer to the same list
}
```

### Identifiers

Identifiers are the valid names for functions and variables. An identifier must consist only of Latin letters (lowercase or uppercase), digits and underscores (`_` character). An identifier cannot start with a digit.

Some tokens are reserved and cannot be used as identifers. These are the keywords: `break`, `continue`, `do`, `else`, `false`, `for`, `func`,  `if`, `len`, `none`, `print`, `return`, `then`, `true`, `while`.

### Comments and spaces

Comments start with `//` and continue to the end of the line. So, they just look like single-line comments from C.

Spaces can be safely ignored, except for the places where they can help to separate the tokens.

### Program structure

The program consists of one or more functions. Each of the functions can take zero or more arguments. The entry point of the program is `main()` function, which takes no arguments. Any two declared functions cannot have the same name. The name of the function must be an identifier (see above).

Example function declarations:

```
func first_func() {
    // function body
}

func second_func(x, y, z)
{
    // function takes three arguments: x, y, z.
    // function body
}
```

Function body consists of zero or more _statements_, see [below](#statements).

### Variables

During the execution of a function, it may hold some local variables. These variables are tied to the function (more precisely, to the frame of the stack on which the function is executed). Unlike C++ or similar languages, the variable goes out of scope only when the function ends, not when the block ends. Example:

```
x = 1;
{
    x = 4;
    u = 8;
    print x; // prints 4
}
print x; // still prints 4
print u; // prints 8
```

### Statements

There are the following types of statements:

* Empty statement. Contains just one `;` and does nothing)

* Expression statement. Contains an _expression_ (see [below][#expressions]) followed by `;`. For example,

  ```
  42; // simple expression statement, does nothing
  my_func(); // calls `my_func`
  my_func(1, 2); // calls `my_func` with arguments `1` and `2`
  ```

- Block. Contains multiple statements enclosed in curly brackets. For example,

  ```
  {
      call1();
      call2();
      call3();
  }
  ```

- If. For example,

  ```
  if condition then doThis(); else doThat();
  if condition2 then {
      do1();
      do2();
  } else {
      do3();
  }
  ```

  The bodies of `then` and `else` branches are statements (expression statements in the first case and blocks in the second case).

- For. Iterates over a range in ascending order (without including the upper bound), putting the current value into a given variable. After the iteration is done, the variable is given its old value before `for`. For example,

  ```
  i = 42;
  for i in 0..4 do {
      print i; // prints 0, 1, 2 and 3 sequentially
  }

  print 42; // prints 42, as the old value is assigned back to `i` after iteration.

  for i in 2-2..2+2 do {
      print i; // prints 0, 1, 2 and 3 sequentially,
              // because the range is from 0 = 2-2 and 4 = 2+2.
  }
  ```

  Again, the body of `for` is a statement.

- While. Checks for condition, if it's still true, then executes the statements in its body. The process is repeated until the condition becomes false during one of the checks. Example:

  ```
  i = 1;
  while i < 50 do {
      i = i * 2;
      print i; // print 2, 4, 8, 16, 32 sequentially.
  }
  ```

  Again, the body of `while` is a statement.

- Assignment. There can be either assignment to a vairable or to a list item. Example:

  ```
  a = 3; // assigns 3 to a
  b = [1,2,[3,4]]; // assigns [1,2,[3,4]] to b
  b[0] = 3; // now, b is equal to [3,2,[3,4]]
  b[2][0] = 5; // now, b is equal to [3,2,[5,4]]
  ```

- Loop control statements (`break;` and `continue;`)

- Return statement. Can be either just `return;` or `return <expr>;` where `<expr>` is some expression. If the expression is omitted, then `return` returns `none`. If the execution goes to the end of the function without encountering any `return` statements, then it's assumed that the function just returns `none`.

- Print statement. Prints the expression value.  See above for examples. You can just use simple `print()` from Python to implement this statement.

## Expressions

Expressions are using within various statements. An expression can contain the following:

- Literals (`none`, `true`, `false`)
- Integers
- List literals (like `[1, 2, 3, 4]`, `[2+2, 3, 4, [1, 2, 3]]`)
- Variables. In this case the variable value is substituted. The variable must be declared.
- Function calls (like `fn1(1, 2, 3)`, `a(b(), c(1, 2))`)
- List length operator (like `len([1, 2, 3])`, `len(x)`)
- Operators. The operators are similar to Python (except that `!`, `&` and `|` are used instead of `not`, `and` and `or`, respectively). Operators have the following priority, from highest to lowest:
  - Indexing operator (like in `a[1][2]`)
  - Unary operators (`+`, `-`, `!`)
  - Integer multiplication, division and modulo (`*`, `/`, `%`)
  - Binary plus and minus (`+`, `-`). Note that `+` means both integer addition and list concatenation.
  - Comparison operators (`<`, `<=`, `>`, `>=`, `==`, `!=`).
  - Logical AND and OR (`&`, `|`). Unlike C, both `&` and `|` have the same priority.

## Testing

To run the tests, use the following command:

```
python3 test.py
```
