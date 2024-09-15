//! Basic CSS block layout.

use crate::style;

/*
    The layout module takes the style tree and translates it into a bunch of rectangles in
    a two-dimensional space.

    The layout module's input is the style tree, and its output is yet another tree, the
    "layout tree".
 */

/*
    Layout is all about "boxes". A box is a rectangular section of a web page. It has a
    width, a height, and a position on the page. This rectangle is called the "content area"
    because it's where the box's content is drawn. The content may be text, image, video,
    or other boxes.

    A box may also have padding, borders, and margins surrounding its content area. The CSS
    spec has a [diagram](https://www.w3.org/TR/CSS2/box.html#box-dimensions) showing how
    all these layers fit together.

    The engine stores a box's content area and surrounding areas in the following structure.

    Rust note: `f32` is a 32-bit floating point type.
 */

// CSS box model. All sizes are in px.

/// Position of the content area relative to the document origin:
struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

/// Surrounding edges:
struct EdgeSizes {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

struct Dimensions {
    content: Rect,
    padding: EdgeSizes,
    border: EdgeSizes,
    margin: EdgeSizes,
}


/*
    Block and Inline Layout

    The CSS display property determines which type of box an element generates. CSS
    defines several box types, each with its own layout rules. Here only going to talk
    about two of them: "block" and "inline".

    I'll use this bit of pseudo-HTML to illustrate the difference:
        <container>
            <a></a>
            <b></b>
            <c></c>
            <d></d>
        </container>
 */

/// A node in the layout tree.
/*
    The Layout Tree

    The layout tree is a collection of boxes. A box has dimensions, and it may contain
    child boxes.
 */
pub struct LayoutBox<'a> {
    pub dimensions: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

/*
    A box can be a block node, an inline node, or an anonymous block box. (This will
    need to change when I implement text layout, because line wrapping can cause a
    single inline node to split into multiple boxes. But it will do for now.)
 */
enum BoxType<'a> {
    BlockNode(&'a style::StyledNode<'a>),
    InlineNode(&'a style::StyledNode<'a>),
    AnonymousBlock,
}

impl<'a> LayoutBox<'a> {
    fn new(box_type: BoxType<'a>) -> LayoutBox {
        LayoutBox {
            box_type,
            dimensions: Default::default(), // initially set all fields to 0.0
            children: Vec::new(),
        }
    }

    fn get_styled_node(&self) -> &'a style::StyledNode<'a> {
        match self.box_type {
            BoxType::BlockNode(node) | BoxType::InlineNode(node) => node,
            BoxType::AnonymousBlock => panic!("Anonymous block box has no styled node"),
        }
    }
}