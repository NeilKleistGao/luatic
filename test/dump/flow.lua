-- BINARY CHUNK
-- up values size: 1
-- PROTOTYPE
-- source: @flow.lua
-- define: line 0 to line 0
-- num params: 0
-- is var arg: yes
-- max stack size: 8
-- code: 
---- Variable Arguments Prepare
---- Load I at 0, value: 2
---- Less Than I
---- Jump: 4
---- Get Table Up
---- Load True at 1
---- Call
---- Jump: 10
---- Load I at 0, value: 1
---- Less Than I
---- Jump: 4
---- Get Table Up
---- Load False at 1
---- Call
---- Jump: 3
---- Get Table Up
---- Load Nil: [1, 1]
---- Call
---- Load False at 0
---- Test: if (R[0] != 0) then pc++
---- Jump: 4
---- Get Table Up
---- Load K(constant) from 1 to 1
---- Call
---- Jump: -7
---- Load I at 0, value: 1
---- Load I at 1, value: 10
---- Load I at 2, value: 1
---- For Prepare
---- Get Table Up
---- Move
---- Call
---- For Loop
---- Get Table Up
---- Load K(constant) from 2 to 1
---- Call
---- Load False at 0
---- Test: if (R[0] != 0) then pc++
---- Jump: -6
---- Jump: 1
---- Jump: 0
---- Load I at 0, value: 1
---- Load I at 1, value: 10
---- Load I at 2, value: 1
---- For Prepare
---- Load I at 4, value: 1
---- Load I at 5, value: 10
---- Load I at 6, value: 1
---- For Prepare
---- Jump: 2
---- For Loop
---- For Loop
---- Return: 0, 1
-- constant: 
---- String
---- String
---- String
