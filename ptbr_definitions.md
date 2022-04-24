# Definições de sintaxe para PTBR

PTBR é uma linguagem exótica porque carece de símbolos comuns usados na maioria das linguagens, como parênteses, e também faz uso de caracteres normalmente ignorados, como `é`

## Definição de variável

As definições de variáveis são escritas como Identificador seguido pela palavra-chave `é` seguida pelo valor/expressão/entrada do usuário a ser atribuída

Exemplo:
```
a é 1
```

## Saída

A saída para a tela é escrita como a palavra-chave `mostre` seguida pelo valor/expressão a ser exibido

Vários valores podem ser exibidos na mesma linha anexando-os após o último argumento da palavra-chave `mostre` usando a palavra-chave `e`

Exemplo:
```
mostre a
mostre a e b
mostre a e b e a mais b
```

## Expressões

As expressões são escritas como a palavra-chave da expressão cercada por um valor/identificador/expressão em ambos os lados

As palavras-chave de expressão válidas são:
- mais
- menos
- vezes
- dividido por

Exemplo:
```
a mais 1
a menos 1
a vezes 1
a dividido por 1
```

## Declarações If

Se as instruções forem escritas como a palavra-chave `se` seguida por um operador de comparação (e opcionalmente um modificador) cercado por um valor/identificador/expressão em ambos os lados

As instruções if também podem ter uma instrução else após o bloco de código e a palavra-chave end, explicada na seção "Else"

Se as instruções exigem que o código seja executado, tudo entre a instrução if e a palavra-chave `até aqui` é considerado código interno da instrução if

Os operadores de comparação válidos são:
- é
- não é

Os modificadores de comparação válidos são:
- maior que
- menor que
- maior ou igual a
- menor ou igual a

Você também pode adicionar condições lógicas para as comparações colocando uma em cada lado de `e` (e) ou `ou` (ou)

Exemplo:
```
se a é 1
se a não é 1
se a é maior que 1
se a é menor que 1
se a não é maior que 1
se a não é menor que 1
se a é maior ou igual a 1
se a é menor ou igual a 1

se a é 1 e b é 2
se a é 1 ou b é 2
```

## Senão

As instruções Else são um acompanhamento opcional de uma instrução if, se essa instrução for avaliada como falsa, o interpretador pulará para a palavra-chave `senão` imediatamente após a palavra-chave end, se presente

As instruções Else requerem que o código seja executado, tudo entre a instrução else e a palavra-chave `até aqui` é considerado código interno da instrução else

Exemplo:
```
se a é 1
mostre a
até aqui

senão
mostre "não é 1"
até aqui
```

## While Loops

Enquanto os loops são escritos como a palavra-chave `enquanto` seguida por um operador de comparação (e opcionalmente um modificador) cercado por um valor/identificador/expressão em ambos os lados

Enquanto os loops exigem que o código seja executado, tudo entre o loop while e a palavra-chave `até aqui` é considerado código interno do loop while

Os operadores de comparação válidos são:
- for
- não for

Os modificadores de comparação válidos são:
- maior que
- menor que
- maior ou igual a
- menor ou igual a

Você também pode adicionar condições lógicas para as comparações colocando uma em cada lado de `e` (e) ou `ou` (ou)

Exemplo:
```
enquanto a for 1
enquanto a não for 1
enquanto a for maior que 1
enquanto a não for menor ou igual a 1

enquanto a for 1 e b for 2
enquanto a for 1 ou b for 2
```
## Definição de função

As definições de função são escritas como a palavra-chave `defina a função` seguida por um identificador e opcionalmente a palavra-chave `usando` e uma cadeia de identificadores a serem usados dentro do escopo da função

As funções exigem que o código seja executado, tudo entre a definição da função e a palavra-chave `até aqui` é considerado código interno da função

Exemplo:
```
defina a função teste
mostre "teste"
até aqui

defina a função soma usando a e b
mostre a mais b
até aqui
```

## Chamadas de Função

As chamadas de funções são escritas como a palavra-chave `função` seguida de seu identificador e opcionalmente a palavra-chave `usando` e uma cadeia de identificadores a serem usados pela função, caso ela seja declarada com identificadores

Exemplo:
```
função teste


a é 2
b é 5

função soma usando a e b
```

## Entrada do usuário

O uso de entrada é feito atribuindo a palavra-chave `entrada de` seguida pelo tipo de dados que está sendo inserido em uma variável

Tipos de entrada suportados:
- número
- texto

Example:
```
a é entrada de número
b é entrada de texto

mostre "a é " e a
mostre "b é " e b
```

## Retornos de função

Os retornos de função são escritos como a palavra-chave `retorne` seguida pelo valor/expressão/identificador a ser retornado

Os retornos de função devem estar dentro de sua respectiva função, porque os valores retornados estão vinculados ao escopo dessa função, mas também podem ser aninhados dentro de outros blocos de código, como os de instruções if que residem dentro da função

Os retornos de função também significam que as funções agora podem ser chamadas como valores, se essa função retornar um valor

Exemplo:
```
defina a função soma usando a e b

retorne a mais b

até aqui


a é 2
b é 8

mostre função soma usando a e b
```

## Comentários

Existem dois tipos de comentários disponíveis, de várias linhas e de uma linha:

Comentários de várias linhas são cercados pela palavra-chave de prefixo `comentário` e o sufixo `fim do comentário`

Comentários de linha única são cercados pela palavra-chave de prefixo `comentário` e o final da linha

Exemplo:
```
comentário this is a single-line comment

comentário
this is a
multi-line
comment
fim do comentário
```
