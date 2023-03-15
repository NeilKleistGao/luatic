#include "instructions_pretty_printer.h"

namespace instructions {
void PrintInstruction(FILE* p_fp,
Instruction p_ins,const std::string& p_indent) {
switch(p_ins & 0x7F){
case 0: fprintf(p_fp, "%sMove\n", p_indent.c_str());break;
case 1: fprintf(p_fp, "%sLoad I\n", p_indent.c_str());break;
case 2: fprintf(p_fp, "%sLoad F\n", p_indent.c_str());break;
case 3: fprintf(p_fp, "%sLoad K\n", p_indent.c_str());break;
case 4: fprintf(p_fp, "%sLoad KX\n", p_indent.c_str());break;
case 5: fprintf(p_fp, "%sLoad False\n", p_indent.c_str());break;
case 6: fprintf(p_fp, "%sL False Skip\n", p_indent.c_str());break;
case 7: fprintf(p_fp, "%sLoad True\n", p_indent.c_str());break;
case 8: fprintf(p_fp, "%sLoad Nil\n", p_indent.c_str());break;
case 9: fprintf(p_fp, "%sGet Up Value\n", p_indent.c_str());break;
case 10: fprintf(p_fp, "%sSet Up Value\n", p_indent.c_str());break;
case 11: fprintf(p_fp, "%sGet Table Up\n", p_indent.c_str());break;
case 12: fprintf(p_fp, "%sGet Table\n", p_indent.c_str());break;
case 13: fprintf(p_fp, "%sGet I\n", p_indent.c_str());break;
case 14: fprintf(p_fp, "%sGet Field\n", p_indent.c_str());break;
case 15: fprintf(p_fp, "%sSet Table Up\n", p_indent.c_str());break;
case 16: fprintf(p_fp, "%sSet Table\n", p_indent.c_str());break;
case 17: fprintf(p_fp, "%sSet I\n", p_indent.c_str());break;
case 18: fprintf(p_fp, "%sSet Field\n", p_indent.c_str());break;
case 19: fprintf(p_fp, "%sNew Table\n", p_indent.c_str());break;
case 20: fprintf(p_fp, "%sSelf\n", p_indent.c_str());break;
case 21: fprintf(p_fp, "%sAdd I\n", p_indent.c_str());break;
case 22: fprintf(p_fp, "%sAdd K\n", p_indent.c_str());break;
case 23: fprintf(p_fp, "%sSub K\n", p_indent.c_str());break;
case 24: fprintf(p_fp, "%sMul K\n", p_indent.c_str());break;
case 25: fprintf(p_fp, "%sMod K\n", p_indent.c_str());break;
case 26: fprintf(p_fp, "%sPow K\n", p_indent.c_str());break;
case 27: fprintf(p_fp, "%sDiv K\n", p_indent.c_str());break;
case 28: fprintf(p_fp, "%sI Div K\n", p_indent.c_str());break;
case 29: fprintf(p_fp, "%sBit And K\n", p_indent.c_str());break;
case 30: fprintf(p_fp, "%sBit Or K\n", p_indent.c_str());break;
case 31: fprintf(p_fp, "%sBit Xor K\n", p_indent.c_str());break;
case 32: fprintf(p_fp, "%sShift Right I\n", p_indent.c_str());break;
case 33: fprintf(p_fp, "%sShift Left I\n", p_indent.c_str());break;
case 34: fprintf(p_fp, "%sAdd\n", p_indent.c_str());break;
case 35: fprintf(p_fp, "%sSub\n", p_indent.c_str());break;
case 36: fprintf(p_fp, "%sMul\n", p_indent.c_str());break;
case 37: fprintf(p_fp, "%sMod\n", p_indent.c_str());break;
case 38: fprintf(p_fp, "%sPow\n", p_indent.c_str());break;
case 39: fprintf(p_fp, "%sDiv\n", p_indent.c_str());break;
case 40: fprintf(p_fp, "%sI Div\n", p_indent.c_str());break;
case 41: fprintf(p_fp, "%sBit And\n", p_indent.c_str());break;
case 42: fprintf(p_fp, "%sBit Or\n", p_indent.c_str());break;
case 43: fprintf(p_fp, "%sBit Xor\n", p_indent.c_str());break;
case 44: fprintf(p_fp, "%sShift Left\n", p_indent.c_str());break;
case 45: fprintf(p_fp, "%sShift Right\n", p_indent.c_str());break;
case 46: fprintf(p_fp, "%sMM Bin\n", p_indent.c_str());break;
case 47: fprintf(p_fp, "%sMM Bin I\n", p_indent.c_str());break;
case 48: fprintf(p_fp, "%sMM Bin K\n", p_indent.c_str());break;
case 49: fprintf(p_fp, "%sUNM\n", p_indent.c_str());break;
case 50: fprintf(p_fp, "%sBit Not\n", p_indent.c_str());break;
case 51: fprintf(p_fp, "%sNot\n", p_indent.c_str());break;
case 52: fprintf(p_fp, "%sLen\n", p_indent.c_str());break;
case 53: fprintf(p_fp, "%sConcat\n", p_indent.c_str());break;
case 54: fprintf(p_fp, "%sClose\n", p_indent.c_str());break;
case 55: fprintf(p_fp, "%sTBC\n", p_indent.c_str());break;
case 56: fprintf(p_fp, "%sJump\n", p_indent.c_str());break;
case 57: fprintf(p_fp, "%sEqual\n", p_indent.c_str());break;
case 58: fprintf(p_fp, "%sLess Than\n", p_indent.c_str());break;
case 59: fprintf(p_fp, "%sLess Equal\n", p_indent.c_str());break;
case 60: fprintf(p_fp, "%sEqual K\n", p_indent.c_str());break;
case 61: fprintf(p_fp, "%sEqual I\n", p_indent.c_str());break;
case 62: fprintf(p_fp, "%sLess Than I\n", p_indent.c_str());break;
case 63: fprintf(p_fp, "%sLess Equal I\n", p_indent.c_str());break;
case 64: fprintf(p_fp, "%sGreater Than I\n", p_indent.c_str());break;
case 65: fprintf(p_fp, "%sGreater Equal I\n", p_indent.c_str());break;
case 66: fprintf(p_fp, "%sTest\n", p_indent.c_str());break;
case 67: fprintf(p_fp, "%sTest Set\n", p_indent.c_str());break;
case 68: fprintf(p_fp, "%sCall\n", p_indent.c_str());break;
case 69: fprintf(p_fp, "%sTail Call\n", p_indent.c_str());break;
case 70: fprintf(p_fp, "%sReturn %d, %d\n", p_indent.c_str(),((p_ins >> 7) & 0xFF),((p_ins >> 16) & 0xFF));break;
case 71: fprintf(p_fp, "%sReturn 0\n", p_indent.c_str());break;
case 72: fprintf(p_fp, "%sReturn 1\n", p_indent.c_str());break;
case 73: fprintf(p_fp, "%sFor Loop\n", p_indent.c_str());break;
case 74: fprintf(p_fp, "%sFor Prepare\n", p_indent.c_str());break;
case 75: fprintf(p_fp, "%sT For Prepare\n", p_indent.c_str());break;
case 76: fprintf(p_fp, "%sT For Call\n", p_indent.c_str());break;
case 77: fprintf(p_fp, "%sT For Loop\n", p_indent.c_str());break;
case 78: fprintf(p_fp, "%sSet List\n", p_indent.c_str());break;
case 79: fprintf(p_fp, "%sClosure\n", p_indent.c_str());break;
case 80: fprintf(p_fp, "%sVariable Arguments\n", p_indent.c_str());break;
case 81: fprintf(p_fp, "%sVariable Arguments Prepare\n", p_indent.c_str());break;
case 82: fprintf(p_fp, "%sExtra Arguments\n", p_indent.c_str());break;
default: fprintf(p_fp, "%sInvalid Op Code %d\n", p_indent.c_str(), p_ins);
}
}
}

