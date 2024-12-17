use core::fmt;

use crate::node::Node;

#[derive(Clone)]
pub struct Grid(pub Vec<Vec<Option<Node>>>);

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0.iter() {
            let row_str: String = row
                .iter()
                .map(|opt| match opt {
                    None => '.',
                    Some(Node::Box { .. }) => 'O',
                    Some(Node::Boxleft { .. }) => '[',
                    Some(Node::Boxright { .. }) => ']',
                    Some(Node::Wall { .. }) => '#',
                    Some(Node::Robot { .. }) => '@',
                })
                .collect();

            // Write each row's string to the formatter
            writeln!(f, "{}", row_str)?;
        }
        Ok(())
    }
}

// Implement Display for Grid
impl Grid {
    pub fn new(v: Vec<Vec<char>>) -> Self {
        let mut nodes: Vec<Vec<Option<Node>>> = vec![];

        for (y, line) in v.into_iter().enumerate() {
            let mut row: Vec<Option<Node>> = vec![];
            for (x, char) in line.into_iter().enumerate() {
                match char {
                    '@' => row.push(Some(Node::Robot { y, x })),
                    'O' => row.push(Some(Node::Box { y, x })),
                    '[' => row.push(Some(Node::Boxleft { y, x })),
                    ']' => row.push(Some(Node::Boxright { y, x })),
                    '#' => row.push(Some(Node::Wall { y, x })),
                    _ => row.push(None),
                };
            }

            nodes.push(row);
        }

        Self(nodes)
    }
}
