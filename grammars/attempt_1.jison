/* lexical grammar */
%lex
%%

\s+                   /* skip whitespace */
[a-zA-Z]\b  return 'IDENT'
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


term : abs | appish | appish abs {$$={callee:$1,arg:$2};};

var : IDENT {$$=yytext;};

abs : LAM var DOT  term   {$$={fp: $2, body: $4};};

app : appish high_precedence {$$={callee:$1,arg:$2};};
appish : app | high_precedence;

high_precedence : var | paren_ex;
paren_ex : "(" term ")" {$$={inner:$2};};