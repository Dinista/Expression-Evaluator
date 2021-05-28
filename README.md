# Expression-Evaluator
A basic implementation of an expression evaluator based in concepts such as lexer, parser, tree representation and Shunting Yard algorithm. <b> Theres two versions of the program, one wrote in Rust and another one wrote in Pyhton </b>, for comparison proprose. Also an portuguese article was written comaparing the two implementations.

The article can be found in the folder <i>/docs</i>

<b>This is the first project of the class Imperative and Object Oriented Programming Paradigm </b>

## How to use

### Input 
The expressions are made up of integers, addition (+), subtraction (-), multiplication (*) and division (/) operators and parentheses ((e)) and follow the precedence and associative commonly used in mathematics. An exemple of entry will be:

    (10 / 3 + 23) * (1 - 4)

### output
An example of output for the previus input will be:

> (3 + 23) * (1 - 4)\
26 * (1 - 4)\
26 * -3\
-78

## Evaluation Steps

### Lexer

Receives a string and returns an array of tokens.

      "31 * (4 + 10)" -> [31, "*", "(", 4, "+", 10, ")"]
      
### Parser

Receives a list of tokens and returns a tree that represents an expression.
      
      [31, "*", "(", 4, "+", 10, ")"]   ->      *
                                              /   \
                                             31    +
                                                 /   \
                                                4    10
### Shunting Yard

In computer science, the shunting-yard algorithm is a method for parsing mathematical expressions specified in infix notation. It can produce either a postfix notation string, also known as Reverse Polish notation (RPN), or an abstract syntax tree (AST).

### eval-step

Receives a tree that represents an expression and returns a tree with an evaluated operator.

       *                        *
     /   \                    /   \
    31    +        ->        31   14
        /   \
       4    10

### to-string

Receives a tree that represents an expression and returns a string that represents the same expression.

       *
     /   \
    31    +        ->        "31 * (4 + 10)"
        /   \
       4    10

## Unit tests

Both Rust and Python implementation has unit tests for each evaluation step.
