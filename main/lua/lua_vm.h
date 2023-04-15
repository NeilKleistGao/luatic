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

#ifndef LUATIC_LUA_VM_H
#define LUATIC_LUA_VM_H

#include <memory>
#include <vector>

/* clang-format off */
extern "C" {
#include "../backends/bin/lua-5.4.4/src/lstate.h"
#include "../backends/bin/lua-5.4.4/src/lauxlib.h"
#include "../backends/bin/lua-5.4.4/src/lualib.h"
  typedef struct lua_State lua_State;
};
/* clang-format on */

class LuaVM {
public:
  explicit LuaVM(const std::vector<std::string>& p_args);
  ~LuaVM();

  static std::shared_ptr<LuaVM>
    StartVM(const std::vector<std::string>& p_args = {});
  static void Halt();

  int DoFile(const std::string& p_filename);

private:
  static std::shared_ptr<LuaVM> s_ins;
  lua_State* m_state;
};

#endif //LUATIC_LUA_VM_H
