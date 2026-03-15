## [BEHAVIOR]
- Ends program by default when type information is incorrect unless `#warn` is used.

- Binary representation. 
Why?
WHY NOT

Why?

## [Types]
i8, u8, i16, u16, i32, u32, i64, u64
i128, u128, f16, f32, f64, f128, sized, unsized,
char, bool, (maybe capital) str, struct, enum, nil (maybe not), BigInt, BigFloat, List, Map, Set

`struct` for a structure of data.
`enum` for an Enum type which can also hold data.

## [Operators]
`!`: Not operator.

### MIGHT EXIST
`||`: Or operator.

```chrn

alias LongDefault(x, y) = !IsEmpty, Range(x, y), StartsW("ch") EndsW("ern") Contains("chern")

alias ShortDefault() = IsWhitespace

var->
    special_string: str [LongDefault(0, 5)]

    some_str: str [ShortDefault()]
```


# DOES NOT EXIST YET
`_`: Match all for ignoring parameters

```chrn
alias gopher(x, y) = !IsEmpty, Range(x?: 0.0, y?: 5.2), StartsW("ch") EndsW("ern") Contains("chern")

var->
    special_stir: str [gopher(0.5, _)] // defaults to (0.5, 5.0) 

    stirring: str [gopher(2.0, 5.0)] // Works as normal
```

`?`: Infers type and expects type consistency throughout entire `.chrn` file. Can also be used for optional parameters within an alias function, which allows for a default to be specified.

# DOES NOT EXIST YET
`~`: Name bypass operator. ~str

# DOES NOT EXIST YET
`(range)`: Explicit range syntax. The '=' is required. `0..=5`

## [Predicates]
`IsEmpty`: Checks if the given array or string has a length of 0.

`IsWhitespace`: Checks if a string is only whitespace within UTF-8 standards, or is empty.

## Functions (Predicate)
(WHAT TO DO WITH THIS?)

`Range(inclusive, inclusive)`: Checks if the data being viewed matches the range given. For arrays and strings, this checks the length. For numbers, this checks the numeric value.

// WHAT IF ALL OF THESE WORKED ON NUMBERS?
Why? WHAT IF WE HAVE BINARY ONLY?

`Contains(Literal)`: Checks if the data being viewed contains the given literal.

`StartsW(Literal)`: Checks if the data being viewed starts with the given literal.

`EndsW(Literal)`:

// Does not exist yet
`Regex("0-9a-zA-Z*")`

## Statements

`alias`: Allows for predicates to be stored within a single keyword in the case of long conditions.

```chrn
alias LongDefault(x, y) = !IsEmpty, Range(x, y), StartsW("ch") EndsW("ern") Contains("chern")

alias ShortDefault() = IsWhitespace

var->
    special_string: str [LongDefault(0, 5)]

    some_str: str [ShortDefault()]
```

`bind`: Defines where a serialized file is located that should be checked, or deserialized.

## [Sections]

// This sounds convoluted..
- Sections are how data can be parsed in different ways. They exist as opposed a keywords so that data is always defined in a readable, expected manner.

- The `->` operator is used after section keywords to swap to the section.

`var`: Front facing definitions of the data to be serialized or deserialized.

```chrn
// If we have struct Person, it would look like
var->
    name: str
    age: u8

// But given nested data such as
    account: Account

// it would need a nest section
```
```
```

`nest->`: Allows for the definition of a struct or enum

```chrn
var->
    account: Account
    state: State
nest->
    struct Account {
        balance: BigFloat
    }

    enum State {
        Ready(str) // Enums can only store ONE type (as of right now) Would likely need tuple expr
        InProgress
        Failed
    }

```

# DOES NOT EXIST YET
`override->`: What to default to when a language doesn't contain a particular type. Language defaults exist but this can change any if needed.

(Probably not a good idea)
There is also a "like" category. A "JAVA_LIKE" category would have all of the int, short, logic for a batch of languages.

`complex`: Define complex rules such as enum bounds.

    complex:
        State.variants = 5

## Arguments
`#warn`: Would warn instead of terminating.

`#ign_if`: (Would remove anything that didn't align under condition rather than crash or warn.)

`#scientific`, `#hex`, `#bin`, `#octo`: Numeric notations to output in serialized file.

#### Full example of language

// YOU HAVE DONE THE SAME EXAMPLE OVER 50 TIMES CHOOSE SOMETHING ELSE
```chrn
@def
    var->
        name: str
        age: u8 #warn #bin
        pets: List<Pet> [!IsEmpty, Range(5, 15)]
    nest->
        struct Pet {
            name: str [!IsWhitespace]
            color: Color
        }

        enum Color {Red(u8) Blue(u8) Green(u8) } #hex
@end
```

## POSSIBLE FEATURES
Utilities to alter actual main file, such as trimming all strings.

Numerics: Binary, hex, octo. Allows for notation to serialize to be a specific notation. Unicode.

Maybe arithmetic
# Ok maybe

Matrix declarations.
HOW?

Unified serialization rules for any md file. 
Yaml, XML(Forgot this existed), Json, BINARY(I don't know) BINARY
