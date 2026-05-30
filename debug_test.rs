use vaelvet_ui::Site;
use vaelvet_ui::routes::Home;

use dioxus::prelude::*;

fn main() {
    let mut dom = VirtualDom::new(Home);
    dom.rebuild_in_place();
    let html = dioxus_ssr::render(&dom);
    
    println!("HTML OUTPUT:");
    println!("{}", html);
    println!("END HTML OUTPUT");
    
    // Check for the specific service
    if html.contains("Crisis & Counsel") {
        println!("FOUND: Crisis & Counsel");
    } else {
        println!("NOT FOUND: Crisis & Counsel");
    }
    
    if html.contains("Crisis &amp; Counsel") {
        println!("FOUND: Crisis &amp; Counsel");
    } else {
        println!("NOT FOUND: Crisis &amp; Counsel");
    }
}