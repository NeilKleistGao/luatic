grammar Luatic;

// expr:
//   boolean
//   | integer
//   | number
//   | string
//   | array
//   | table
//   | lambda
//   | expr PT_LS expr PT_RS // select
//   | expr PT_LP (expr (PT_COMMA expr)*)? PT_RP // application
//   // operation expressions
//   | <assoc=right> expr PT_POW expr
//   | (PT_MINUS | KW_NOT) expr
//   | expr (PT_MUL | PT_DIV | PT_IDIV | PT_MOD) expr
//   | expr (PT_PLUS | PT_MINUS) expr
//   | expr (PT_LT | PT_RT | PT_LE | PT_GE | PT_EQ | PT_NE) expr
//   | expr KW_AND expr
//   | expr KW_OR expr
//   ;

// stat:
//   PT_SEMI
//   | global_stat
//   | local_stat
//   | local_in_stat
//   | full_if_stat
//   | while_stat
//   | do_stat
//   | for_in_stat
//   | func_stat
//   | ret_stat
//   // TODO: dialog
//   ;

dialog_block: PT_LSF IDENT PT_RSF block;
say_stat: character PT_COLON ((string PT_SEMI) | say_block);
// TODO: logic stat
stat: say_stat;

prgm: dialog_block*; // TODO: stat*
block: PT_LB stat* PT_RB;
say_block: PT_LB ((string PT_COMMA)* string PT_COMMA?) PT_RB; // TODO: logic block


// global_stat: KW_GLOBAL IDENT PT_ASGN expr PT_SEMI;
// local_stat: KW_LOCAL IDENT PT_ASGN expr PT_SEMI;
// local_in_stat: KW_LOCAL IDENT PT_ASGN expr KW_IN expr PT_SEMI;
// full_if_stat: if_stat elif_stat* else_state+;
// while_stat: KW_WHILE PT_LP expr PT_RP block;
// do_stat: KW_DO block KW_WHILE PT_LP expr PT_RP PT_SEMI;
// for_in_stat: KW_FOR param_list KW_IN expr block;
// func_stat: KW_FUNC IDENT PT_LP param_list PT_RP block;
// ret_stat: KW_RET expr PT_SEMI;

// boolean: KW_TRUE | KW_FALSE;
// integer: INT | HEX;
// number: FLOAT | HEX_FLOAT;

string: NORMALSTRING;
character: CHARASTRING;

// if_stat: KW_IF PT_LP expr PT_RP block;
// elif_stat: KW_ELSE KW_IF PT_LP expr PT_RP block;
// else_state: KW_ELSE block;

// param_list: (IDENT (PT_COMMA IDENT)*)?;

// array: PT_LB (expr (PT_COMMA expr)* PT_COMMA?)? PT_RB;
// kv_pair: PT_LS string PT_RS PT_ASGN expr;
// table: PT_LB (kv_pair (PT_COMMA kv_pair)* PT_COMMA?)? PT_RB;

// lambda: KW_FUNC PT_LP param_list PT_RP block PT_SEMI;

// KEYWORDS
// KW_GLOBAL: 'global';
// KW_TRUE: 'true';
// KW_FALSE: 'false';
// KW_LOCAL: 'local';
// KW_IN: 'in';
// KW_IF: 'if';
// KW_ELSE: 'else';
// KW_WHILE: 'while';
// KW_FOR: 'for';
// KW_FUNC: 'function';
// KW_GOTO: 'goto';
// KW_DO: 'do';
// KW_RET: 'return';
// KW_AND: 'and';
// KW_OR: 'or';
// KW_NOT: 'not';

// PUNCTUATION
PT_SEMI: ';';
PT_COLON: ':';
PT_COMMA: ',';
PT_ASGN: '=';
PT_LT: '<';
PT_RT: '>';
PT_LP: '(';
PT_RP: ')';
PT_LB: '{';
PT_RB: '}';
PT_LS: '[';
PT_RS: ']';
PT_PLUS: '+';
PT_MINUS: '-';
PT_MUL: '*';
PT_DIV: '/';
PT_IDIV: '//';
PT_MOD: '%';
PT_LE: '<=';
PT_GE: '>=';
PT_EQ: '==';
PT_NE: '~=';
PT_AND: '&';
PT_OR: '|';
PT_XN: '~';
PT_LSF: '<<';
PT_RSF: '>>';
PT_POW: '^';

// IDENTITY
IDENT: [a-zA-Z_][a-zA-Z_0-9]*;

// LITERALS
NORMALSTRING: '"' ( EscapeSequence | ~('\\' | '"'))* '"';
CHARASTRING: '[' ( EscapeSequence | ~('\\' | '[' | ']'))* ']';

INT: Digit+;
HEX: '0' [xX] HexDigit+;

FLOAT:
	Digit+ '.' Digit* ExponentPart?
	| '.' Digit+ ExponentPart?
	| Digit+ ExponentPart;

HEX_FLOAT:
	'0' [xX] HexDigit+ '.' HexDigit* HexExponentPart?
	| '0' [xX] '.' HexDigit+ HexExponentPart?
  | '0' [xX] HexDigit+ HexExponentPart;

fragment ExponentPart: [eE] [+-]? Digit+;

fragment HexExponentPart: [pP] [+-]? Digit+;

fragment DecimalEscape:
	'\\' Digit
	| '\\' Digit Digit
	| '\\' [0-2] Digit Digit;

fragment EscapeSequence:
	'\\' [abfnrtvz"'\\]
	| '\\' '\r'? '\n'
	| DecimalEscape
	| HexEscape
	| UtfEscape;

fragment HexEscape: '\\' 'x' HexDigit HexDigit;
fragment UtfEscape: '\\' 'u{' HexDigit+ '}';

fragment Digit: [0-9];
fragment HexDigit: [0-9a-fA-F];

fragment NESTED_STR: '=' NESTED_STR '=' | '[' .*? ']';

COMMENT: '--[' NESTED_STR ']' -> channel(1);

LINE_COMMENT:
	'--' (
		// --
		| '[' '='* // --[==
		| '[' '='* ~('=' | '[' | '\r' | '\n') ~('\r' | '\n')* // --[==AA
		| ~('[' | '\r' | '\n') ~('\r' | '\n')* // --AAA
	) ('\r\n' | '\r' | '\n' | EOF) -> channel(1);

WS: [ \t\u000C\r\n]+ -> channel(1);
