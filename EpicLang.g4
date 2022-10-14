grammar EpicLang;


Break : 'break';
Do : 'do';
Continue : 'continue';
Else : 'else';
Func : 'func';
For : 'for';
If : 'if';
Then : 'then';
Len : 'len';
NoneObj : 'none';
Print : 'print';
Return : 'return';
While : 'while';
TrueLiteral : 'true';
FalseLiteral : 'false';

// Lexer rules
LINE_COMMENT
    : '//' ~[\r\n]* -> skip;
NUMBER: [0-9]+;
SPACE: [ \t\n]+ -> skip;
VARIABLE: [a-zA-Z_][a-zA-Z0-9_]*;


// Parser rules
program: functionDecl* EOF;

block: '{' stmt* '}';

identifier: var=VARIABLE;

functionDecl: Func (name=VARIABLE) '(' (identifier (',' identifier)* )? ')' block;

stmt: block # BlockStmt
    | Print expr ';'# PrintStmt
    | Continue ';' # ContinueStmt
    | Break ';' # BreakStmt
    | Return expr? ';' # ReturnStmt
    | If expr Then stmt # IfStmt
    | If expr Then stmt Else stmt # IfElseStmt
    | For var=VARIABLE 'in' (lbound=expr) '..' (ubound=expr) Do stmt # ForStmt
    | While expr Do stmt # WhileStmt
    | (var=VARIABLE) '=' (rhs=expr) ';'# AssignStmt
    | expr '[' expr ']' '=' expr # AssignIdxStmt
    | expr ';' # SingleStmt
    | ';' # NullStmt
    ;

expr:  (name=VARIABLE) '(' (expr (',' expr)*)?  ')' # FuncCall
    | '(' expr ')' # ParenExpr
    | expr '[' expr ']' # IndexExpr
    | op=('+' | '-' | '!' | Len ) expr # UnaryExpr
    | expr op=('*' | '/' | '%' ) expr # BinExpr
    | expr op=('+' | '-') expr # BinExpr
    | expr op=('<' | '<=' | '>' | '>=' | '==' | '!=') expr #BinExpr
    | expr op=('&' | '|') expr #BinExpr
    | '[' (expr (',' expr)*)? ']' #ListExpr
    | TrueLiteral #TrueLiteral
    | FalseLiteral #FalseLiteral
    | num=NUMBER # NumExpr
    | var=VARIABLE # VarExpr
    | NoneObj # NoneExpr
    ;
