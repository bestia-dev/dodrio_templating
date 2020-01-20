//! **dodrio_templating**  

//region: extern and use statements
mod fetchmod;
mod htmlentitiesmod;
mod routermod;

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
    pub respbody: String,
    pub local_route: String,
}

#[wasm_bindgen(start)]
pub fn wasm_bindgen_start() {
    log1("start");
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    // Get the document's `<body>`.
    let window: Window = unwrap!(web_sys::window());
    let document = unwrap!(window.document());
    let div_for_virtual_dom = unwrap!(document.get_element_by_id("div_for_virtual_dom"));

    // Construct a new rendering component.
    let rrc = RootRenderingComponent {
        respbody: "".to_owned(),
        local_route: "".to_owned(),
    };

    // Mount the component to the `<body>`.
    let vdom = dodrio::Vdom::new(&div_for_virtual_dom, rrc);

    // Start the URL router.
    routermod::start(vdom.weak());

    // Run the component forever.
    vdom.forget();
}

pub fn hash_change() {
    // TODO: how to get to vdom or to rrc ?
}

impl Render for RootRenderingComponent {
    /// The `Render` implementation.  
    /// It is called when scheduled to render the vdom.  
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        let bump = cx.bump;
        //return
        div(&cx)
            .children([{
                // html fragment from file defined in query
                if self.respbody.is_empty() {
                    div(&cx).finish()
                } else {
                    get_root_element(&self.respbody, bump).unwrap()
                }
            }])
            .finish()
    }
}

/// get root element Node.  
/// I wanted to use dodrio::Node, but it has only private methods.  
/// I must use element_builder.  
fn get_root_element<'a>(input: &str, bump: &'a Bump) -> Result<Node<'a>, String> {
    log1("get_root_element");
    let mut pp = ReaderForMicroXml::new(input);

    let mut root_element;
    match pp.read_event() {
        Event::StartElement(name) => {
            let name = bumpalo::format!(in bump, "{}",name).into_bump_str();
            // log1(&format!("START id_num {} name {}", id_num, name));
            root_element = ElementBuilder::new(bump, name);
            // recursive function can return error
            match fill_element_builder(&mut pp, root_element, bump) {
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
    pp: &mut ReaderForMicroXml,
    mut element: ElementBuilder<
        'a,
        bumpalo::collections::Vec<'a, Listener<'a>>,
        bumpalo::collections::Vec<'a, Attribute<'a>>,
        bumpalo::collections::Vec<'a, Node<'a>>,
    >,
    bump: &'a Bump,
) -> Result<
    ElementBuilder<
        'a,
        bumpalo::collections::Vec<'a, Listener<'a>>,
        bumpalo::collections::Vec<'a, Attribute<'a>>,
        bumpalo::collections::Vec<'a, Node<'a>>,
    >,
    String,
> {
    loop {
        match pp.read_event() {
            Event::StartElement(name) => {
                //construct a child element and fill it (recursive)
                let name = bumpalo::format!(in bump, "{}",name).into_bump_str();
                let mut child_element = ElementBuilder::new(bump, name);
                child_element = fill_element_builder(pp, child_element, bump)?;
                element = element.child(child_element.finish());
            }
            Event::Attribute(name, value) => {
                let name = bumpalo::format!(in bump, "{}",name).into_bump_str();
                let value = bumpalo::format!(in bump, "{}",value).into_bump_str();
                element = element.attr(name, value);
            }
            Event::TextNode(txt) => {
                let txt =
                    bumpalo::format!(in bump, "{}",htmlentitiesmod::decode_minimum_html_entities(txt))
                        .into_bump_str();
                // here accepts only utf-8.
                // only minimum html entities are decoded
                element = element.child(text(txt));
            }
            Event::EndElement(_name) => {
                // TODO: test if the element name is correct
                return Ok(element);
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
