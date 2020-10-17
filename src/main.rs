extern crate arts;

use arts::parse::html;
use arts::render::render::walk;

fn main() {
    let node = html::parse("<html>これはhtmlです</html>");
    walk(node);
}