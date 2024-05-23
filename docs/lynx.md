## Lynx File Structure
```lua
filename = {}
filename.dialogues = {}
filename.dialogues.hello = {
  -- ...
}

filename.variables = {}
filename.variables.name = 0 -- ...

filename.code = {}
filename.code.some_hash_value_here = function()
end

return filename
```
