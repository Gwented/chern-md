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

`?`: Infers type and expects type consistency throughout entire `.chrn` file.  Name affects nothing.
// I AM TRYING TO UNDERSTAND WHAT MY POINT WAS HERE I DONT REMEMBER.

Can be used as "I don't know" for the naming so it doesn't matter. Exmpl: name?: ? (likely illegal)
Maybe inference only works if there is one of one type? (WHAT DOES THAT MEAN?)

-- After instruction.
`!?`: Self declaring type inference that then alters props file.
!!?: Ignores the identifier or something...
invalid()
NO ALIASES PLEASE NO
--

# DOES NOT EXIST YET
`~`: Name bypass operator. ~str

// May remove this
`(range)`: Explicit range syntax. The '=' is required. `0..=5`

## [Predicates]
`IsEmpty`: Checks if the given array or string has a length of 0.

`IsWhitespace`: Checks if a string is only whitespace within UTF-8 standards, or is empty.

## Functions (Predicate)
(WHAT TO DO WITH THIS?)
`Range((range) OR ~x)`: Checks if length of string is in condition. name: str (Range(0..=5), !IsEmpty())

`Contains(Literal)`:

`StartsW(Literal)`:

`EndsW(Literal)`:

## Statements

`bind`: A statement that is where the serialized file is located that should be checked, serialized, or deserialized.

## [Sections]

// This sounds convoluted..
- Sections the type of data to be parsed is described. They exist as opposed a keywords so that data is always defined in a readable, expected manner.

- The `->` operator is used after section keywords to switch grammar rules.


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
        Ready(str) // Enums can only store ONE type (as of right now)
        InProgress
        Failed
    }

```

# DOES NOT EXIST YET
`override->`: What to default to when a language doesn't contain a particular type. Language defaults exist but this can change any if needed.

# Full example of language

// YOU HAVE DONE THE SAME EXAMPLE OVER 50 TIMES CHOOSE SOMETHING ELSE
// How make fucnctin pointer arrar for dynamic C array method OOP C like C++ or Java
```chrn
@def
    var->
        name: str
        age: u8 #warn #bin
        pets: List<Pet> [!IsEmpty, Range(0, 15)]
    nest->
        struct Pet {
            name: str [!IsWhitespace]
            color: Color
        }

        enum Color {Red(u8) #hex Blue(u8) #hex Green(u8) #hex }
@end
```


(Probably not a good idea)
There is also a "like" category. A "JAVA_LIKE" category would have all of the int, short, logic for a batch of languages.

`complex`: Define complex rules such as enum bounds.
Example:
    complex:
        State.variants = 5

## Arguments
`#warn`:

`#ign_if`: (would remove anything that didn't align under condition rather than crash or warn.)

`#scientific`, `#hex`, `#bin`, `#octo`: Numeric notations to output in serialized file.


## POSSIBLE FEATURES
Utilities to alter actual main file, such as trimming all strings.


Numerics: Binary, hex, octo. Allows for notation to serialize to be a specific notation. Unicode.

Maybe more complex operations such as '/' and '*'
# No

Matrix declarations.

Unified serialization rules for any md file.
