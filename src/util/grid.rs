pub struct ArenaTree<T> {
    arena: Vec<Node<T>>,
}

struct Node<T> {
    value: T,
    parent: Option<usize>,
    children: Vec<usize>,
}

impl<T> ArenaTree<T> {
    pub fn new(value: T) -> Self {
        ArenaTree {
            arena: vec![Node {
                value,
                parent: None,
                children: vec![],
            }],
        }
    }

    pub fn insert(&mut self, node: usize, value: T) -> usize {
        let new_node_idx = self.arena.len();

        let new_node = Node {
            value,
            parent: Some(node),
            children: vec![],
        };

        self.arena.push(new_node);
        self.arena[node].children.push(new_node_idx);
        new_node_idx
    }

    pub fn get_parent(&self, node: usize) -> Option<usize> {
        self.arena[node].parent
    }

    pub fn get_children(&self, node: usize) -> Vec<usize> {
        self.arena[node].children.clone()
    }

    pub fn has_children(&self, node: usize) -> bool {
        !self.arena[node].children.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.arena.iter().map(|x| &x.value)
    }
}

impl<T> std::ops::Index<usize> for ArenaTree<T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.arena[index].value
    }
}
