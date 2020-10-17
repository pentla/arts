use std::rc::Rc;
use markup5ever_rcdom::{Node, NodeData, Handle};

pub fn walk(node: Rc<Node>) {
    walk_iter(&node.children.borrow()[0]);
}

fn walk_iter(node: &Handle) {
    match node.data {
        NodeData::Text {ref contents} => {

        },
        NodeData::Document {} => {

        },
        NodeData::Doctype { ref name, ref public_id, ref system_id } => {},
        NodeData::Comment { ref contents } => {},
        NodeData::ProcessingInstruction { ref target, ref contents} => {},
        NodeData::Element { ref name, ref attrs, ref template_contents, ref mathml_annotation_xml_integration_point} => {}
    };
    for child in node.children.borrow().iter() {
        walk_iter(child)
    }
}