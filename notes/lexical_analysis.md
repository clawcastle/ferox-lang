# Lexemes and Tokens

Lexical analysis is about scanning through the list of characters and grouping them together into the smallest sequences that still represent something. Each such a blob of characters is called a lexeme.
If we take the example line of code here: 
`var language = "lox";`

The lexemes are:
`var`, `language`, `=`, `"lox"` and `;`.

The lexemes are only the raw substrings of source code. When bundling these with other information, we get tokens. This other information includes:

## Token type
Instead of directly comparing the raw strings of the lexemes, we represent each token by a distinct token type, in this case represented by a Rust enum.

# Literal value
Since the scanner has to go through each character in a literal to identify it, it can also convert that textual representation to the runtime object that will be used by the interpreter later.

# Location information
If we want to provide more sophisticated error feedback, we might want to store some information about where in the source code a given token is found.