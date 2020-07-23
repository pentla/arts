fn main() {
    println!("Hello, world!");
    let html = "<html><body>hello</body></html>";
    println!("{}, {}", html.as_bytes().len(), html.as_bytes()[3]);

}

// fn parse_nodes(html_str: &str) {
//     let nodes: Vec<i32> = Vec::new();
//     let html = html_str.as_bytes();
//     let len = html.len();
//     loop {
//         let mut index = 0;
//         if len <= index {
//             break;
//         }
//         if html_chars[index] == "<" && html[index+1] == "/" {

//         }
//     }
// }