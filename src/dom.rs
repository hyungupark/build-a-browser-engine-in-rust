//! Basic DOM data structures.

use std::collections::HashMap;


/*
    The DOM

    The DOM is a tree of nodes. A node has zero or more children.
 */
pub struct Node {
    pub node_type: NodeType, // data specific to each node type
    pub children: Vec<Node>, // data common to all nodes
}


/*
    There are several [node_types](https://dom.spec.whatwg.org/#dom-node-nodetype),
    but for now we will ignore most of them and say that a node is either an Element or a Text node.
    In a language with inheritance these would be subtypes of Node.
    In Rust, they can be an enum (Rust's keyword for a "tagged union" or "sum type").

    e.g.
        NodeType {
            Element(ElementData),
            Text("Hello, World!"),
        }
 */
pub enum NodeType {
    Element(Element),
    Text(String),
}


/*
    An element includes a tag name and any number of attributes, which can be stored as a map from
    names to values. This engine doesn't support namespaces, so it just stores tag and attribute names
    as simple strings.

    e.g.
        Element {
            tag_name: "p",
            attributes: AttributeMap,
        }
 */
pub struct Element {
    pub tag_name: String,
    pub attributes: AttributeMap,
}

/*
    e.g.
        { "id": "...", "class": "...", "style": "..." }
 */
type AttributeMap = HashMap<String, String>;


/*
    Finally, some constructor functions to make it easy to create new nodes.
 */

// Constructor functions for convenience

pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        node_type: NodeType::Text(data),
    }
}

pub fn element(tag_name: String, attributes: AttributeMap, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(Element { tag_name, attributes }),
    }
}
