//! **dodrio_templating**  

//region: extern and use statements
mod fetchmod;
mod htmlentitiesmod;
mod routermod;
mod fncallermod;

use reader_for_microxml::*;

//use console_error_panic_hook;
use dodrio::bumpalo::{self, Bump};
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use dodrio::{Node, Listener, Attribute, Render, RenderContext};
use dodrio::builder::*;
//use wasm_bindgen::JsCast; //don't remove this. It is needed for dyn_into.
use web_sys::{console, Window};
//endregion

use std::str;
use wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

///simple console write with a string
fn log1(x: &str) {
    console::log_1(&JsValue::from_str(x));
}

#[derive(Debug, Clone)]
pub struct RootRenderingComponent {
    pub html_template: String,
    pub local_route: String,
    pub test1: String,
}

#[wasm_bindgen(start)]
pub fn wasm_bindgen_start() {
    log1("wasm_bindgen_start");
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    // Get the document's `<body>`.
    let window: Window = unwrap!(web_sys::window());
    let document = unwrap!(window.document());
    let div_for_virtual_dom = unwrap!(document.get_element_by_id("div_for_virtual_dom"));

    // Construct a new rendering component.
    let rrc = RootRenderingComponent {
        html_template: "".to_owned(),
        local_route: "".to_owned(),
        test1: "test1".to_owned(),
    };

    // Mount the component to the `<body>`.
    let vdom = dodrio::Vdom::new(&div_for_virtual_dom, rrc);

    // Start the URL router.
    routermod::start(vdom.weak());

    // Run the component forever.
    vdom.forget();
}

impl Render for RootRenderingComponent {
    /// The `Render` implementation.  
    /// It is called when scheduled to render the vdom.  
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        let bump = cx.bump;
        //return
        // html fragment from file defined in query
        if self.html_template.is_empty() {
            div(&cx).finish()
        } else {
            get_root_element(&self, bump).unwrap()
        }
    }
}

/// get root element Node.  
/// I wanted to use dodrio::Node, but it has only private methods.  
/// I must use element_builder.  
fn get_root_element<'a>(rrc: &RootRenderingComponent, bump: &'a Bump) -> Result<Node<'a>, String> {
    let mut pp = ReaderForMicroXml::new(&rrc.html_template);
    let mut dom_path = Vec::new();
    let mut root_element;
    let mut html_or_svg = 0; //0-html, 1-svg
    match pp.read_event() {
        Event::StartElement(name) => {
            dom_path.push(name.to_owned());
            let name = bumpalo::format!(in bump, "{}",name).into_bump_str();
            root_element = ElementBuilder::new(bump, name);
            if name == "svg" {
                html_or_svg = 1; //svg
            }
            if html_or_svg == 1 {
                //svg elements have this namespace
                root_element = root_element.namespace(Some("http://www.w3.org/2000/svg"));
            }
            // recursive function can return error
            match fill_element_builder(rrc, &mut pp, root_element, bump, html_or_svg, &mut dom_path)
            {
                //the methods are move, so I have to return the moved value
                Ok(new_root_element) => root_element = new_root_element,
                Err(err) => {
                    return Err(err);
                }
            }
        }
        _ => {
            //return error
            return Err("Error: no root element".to_owned());
        }
    }
    //return
    Ok(root_element.finish())
}

/// Recursive function to fill the tree with a node.  
/// Moves & Returns ElementBuilder or error.  
/// I must `move` ElementBuilder because its methods are all `move`.  
/// It makes the code less readable. It is only good for chaining and type changing.  
fn fill_element_builder<'a>(
    rrc: &RootRenderingComponent,
    pp: &mut ReaderForMicroXml,
    mut element: ElementBuilder<
        'a,
        bumpalo::collections::Vec<'a, Listener<'a>>,
        bumpalo::collections::Vec<'a, Attribute<'a>>,
        bumpalo::collections::Vec<'a, Node<'a>>,
    >,
    bump: &'a Bump,
    mut html_or_svg: usize,
    dom_path: &mut Vec<String>,
) -> Result<
    ElementBuilder<
        'a,
        bumpalo::collections::Vec<'a, Listener<'a>>,
        bumpalo::collections::Vec<'a, Attribute<'a>>,
        bumpalo::collections::Vec<'a, Node<'a>>,
    >,
    String,
> {
    let mut replacement: Option<String> = None;
    loop {
        match pp.read_event() {
            Event::StartElement(name) => {
                dom_path.push(name.to_owned());
                //construct a child element and fill it (recursive)
                let name = bumpalo::format!(in bump, "{}",name).into_bump_str();
                let mut child_element = ElementBuilder::new(bump, name);
                if name == "svg" {
                    //this tagname changes to svg now
                    html_or_svg = 1; //svg
                }
                if html_or_svg == 1 {
                    //this is the
                    //svg elements have this namespace
                    child_element = child_element.namespace(Some("http://www.w3.org/2000/svg"));
                }
                if name == "foreignObject" {
                    //this tagname changes to html for children, not for this element
                    html_or_svg = 0; //html
                }
                child_element =
                    fill_element_builder(rrc, pp, child_element, bump, html_or_svg, dom_path)?;
                element = element.child(child_element.finish());
            }
            Event::Attribute(name, value) => {
                if name.starts_with("data-t-") {
                    //the rest of the name does not matter.
                    //The replacement will always be applied to the next attribute.
                    let fn_name = value;
                    let repl_txt = fncallermod::call_function(rrc, fn_name);
                    replacement = Some(repl_txt);
                } else {
                    let name = bumpalo::format!(in bump, "{}",name).into_bump_str();
                    let value2;
                    match replacement {
                        Some(repl) => {
                            value2 =
                        bumpalo::format!(in bump, "{}",htmlentitiesmod::decode_minimum_html_entities(&repl))
                            .into_bump_str();
                            //empty the replacement for the next node
                            replacement = None;
                        }
                        None => {
                            value2 =
                        bumpalo::format!(in bump, "{}",htmlentitiesmod::decode_minimum_html_entities(value))
                            .into_bump_str();
                        }
                    }
                    element = element.attr(name, value2);
                }
            }
            Event::TextNode(txt) => {
                let txt2;
                match replacement {
                    Some(repl) => {
                        txt2 =
                    bumpalo::format!(in bump, "{}",htmlentitiesmod::decode_minimum_html_entities(&repl))
                        .into_bump_str();
                        //empty the replacement for the next node
                        replacement = None;
                    }
                    None => {
                        txt2 =
                    bumpalo::format!(in bump, "{}",htmlentitiesmod::decode_minimum_html_entities(txt))
                        .into_bump_str();
                    }
                }
                // here accepts only utf-8.
                // only minimum html entities are decoded
                element = element.child(text(txt2));
            }
            Event::Comment(txt) => {
                //the main goal of comments is to change the value of the next text node
                //with the result of a function
                // it must look like <!--t=get_text-->
                if txt.starts_with("t=") {
                    let fn_name = &txt[2..];
                    let repl_txt = fncallermod::call_function(rrc, fn_name);
                    replacement = Some(repl_txt);
                }
            }
            Event::EndElement(name) => {
                let last_name = dom_path.pop().unwrap();
                //it can be also auto-closing element
                if last_name == name || name == "" {
                    return Ok(element);
                } else {
                    return Err(format!("End element not correct: {} {}", last_name, name));
                }
            }
            Event::Error(error_msg) => {
                return Err(format!("{}", error_msg));
            }
            Event::Eof => {
                return Ok(element);
            }
        }
    }
}
