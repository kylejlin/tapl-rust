/* description: Parses end executes mathematical expressions. */

/* lexical grammar */
%lex
%%

\s+                   /* skip whitespace */
[0-9]+("."[0-9]+)?\b  return 'IDENT'
"lam"                   return 'LAM'
"." return "DOT"
"("                   return '('
")"                   return ')'
<<EOF>>               return 'EOF'
.                     return 'INVALID'

/lex

/* operator associations and precedence */

%start expressions

%% /* language grammar */

expressions
    : term EOF
        {return $1;}
    ;


term : abs | callable | callable abs {$$={callee:$1,arg:$2};};

var : IDENT {$$=yytext;};

abs : LAM var DOT  term   {$$={fp: $2, body: $4};};

callable = arg callable_rhs
callable_rhs = arg callable_rhs | EPSILON;
// a b c d
// raw = a (b (c (d EPSILON)))
// ->
// correct(raw) = (((a b) c) d) EPSILON
// correct(x y) = x y
// correct()

/*
a b
raw = a b
correct = a b

correct T y = T y

a b c
raw = a (b c)
correct = (a b) c

correct T (y1 y2)
    = correct (T y1) y2
    = (T y1) y2

a b c d
raw = a (b (c d))
correct = ((a b) c) d

correct T (y1 (y21 y22))
    = correct (T y1) (y21 y22)
    = correct ((T y1) y21) y22
    = ((T y1) y21) y22

correct T y = T y
correct T (y1 y2) = correct (T y1) y2

*/


arg : var | paren_ex;
paren_ex : "(" term ")" {$$={inner:$2};};