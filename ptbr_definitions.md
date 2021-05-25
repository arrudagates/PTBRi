# Syntax definitions for PTBR

PTBR is an exotic language because it lacks common symbols used in most languages, like parenthesis, and also makes use of characters usually ignored, like `é`

## Variable definition

Variable definitions are written as Identifier followed by the keyword `é` followed by the value/expression to be assigned

Example:
```
a é 1
```

## Output

Output to the screen is written as the keyword `mostre` followed by the value/expression to be displayed

Multiple values can be displayed on the same line by appending them in the after the last argument of the `mostre` keyword using the `e` keyword

Example:
```
mostre a
mostre a e b
mostre a e b e a mais b
```

## Expressions

Expressions are written as the keyword of the expression surrounded with a value/identifier/expression on both sides

Valid expression keywords are:
- mais
- menos
- vezes
- dividido por

Example:
```
a mais 1
a menos 1
a vezes 1
a dividido por 1
```

## If Statements

If statements are written as the keyword `se` followed by a comparison operator (and optionally a modifier) surrounded by a value/identifier/expression on both sides

Valid comparison operators are:
- é
- não é

Valid comparison modifiers are:
- maior que
- menor que
- maior ou igual a
- menor ou igual a

Example:
```
se a é 1
se a não é 1
se a é maior que 1
se a é menor que 1
se a não é maior que 1
se a não é menor que 1
se a é maior ou igual a 1
se a é menor ou igual a 1
```
