inputs  
gates  
outputs  

I'd like something that can take inputs in x00 form, and produce an output that's indexable
by z00, etc.

so given x00, y00, it should be z00  
i suppose it's just state or config  
let's go with config.

```rust
execute(map, gates) -> map of z indexes
expect(map) -> map of z-indexes
```

i need some way to traverse starting at x00, y00\
asdf

okay here are some rules i've identified.

if it outputs to z__, it should be an XOR\
if it has xy inputs, it should be XOR or AND\
if it's an xy XOR, it should go directly to an XOR\
if it's an xy AND, it should lead into an OR and an AND
