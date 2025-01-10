- [ ] rewrite get_search_ranges  

just get start and end coordinates  
(row, start, end)  

use vectors to get target coordinates  
get left: start = (-1,-1), (0,-1), (1,-1)  
get right: end = (-1,1), (0,1), (1,1)  
get up: ...  
get down: ...  
etc.  

if target coordinate is a symbol, note it  
  
part 2 needs a hashmap  

key: coordinate  
value: (char, Vec<part_number>)  

filter hashmap values where char is * and vec.len() is 2
multiply and sum
