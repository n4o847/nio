# Grammar

```
program : expr ( ';' expr ) *

expr : infix_expr
     | assinment_expr
     | ident_expr
     | grouped_expr
     | lambda_expr
     | int_literal

infix_expr : expr infix expr

assignment_expr : ident '=' expr

grouped_expr : '(' expr ')'

lambda_expr : '|' ( ident ( ',' ident ) * ) ? '|' expr

ident_expr : ident
```
