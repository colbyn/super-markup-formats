use crate::{AttributeMap, Element, Fragment, Node, TagBuf};

// ————————————————————————————————————————————————————————————————————————————
// HTML REDUCER
// ————————————————————————————————————————————————————————————————————————————

pub trait HtmlReducer {
    type Output;
    fn visit_text(&mut self, text: String) -> Self::Output;
    fn visit_fragment(&mut self, fragment: Vec<Self::Output>) -> Self::Output;
    fn visit_element(
        &mut self,
        tag: TagBuf,
        attributes: AttributeMap,
        children: Self::Output,
    ) -> Self::Output;
}

// ————————————————————————————————————————————————————————————————————————————
// IMPLEMENTATION
// ————————————————————————————————————————————————————————————————————————————

impl Node {
    fn apply_html_reducer<R: HtmlReducer>(self, reducer: &mut R) -> R::Output {
        match self {
            Self::Text(text) => reducer.visit_text(text),
            Self::Element(element) => element.apply_html_reducer(reducer),
            Self::Fragment(fragment) => fragment.apply_html_reducer(reducer),
        }
    }
}

impl Element {
    fn apply_html_reducer<R: HtmlReducer>(self, reducer: &mut R) -> R::Output {
        let Element { tag, attributes, children } = self;
        let children = children.apply_html_reducer(reducer);
        reducer.visit_element(tag, attributes, children)
    }
}

impl Fragment {
    fn apply_html_reducer<R: HtmlReducer>(self, reducer: &mut R) -> R::Output {
        let nodes = self
            .into_iter()
            .map(|x| x.apply_html_reducer(reducer))
            .collect::<Vec<_>>();
        reducer.visit_fragment(nodes)
    }
}

// ————————————————————————————————————————————————————————————————————————————
// ENTRYPOINT
// ————————————————————————————————————————————————————————————————————————————

pub fn apply_html_reducer<R: HtmlReducer>(node: Node, reducer: &mut R) -> R::Output {
    node.apply_html_reducer(reducer)
}
