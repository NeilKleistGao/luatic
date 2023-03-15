/* THIS FILE IS AUTOMATICALLY GENERATED. PLEASE DO NOT MODIFY. */
#include "instructions_pretty_printer.h"

namespace instructions {
  void PrintInstruction(FILE* p_fp,
                        Instruction p_ins,
                        const std::string& p_indent) {
    switch (p_ins & 0x7F) {
      case 0:
        fprintf(p_fp, "%s Move\n", p_indent.c_str());
        break;
      case 1:
        fprintf(p_fp, "%s Load I\n", p_indent.c_str());
        break;
      case 2:
        fprintf(p_fp, "%s Load F\n", p_indent.c_str());
        break;
      case 3:
        fprintf(p_fp, "%s Load K\n", p_indent.c_str());
        break;
      case 4:
        fprintf(p_fp, "%s Load KX\n", p_indent.c_str());
        break;
      case 5:
        fprintf(p_fp, "%s Load False\n", p_indent.c_str());
        break;
      case 6:
        fprintf(p_fp, "%s L False Skip\n", p_indent.c_str());
        break;
      case 7:
        fprintf(p_fp, "%s Load True\n", p_indent.c_str());
        break;
      case 8:
        fprintf(p_fp, "%s Load Nil\n", p_indent.c_str());
        break;
      case 9:
        fprintf(p_fp, "%s Get Up Value\n", p_indent.c_str());
        break;
      case 10:
        fprintf(p_fp, "%s Set Up Value\n", p_indent.c_str());
        break;
      case 11:
        fprintf(p_fp, "%s Get Table Up\n", p_indent.c_str());
        break;
      case 12:
        fprintf(p_fp, "%s Get Table\n", p_indent.c_str());
        break;
      case 13:
        fprintf(p_fp, "%s Get I\n", p_indent.c_str());
        break;
      case 14:
        fprintf(p_fp, "%s Get Field\n", p_indent.c_str());
        break;
      case 15:
        fprintf(p_fp, "%s Set Table Up\n", p_indent.c_str());
        break;
      case 16:
        fprintf(p_fp, "%s Set Table\n", p_indent.c_str());
        break;
      case 17:
        fprintf(p_fp, "%s Set I\n", p_indent.c_str());
        break;
      case 18:
        fprintf(p_fp, "%s Set Field\n", p_indent.c_str());
        break;
      case 19:
        fprintf(p_fp, "%s New Table\n", p_indent.c_str());
        break;
      case 20:
        fprintf(p_fp, "%s Self\n", p_indent.c_str());
        break;
      case 21:
        fprintf(p_fp, "%s Add I\n", p_indent.c_str());
        break;
      case 22:
        fprintf(p_fp, "%s Add K\n", p_indent.c_str());
        break;
      case 23:
        fprintf(p_fp, "%s Sub K\n", p_indent.c_str());
        break;
      case 24:
        fprintf(p_fp, "%s Mul K\n", p_indent.c_str());
        break;
      case 25:
        fprintf(p_fp, "%s Mod K\n", p_indent.c_str());
        break;
      case 26:
        fprintf(p_fp, "%s Pow K\n", p_indent.c_str());
        break;
      case 27:
        fprintf(p_fp, "%s Div K\n", p_indent.c_str());
        break;
      case 28:
        fprintf(p_fp, "%s I Div K\n", p_indent.c_str());
        break;
      case 29:
        fprintf(p_fp, "%s Bit And K\n", p_indent.c_str());
        break;
      case 30:
        fprintf(p_fp, "%s Bit Or K\n", p_indent.c_str());
        break;
      case 31:
        fprintf(p_fp, "%s Bit Xor K\n", p_indent.c_str());
        break;
      case 32:
        fprintf(p_fp, "%s Shift Right I\n", p_indent.c_str());
        break;
      case 33:
        fprintf(p_fp, "%s Shift Left I\n", p_indent.c_str());
        break;
      case 34:
        fprintf(p_fp, "%s Add\n", p_indent.c_str());
        break;
      case 35:
        fprintf(p_fp, "%s Sub\n", p_indent.c_str());
        break;
      case 36:
        fprintf(p_fp, "%s Mul\n", p_indent.c_str());
        break;
      case 37:
        fprintf(p_fp, "%s Mod\n", p_indent.c_str());
        break;
      case 38:
        fprintf(p_fp, "%s Pow\n", p_indent.c_str());
        break;
      case 39:
        fprintf(p_fp, "%s Div\n", p_indent.c_str());
        break;
      case 40:
        fprintf(p_fp, "%s I Div\n", p_indent.c_str());
        break;
      case 41:
        fprintf(p_fp, "%s Bit And\n", p_indent.c_str());
        break;
      case 42:
        fprintf(p_fp, "%s Bit Or\n", p_indent.c_str());
        break;
      case 43:
        fprintf(p_fp, "%s Bit Xor\n", p_indent.c_str());
        break;
      case 44:
        fprintf(p_fp, "%s Shift Left\n", p_indent.c_str());
        break;
      case 45:
        fprintf(p_fp, "%s Shift Right\n", p_indent.c_str());
        break;
      case 46:
        fprintf(p_fp, "%s MM Bin\n", p_indent.c_str());
        break;
      case 47:
        fprintf(p_fp, "%s MM Bin I\n", p_indent.c_str());
        break;
      case 48:
        fprintf(p_fp, "%s MM Bin K\n", p_indent.c_str());
        break;
      case 49:
        fprintf(p_fp, "%s UNM\n", p_indent.c_str());
        break;
      case 50:
        fprintf(p_fp, "%s Bit Not\n", p_indent.c_str());
        break;
      case 51:
        fprintf(p_fp, "%s Not\n", p_indent.c_str());
        break;
      case 52:
        fprintf(p_fp, "%s Len\n", p_indent.c_str());
        break;
      case 53:
        fprintf(p_fp, "%s Concat\n", p_indent.c_str());
        break;
      case 54:
        fprintf(p_fp, "%s Close\n", p_indent.c_str());
        break;
      case 55:
        fprintf(p_fp, "%s TBC\n", p_indent.c_str());
        break;
      case 56:
        fprintf(p_fp, "%s Jump\n", p_indent.c_str());
        break;
      case 57:
        fprintf(p_fp, "%s Equal\n", p_indent.c_str());
        break;
      case 58:
        fprintf(p_fp, "%s Less Than\n", p_indent.c_str());
        break;
      case 59:
        fprintf(p_fp, "%s Less Equal\n", p_indent.c_str());
        break;
      case 60:
        fprintf(p_fp, "%s Equal K\n", p_indent.c_str());
        break;
      case 61:
        fprintf(p_fp, "%s Equal I\n", p_indent.c_str());
        break;
      case 62:
        fprintf(p_fp, "%s Less Than I\n", p_indent.c_str());
        break;
      case 63:
        fprintf(p_fp, "%s Less Equal I\n", p_indent.c_str());
        break;
      case 64:
        fprintf(p_fp, "%s Greater Than I\n", p_indent.c_str());
        break;
      case 65:
        fprintf(p_fp, "%s Greater Equal I\n", p_indent.c_str());
        break;
      case 66:
        fprintf(p_fp, "%s Test\n", p_indent.c_str());
        break;
      case 67:
        fprintf(p_fp, "%s Test Set\n", p_indent.c_str());
        break;
      case 68:
        fprintf(p_fp, "%s Call\n", p_indent.c_str());
        break;
      case 69:
        fprintf(p_fp, "%s Tail Call\n", p_indent.c_str());
        break;
      case 70:
        fprintf(p_fp,
                "%s Return %d, %d\n",
                p_indent.c_str(),
                ((p_ins >> 7) & 0xFF),
                ((p_ins >> 16) & 0xFF));
        break;
      case 71:
        fprintf(p_fp, "%s Return 0\n", p_indent.c_str());
        break;
      case 72:
        fprintf(p_fp, "%s Return 1\n", p_indent.c_str());
        break;
      case 73:
        fprintf(p_fp, "%s For Loop\n", p_indent.c_str());
        break;
      case 74:
        fprintf(p_fp, "%s For Prepare\n", p_indent.c_str());
        break;
      case 75:
        fprintf(p_fp, "%s T For Prepare\n", p_indent.c_str());
        break;
      case 76:
        fprintf(p_fp, "%s T For Call\n", p_indent.c_str());
        break;
      case 77:
        fprintf(p_fp, "%s T For Loop\n", p_indent.c_str());
        break;
      case 78:
        fprintf(p_fp, "%s Set List\n", p_indent.c_str());
        break;
      case 79:
        fprintf(p_fp, "%s Closure\n", p_indent.c_str());
        break;
      case 80:
        fprintf(p_fp, "%s Variable Arguments\n", p_indent.c_str());
        break;
      case 81:
        fprintf(p_fp, "%s Variable Arguments Prepare\n", p_indent.c_str());
        break;
      case 82:
        fprintf(p_fp, "%s Extra Arguments\n", p_indent.c_str());
        break;
      default:
        fprintf(p_fp, "%s Invalid Op Code %d\n", p_indent.c_str(), p_ins);
    }
  }
} // namespace instructions
