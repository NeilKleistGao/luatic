## Syntax Doc

### Keywords
- number
- string
- bool
- true
- false
- void
- global
- local
- in
- if
- match
- else
- function
- return
- for
- while
- goto
- yield

### Types
- Basic types: `3.14: number`, `"abcd": string`, `true: bool`, and `void`
- Table: `{ ["abc"] = 42 }: { ["abc"]: number }`. keys can only be string literals.
- Array: `{1, 2, 3, 4}: {number, 4}`.
- Function `(number, string) -> number`

### Variables
Define a global variable:
```lua
global x = 42;
```

Define a local variable:
```lua
local a = 42;
local b = 42 in ...;
local c = 42 in {
  ...
}
```

### Function
Define a function:
```lua
function f(x) {
  return x + 1;
}

function(x) {
  ...
}
```

### Operators
The same as operators in Lua.

### Condition
```lua
if (...) {
  ...
}
else if (...) {
  ...
}
else {
  ...
}
```

```lua
match ... {
  ... => ...;
  ... => {}
  _ => ...
}
```

### Loop
```lua
-- TODO
for (...; ...; ...) {
  ...
}
```

```lua
for ... in ... {
  ...
}
```

```lua
while (...) {
  ...
}
```

```lua
do {
  ...
} while (...)
```

### Comment
```lua
-- hello comment!
--[[
hello comment!
--
```

### Std
TODO?

### Dialog
```
<<label name>> {
  [name]: ""
  [name(status)]: ""
  ${--[[ execute code here. dialog scope. ]]--}
  [name]: {
    ...
  }
  [name]: "...${--[[ must return a string ]]--}..."
  ${ goto another label; }
}

<<pure code block>> ${
  ...
}
```

### Selection
```
<<label name>> {
  select {
    "..." => ${ ... },
    "..." => ${ ... }
  }
}
```

### Language Tags
```
-- lang: zh-CN
-- single line
-- must appear at the beginning of the file!
```

```
-- 1!5!
-- has highlight
-- but does nothing
```

### Coroutine
```lua
function f(x) {
  local t = x
  while (true) {
    t = t + 1
    yield t
  }

  return 0 -- or it would be typed to void.
}
```
