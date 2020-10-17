extern crate arts;

use arts::parse::html;

fn main() {
    html::parse("<html>これはhtmlです</html>");
}