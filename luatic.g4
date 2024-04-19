grammar luatic;

expr:
  KW_TRUE
  | KW_FALSE;

stat:
  PT_SEMI
  | global_stat;

global_stat: KW_GLOBAL IDENT PT_EQL expr;

// KEYWORDS
KW_GLOBAL: 'global';
KW_TRUE: 'true';
KW_FALSE: 'false';

// PUNCTUATION
PT_SEMI: ';';
PT_EQL: '=';

// IDENTITY
IDENT: [a-zA-Z_][a-zA-Z_0-9]*;

WS: [ \t\u000C\r\n]+ -> channel(1);
