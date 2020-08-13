# Grammar

```
program : expression ';' *

expression : infix_expression
           | assinment_expression
           | identifier_expression
           | integer_literal
           | grouped_expression

infix_expression : expression infix expression

assignment_expression : ident '=' expression

grouped_expression : '(' expression ')'

identifier_expression : ident
```
