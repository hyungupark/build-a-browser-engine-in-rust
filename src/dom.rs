/*
    The DOM

    The DOM is a tree of nodes. A node has zero or more children.
 */
use std::collections::HashMap;

struct Node {
    // data common to all nodes:
    children: Vec<Node>,

    // data specific to each node type:
    node_type: NodeType,
}


/*
    There are several [node_types](https://dom.spec.whatwg.org/#dom-node-nodetype),
    but for now we will ignore most of them and say that a node is either an Element or a Text node.
    In a language with inheritance these would be subtypes of Node.
    In Rust, they can be an enum (Rust's keyword for a "tagged union" or "sum type").
 */

enum NodeType {
    Text(String),
    Element(ElementData),
}


/*
    An element includes a tag name and any number of attributes, which can be stored as a map from
    names to values. This engine doesn't support namespaces, so it just stores tag and attribute names
    as simple strings.
 */

struct ElementData {
    tag_name: String,
    attributes: AttributeMap,
}

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
        node_type: NodeType::Element(ElementData { tag_name, attributes }),
    }
}