for part 2, i should work backwards. I don't need to derive the combinations
    every time.


say i have a design of 'wwr' and patterns 'w' and 'wwr' and 'wr'
there are 2 ways to make wwr

each time design_possible_2 resolves, i should add the design and count to a
hashmap so i don't have to dervice it again.

design_possible_2 needs to take (patterns, design, hashmap)

hashmap must be mutable Arc<Mutex<>>
