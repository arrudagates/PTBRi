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

If statements can also have an else statement after the code block and the end keyword, explained at the section "Else"

If statements require code to be run, everything between the if statement and the keyword `até aqui` is considered inner code of the if statement

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

## Else

Else statements are an optional follow up to an if statement, if that statement evaluates to false, the interpreter will jump to the `senão` keyword immediately after the end keyword, if present

Else statements require code to be run, everything between the else statement and the keyword `até aqui` is considered inner code of the else statement

Example:
```
se a é 1
mostre a
até aqui

senão
mostre "não é 1"
até aqui
```

## While Loops

While loops are written as the keyword `enquanto` followed by a comparison operator (and optionally a modifier) surrounded by a value/identifier/expression on both sides

While loops require code to be run, everything between the while loop and the keyword `até aqui` is considered inner code of the while loop

Valid comparison operators are:
- for
- não for

Valid comparison modifiers are:
- maior que
- menor que
- maior ou igual a
- menor ou igual a

Example:
```
enquanto a for 1
enquanto a não for 1
enquanto a for maior que 1
enquanto a não for menor ou igual a 1
```

## Function Definition

Function definitions are written as the keyword `defina a função` followed by an identifier and optionally the keyword `usando` and a chain of identifiers to be used inside the scope of the function

Functions require code to be run, everything between the function definition and the keyword `até aqui` is considered inner code of the function

Example:
```
defina a função teste
mostre "teste"
até aqui

defina a função soma usando a e b
mostre a mais b
até aqui
```

## Function Calls

Function calls are written as the keyword `função` followed by it's identifier and optionally the keyword `usando` and a chain of identifiers to be used by the function, if it's declared with identifiers

Example:
```
função teste


a é 2
b é 5

função soma usando a e b
```
