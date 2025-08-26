use std::fmt::Display;

// DNS zones are a tree that can be traversed up to recreate
// the domain name
// See https://en.wikipedia.org/wiki/Domain_Name_System#Structure
pub struct ZoneTree {
    name: String,
    parent: Option<Box<ZoneTree>>,
    children: Vec<Box<ZoneTree>>,
}

impl ZoneTree {
    pub fn new(name: String) -> Self {
        Self {
            name,
            parent: None,
            children: vec![],
        }
    }

    pub fn add(root: &mut Self, name: String) {
        let segments = name.split('.').rev();

        // The root has name = ""
        let mut children = root.children;
        let mut child = children[0];
        for seg in segments {
            for child in children {
                if child.name == seg {
                    children = child.children;
                }
            }
        }

        // This is wrong. I need to add the segment if it's not already there at each step
        child.children.push(Box::new(ZoneTree {
            name: seg,
            parent: Box::new(child),
            children: vec![],
        }))
    }
}

impl Display for ZoneTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Traverse up the tree
        todo!()
    }
}
