### ERROR HANDLING ERROR HANDLING ERROR HANDLING ERROR HANDLING ERROR HANDLING

## [BEHAVIOR]
- Ends program by default when type information is correct unless `#warn` is used.

- If a `#warn` is used and `?` is used, there will be a warn (or error) that there is an issue inside of the md file in every lint check if an error is present. So it CAN stay, but it will constantly be noted.

- If the `.chrn` file is turned into binary, the definition is immediately truncated and there needs to be a main.chrn file or else it is a "bag of bytes". Note: May have a way for it to undo itself by storing extremely specific header bytes that define what I should interpret the data as. Complicated :C . 

LANGUAGE BLOAT >>>>>>>

## [Types]
i8, u8, i16, u16, i32, u32, i64, u64
i128, u128, f16, f32, f64, f128, sized, unsized,
char, bool, str, %Struct/Enum,  nil (maybe not), BigInt, BigFloat, List, Map, Set

S% for structure of data.
E% for Enum type.

## [Operators]
`!`: Not operator.

`||`: Or operator.

`?`: Infers type and expects type consistency throughout entire `.chrn` file.  Name affects nothing.
Can be used as "I don't know" for the naming so it doesn't matter. Exmpl: name?: ? (likely illegal)
Maybe inference only works if there is one of one type?

(may remove)-> Can be used in struct definition to perform the same check, but also allows for the name after for clarity.

-- After instruction.
`!?`: Self declaring type inference that then alters props file.
!!?: Ignores the identifier or something...
invalid()
NO ALIASES PLEASE NO
!!!?: const ptr to a const char as a constexpr
--


`~`: Approximation operator acts as a range. Equivalent to: 0 <= x <= 6, 0..=6.

`(range)`: Explicit range syntax. The '=' is required. `0..=5`

'`.`': References past defined variable. `.person:= {..}`


## [Predicates]
`IsEmpty`: Checks if given array or string within a `.chrn` file is empty, or if a string only has whitespace. Ends program with error if true.
`IsWhitespace`: (MAY EXIST)

## Functions (Predicate)
`Len(x || range || ~)`: Checks if length of string is in condition. name: str (Len(0..=5), !IsEmpty)

## [Sections]
`bind`: Sets the `.chrn` file to affirm the syntax of. (or `attach`, `find`)
A bind call within the actual file with serialized data takes precedence.

`var`: Definition of syntax to align while serializing and deserializing.

`default_override`: What to default to when a language doesn't contain a particular type.
There is also a "like" category. A "JAVA_LIKE" category would have all of the int, short, logic for a batch of languages.

`complex_rules`: Define complex rules such as enum bounds.
Example:
    complex_rules:
        .age = ~5
        .state = ~6

## POSSIBLE FEATURES
Utilities to alter actual main file, such as trimming all strings.

Attributes ie. `#warn`, `#ign_if` (would remove anything that didn't align under condition rather than crash or warn.) `#scientific`, `#hex`, `#binary`, `#octo`

Numerics: Binary, hex, octo. Allows for notation to serialize to be a specific notation. Unicode.

Maybe more complex operations such as '/' and '*'

Matrix declarations.

Unified serialization rules for any md file.
