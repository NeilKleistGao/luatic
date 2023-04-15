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

#include "lua_vm.h"

#include <exception>

std::shared_ptr<LuaVM> LuaVM::s_ins = nullptr;

LuaVM::LuaVM(const std::vector<std::string>& p_args) {
  m_state = luaL_newstate();
  lua_gc(m_state, LUA_GCSTOP);
  luaL_openlibs(m_state);
  // TODO: create tables for args
  lua_gc(m_state, LUA_GCRESTART);
  lua_gc(m_state, LUA_GCGEN, 0, 0);
}

LuaVM::~LuaVM() {
  lua_close(m_state);
}

std::shared_ptr<LuaVM> LuaVM::StartVM(const std::vector<std::string>& p_args) {
  if (s_ins == nullptr) {
    s_ins = std::make_shared<LuaVM>(p_args);
    if (s_ins == nullptr) {
      throw std::bad_alloc();
    }
  }

  return s_ins;
}

void LuaVM::Halt() {
  if (s_ins != nullptr) {
    s_ins.reset();
  }
}

int LuaVM::DoFile(const std::string& p_filename) {
  int res = luaL_loadfilex(m_state, p_filename.c_str(), "bt");
  if (res == 0) {
    // TODO: load parameters
    return lua_pcall(m_state, 0, 0, 0);
  }

  return res;
}
