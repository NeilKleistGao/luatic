grammar luatic;

expr:
  KW_TRUE
  | KW_FALSE;

stat:
  PT_SEMI
  | global_stat;

global_stat: KW_GLOBAL IDENT PT_EQL expr;

number: INT | HEX | FLOAT | HEX_FLOAT;

string: NORMALSTRING;

// KEYWORDS
KW_GLOBAL: 'global';
KW_TRUE: 'true';
KW_FALSE: 'false';

// PUNCTUATION
PT_SEMI: ';';
PT_EQL: '=';
PT_LT: '<';
PT_RT: '>';
PT_LP: '(';
PT_RP: ')';
PT_LB: '{';
PT_RB: '}';
PT_LS: '[';
PT_RS: ']';

// IDENTITY
IDENT: [a-zA-Z_][a-zA-Z_0-9]*;

NORMALSTRING: '"' ( EscapeSequence | ~('\\' | '"'))* '"';

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

WS: [ \t\u000C\r\n]+ -> channel(1);
