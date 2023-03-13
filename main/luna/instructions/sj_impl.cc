//
// Created by NeilK on 2023/3/13.
//

#include "sj_impl.h"

namespace instructions {
  int Jump(const InstsJ& p_inst, const std::shared_ptr<LunaStack>& p_stack) {
    return static_cast<int>(p_inst.sj);
  }

} // namespace instructions
