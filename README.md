# PTBR interpreter

![example workflow](https://github.com/arrudagates/PTBRi/actions/workflows/rust.yml/badge.svg)

## O que é PTBR?

PTBR é uma linguagem que está sendo desenvolvida para facilitar o entendimento com o objetivo principal de ser uma forma simples de apresentar falantes de português sem nenhum conhecimento técnico em programação

Um exemplo:

```
a é 5
b é 2
mostre a vezes b
```

## Onde estamos no desenvolvimento?

O PTBRi começou como um fork do interpretador do rust-lci, um interpretador LOLCODE, seguindo essa lógica, a própria linguagem começou como LOLCODE. Essa base de código antiga pode ser encontrada em [old-PTBRi](https://github.com/arrudagates/old-PTBRi)

Esta é uma reescrita do zero com uma reescrita completa de tudo, desde a especificação da linguagem até o analisador e o interpretador.

O código atual está longe de estar pronto para produção, há muito a ser feito, a linguagem está longe de ser 100% implementada e tanto o analisador quanto o interpretador são códigos em nível de protótipo.

## Como usar

Atualmente temos um protótipo básico, ele pode ser testado executando o código com um arquivo ptbr como argumento:

```
cargo run tests/basic.ptbr
// ou compilado
ptbri tests/basic.ptbr
```

## O que já está implementado?

Você pode verificar a sintaxe já implementada [aqui](ptbr_definitions.md)
