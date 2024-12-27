Let's just represent everything in binary.
fuck it.

I need to write a function that takes a string of commands, and works through
them backwards.

Should it belong to the computer? Sure why not.

computer.duplicate accepts a command and returns the smallest number that
generates it.

So really, it's not even a method.

Program: 2,4,1,2,7,5,1,7,4,4,0,3,5,5,3,0

|code | operand | operation | value | a | b | c |
|---|---|---|---|---|---|---|
|2|4|takes lowest 3 bits of a into b|0|0|0||
|1|2|b = b ^ 2|0|0|0||
|7|5|c = a / (2 pow b)|0|0|0||
|1|7|b = b ^ 7|0|0|0||
|4|4|b = b ^ c|0|0|0||
|0|3|a = a / 8|0|0|0||
|5|5|print b|0|0|0||
|3|0|move to beginning|0|0|0||

in order to recreate the program, i need to work backwards.

The number that gets printed is b ^ c so get pairs of final b and c that work

for each b, work backwards to get "original b"

**original b**
- flip it via  b ^ 7
- xor against 2 via b ^ 2

add original b to candidate number and see if the algorith prints the target
number
