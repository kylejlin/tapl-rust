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

app : callable arg {$$={callee:$1,arg:$2};};
callable : app | arg;

arg : var | paren_ex;
paren_ex : "(" term ")" {$$={inner:$2};};