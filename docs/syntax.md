## Syntax Doc

### Types
- Basic types: `42: int`, `3.14: num`, `"abcd": string`, `true: bool`, `nil: nil`
- Table: `{ ["a" 1] [2 2] }`, `{1 2 3 4}`(i.e., `{[1 1] [2 2] [3 3] [4 4]}`)
- Record
- Protocol

### Variables
Define a global variable:
```
(def a 42)
```

Define a local variable:
```
(let [(a 42)] (+ a 1))
```

### Function
Define a function:
```
(defn inc [x] (+ x 1))
```

Define an anonymous function:
```
(fn [x] (+ x 1))
```

### Block
```
(do
  (...)
  (...))
```

### Operators
The same as operators in Lua.

### Record
```
(record Student [name age]
  (extend Human)
  (def id 114514)
  (defn drink-tea [self tea] (...)))
(def s (Student "T" 24))
```

### Protocol
```
(protocol Foo [x]
  (def a 42)
  (defn f [self] (:a self))
  (undef g)
  (override h [self] (+ (:b super) 1)))
(record Bar [x]
  (with (Foo nil))
  (def g 0)
  (defn h))
```

### Mutation
```
(let [a {1 2 3 4 5}] (do
  (set! (:1 a) 0)))
```

### Condition
```
(if true (do ...) (do ...))
```

```
(match
  (== v 1) ("1")
  (== v 2) ("2")
  ("wtf"))
```

### Loop
```
(for [i 0] (< i 10) (+ i 1) (do ...))
```

### Comment
```
; hello comment!
```

### Import
```
(import "another.ltc")
```
