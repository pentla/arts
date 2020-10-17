use std::rc::Rc;
use html5ever::{parse_document};
use html5ever::driver::ParseOpts;
use markup5ever_rcdom::{RcDom, Node};
use html5ever::tendril::stream::TendrilSink;

pub fn parse(html: &str) -> Rc<Node> {
    let parser = parse_document(RcDom::default(), ParseOpts::default());
    let dom = parser.one(html);
    println!("{:?}", dom.document);

    /*
        Node { data: Document, children: RefCell { value: [Node { data: Element { name: QualName { prefix: None, ns: Atom('http://www.w3.org/1999/xhtml' type=static), 
        local: Atom('html' type=static) }, attrs: RefCell { value: [] }, template_contents: None, mathml_annotation_xml_integration_point: false }, 
        children: RefCell { value: [Node { data: Element { name: QualName { prefix: None, ns: Atom('http://www.w3.org/1999/xhtml' type=static), 
        local: Atom('head' type=static) }, attrs: RefCell { value: [] }, template_contents: None, mathml_annotation_xml_integration_point: false }, children: RefCell { value: [] } }, 
        Node { data: Element { name: QualName { prefix: None, ns: Atom('http://www.w3.org/1999/xhtml' type=static), local: Atom('body' type=static) }, attrs: RefCell { value: [] }, 
        template_contents: None, mathml_annotation_xml_integration_point: false }, children: RefCell { value: [Node { data: Text { contents: RefCell { value: Tendril<UTF8>(shared: "これはhtmlです") } }, children: RefCell { value: [] } }] } }] } }] } }
    */

    if !dom.errors.is_empty() {
        println!("Parse Errors");
        for err in dom.errors.iter() {
            println!("  {}", err);
        }
    }
    dom.document
}