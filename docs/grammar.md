# Grammar

```
program : expr ( ';' expr ) *

expr : infix_expr
     | assinment_expr
     | ident_expr
     | grouped_expr
     | lambda_expr
     | call_expr
     | int_literal
     | string_literal

infix_expr : expr infix expr

assignment_expr : ident '=' expr

grouped_expr : '(' expr ')'

ident_list : ( ident ( ',' ident ) * ) ?
lambda_expr : '|' ident_list '|' expr

expr_list : ( expr ( ',' expr ) * ) ?
call_expr : expr '(' expr_list ')'

ident_expr : ident
```
