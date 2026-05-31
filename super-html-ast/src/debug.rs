use pretty_tree::ToPrettyTree;
use pretty_tree::PrettyTree;

use crate::AttributeMap;
use crate::ast::{Node, Element, Fragment};

impl ToPrettyTree for Node {
    fn to_pretty_tree(&self) -> PrettyTree {
        match self {
            Node::Text(x) => {
                let x = x.trim();
                if x.trim().is_empty() {
                    return PrettyTree::Empty
                }
                PrettyTree::str(x)
            },
            Node::Element(x) => x.to_pretty_tree(),
            Node::Fragment(x) => x.to_pretty_tree(),
        }
    }
}

impl ToPrettyTree for Element {
    fn to_pretty_tree(&self) -> PrettyTree {
        let tag = self.tag.as_original();
        let xs = self.attributes
            .iter()
            .map(|(k, v)| {
                let k = k.as_str();
                let v = v.as_str();
                PrettyTree::key_value(k, v)
            })
            .chain({
                self.children.iter().map(|x| x.to_pretty_tree())
            })
            .collect::<Vec<_>>();
        PrettyTree::branch_of(tag, xs)
    }
}

impl ToPrettyTree for Fragment {
    fn to_pretty_tree(&self) -> PrettyTree {
        if self.is_empty() {
            return PrettyTree::Empty
        }
        if self.len() == 1 {
            return self.get(0).unwrap().to_pretty_tree()
        }
        PrettyTree::fragment(self.as_node_slice())
    }
}

impl ToPrettyTree for AttributeMap {
    fn to_pretty_tree(&self) -> PrettyTree {
        let xs = self
            .iter()
            .map(|(k, v)| {
                let k = k.as_str();
                let v = v.as_str();
                PrettyTree::key_value(k, v)
            })
            .collect::<Vec<_>>();
        PrettyTree::fragment(xs)
    }
}


impl Node {
    pub fn eprintln_debug_tree(&self) {
        let s = pretty_tree::FormatterStyle::default().use_color(true);
        let f = pretty_tree::Formatter::new(s);
        eprintln!("{}", self.to_pretty_tree().normalized().format(&f))
    }
}
impl Element {
    pub fn eprintln_debug_tree(&self) {
        let s = pretty_tree::FormatterStyle::default().use_color(true);
        let f = pretty_tree::Formatter::new(s);
        eprintln!("{}", self.to_pretty_tree().normalized().format(&f))
    }
}
impl Fragment {
    pub fn eprintln_debug_tree(&self) {
        let s = pretty_tree::FormatterStyle::default().use_color(true);
        let f = pretty_tree::Formatter::new(s);
        eprintln!("{}", self.to_pretty_tree().normalized().format(&f))
    }
}
impl AttributeMap {
    pub fn eprintln_debug_tree(&self) {
        let s = pretty_tree::FormatterStyle::default().use_color(true);
        let f = pretty_tree::Formatter::new(s);
        eprintln!("{}", self.to_pretty_tree().normalized().format(&f))
    }
}
