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

#include <iostream>
#include <memory>

#include "cxxopts/cxxopts.hpp"
#include "luna/luna_vm.h"
#include "shared/version.h"

void StartRepl() {
  const auto vm = std::make_shared<LunaVirtualMachine>();
  std::cout << "Luna v" << LUNA_MAJOR_VERSION << "." << LUNA_MINOR_VERSION
            << "." << LUNA_PATCH_VERSION << std::endl;
  std::cout << "Press Ctrl-D to exit." << std::endl;

  std::string cmd;
  std::cout << "> ";
  while (std::getline(std::cin, cmd)) { // stop when meeting a Ctrl-D
    std::cout << cmd << std::endl; // TODO: implement real repl
    std::cout << "> ";
  }
}

int main(int argc, char* argv[]) {
  cxxopts::Options options("Luatic", "luna [options] [script [args]]");
  /* clang-format off */
  options.add_options()
          ("r,repl", "start REPL")
          ("b,binary", "execute binary file")
          ("filename", "script filename", cxxopts::value<std::string>())
          ("arguments", "arguments", cxxopts::value<std::vector<std::string>>())
          ("h, help", "help");
  // TODO: add more options.

  /* clang-format on */
  options.parse_positional({"filename", "arguments"});
  const auto results = options.parse(argc, argv);

  if (results.count("help") > 0) {
    std::cout << options.help() << std::endl;
  } else if (results.count("repl") > 0) {
    StartRepl();
  } else {
    std::cout << options.help() << std::endl;
  }
  // TODO: add more functions.

  return 0;
}
