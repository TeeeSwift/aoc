case 1: seed before map
map:          |------------------|
seeds:|-------|------|

split into before and overlap

case 2: seed after map
seeds:             |-----|--------|
map:  |------------------|

split into after and overlap

case 1: seed around map
seeds:  |---|------------------|--|
map:        |------------------|

split into before, after, and overlap

take a seed range
go through the maps
if there's one that overlaps, process seed,
push the non-overlapped part back onto the seed range stack

if you get through all maps and the seed hasn't been processed,   
just append it to the new array because it receives no conversion


*HINT
second start to the first end = overlap range
