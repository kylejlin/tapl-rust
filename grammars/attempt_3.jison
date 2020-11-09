// Reason for revising:
//
// Remove left recursion so the recursive descent
// parser won't infinitely recurse.

/* lexical grammar */
%lex
%%

\s+                   /* skip whitespace */
[a-zA-Z]?\b  return 'IDENT'
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


abs : LAM var DOT  term   {$$={fp: $2, body: $4};};

callable = arg zero_or_more_args
zero_or_more_args = arg zero_or_more_args | EPSILON;

arg : var | paren_ex;
var : IDENT {$$=yytext;};
paren_ex : "(" term ")" {$$={inner:$2};};