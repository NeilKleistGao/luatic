message("backend: Lua5.4")

include_directories(backends/bin/lua-5.4.4/src)
add_library(backend STATIC backends/config/lua5.4/lua_vm.c)
