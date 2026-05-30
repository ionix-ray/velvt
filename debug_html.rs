use vaelvet_ui::Site;
use vaelvet_ui::routes::Home;

use dioxus::prelude::*;

#[test]
fn debug_html_output() {
    let mut dom = VirtualDom::new(Home);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    
    println!("HTML OUTPUT:");
    println!("{}", html);
    println!("END HTML OUTPUT");
    
    // Save to file for inspection
    std::fs::write("/tmp/debug_output.html", html).expect("Failed to write file");
}