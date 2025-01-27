go by row
if you cross an even number of vertical edges, it's "out"
    if you cross an odd number, it's in

- replace any cell that's not in the loop

F7


for row in rows 
    let up = false;
    let vertical_edges_crossed = 0

    for char in chars 
        if character is '.' {
            if vertical_edges_crossed % 2 = 0 {
                add to "out" count
            }

            else add to in count
        }

        if it's |, add to vertical_edges_crossed
        if it's -, ignore
        if it's 7, see if the last weird onew as L,
            if it was F, we good
        if it's 
