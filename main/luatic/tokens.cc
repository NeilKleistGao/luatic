/*
 * MIT License
 *
 * Copyright (c) 2023 Cunyuan(Holden) Gao
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

#include "tokens.h"

namespace std {
  std::string to_string(Keyword p_kw) {
    switch (p_kw) {
      case Keyword::KW_AND:
        return "and";
      case Keyword::KW_BREAK:
        return "break";
      case Keyword::KW_DO:
        return "do";
      case Keyword::KW_ELSE:
        return "else";
      case Keyword::KW_ELSEIF:
        return "elseif";
      case Keyword::KW_END:
        return "end";
      case Keyword::KW_FALSE:
        return "false";
      case Keyword::KW_FOR:
        return "for";
      case Keyword::KW_FUN:
        return "function";
      case Keyword::KW_GOTO:
        return "goto";
      case Keyword::KW_IF:
        return "if";
      case Keyword::KW_IN:
        return "in";
      case Keyword::KW_LOCAL:
        return "local";
      case Keyword::KW_NIL:
        return "nil";
      case Keyword::KW_NOT:
        return "not";
      case Keyword::KW_OR:
        return "or";
      case Keyword::KW_REPEAT:
        return "repeat";
      case Keyword::KW_RETURN:
        return "return";
      case Keyword::KW_THEN:
        return "then";
      case Keyword::KW_TRUE:
        return "true";
      case Keyword::KW_UNTIL:
        return "until";
      case Keyword::KW_WHILE:
        return "while";
    }
  }

  std::string to_string(Punctuation p_punc) {
    switch (p_punc) {
      case Punctuation::PUN_PLUS:
        return "+";
      case Punctuation::PUN_MINUS:
        return "-";
      case Punctuation::PUN_MUL:
        return "*";
      case Punctuation::PUN_DIV:
        return "/";
      case Punctuation::PUN_MOD:
        return "%";
      case Punctuation::PUN_POW:
        return "^";
      case Punctuation::PUN_LEN:
        return "#";
      case Punctuation::PUN_AND:
        return "&";
      case Punctuation::PUN_XOR:
        return "~";
      case Punctuation::PUN_OR:
        return "|";
      case Punctuation::PUN_LEFT_SHIFT:
        return "<<";
      case Punctuation::PUN_RIGHT_SHIFT:
        return ">>";
      case Punctuation::PUN_FD:
        return "//";
      case Punctuation::PUN_EQ:
        return "==";
      case Punctuation::PUN_NE:
        return "";
      case Punctuation::PUN_LE:
        return "<=";
      case Punctuation::PUN_GE:
        return ">=";
      case Punctuation::PUN_LESS:
        return "<";
      case Punctuation::PUN_GREAT:
        return ">";
      case Punctuation::PUN_ASSIGN:
        return "=";
      case Punctuation::PUN_LEFT_PAR:
        return "(";
      case Punctuation::PUN_RIGHT_PAR:
        return ")";
      case Punctuation::PUN_LEFT_BRA:
        return "{";
      case Punctuation::PUN_RIGHT_BRA:
        return "}";
      case Punctuation::PUN_LEFT_SQR:
        return "[";
      case Punctuation::PUN_RIGHT_SQR:
        return "]";
      case Punctuation::PUN_SEMI:
        return ";";
      case Punctuation::PUN_COLON:
        return ":";
      case Punctuation::PUN_COMMA:
        return ",";
      case Punctuation::PUN_DOT:
        return ".";
      case Punctuation::PUN_DOT2:
        return "..";
      case Punctuation::PUN_DOT3:
        return "...";
      case Punctuation::PUN_SPACE:
        return " ";
    }
  }

  std::string to_string(const Literal& p_lit) {
    if (p_lit.index() == 0) {
      return "literal \"" + std::get<std::string>(p_lit) + "\"";
    } else if (p_lit.index() == 1) {
      return "literal " + std::to_string(std::get<long long>(p_lit));
    } else {
      return "literal " + std::to_string(std::get<double>(p_lit));
    }
  }
} // namespace std
