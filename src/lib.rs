//! **dodrio_templating**  

//region: extern and use statements
mod fetchmod;

use reader_for_microxml::*;

//use console_error_panic_hook;
use dodrio::bumpalo::{self, Bump};
use dodrio::VdomWeak;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use dodrio::{Node, Listener, Attribute, Render, RenderContext};
use dodrio::builder::*;
//use wasm_bindgen::JsCast; //don't remove this. It is needed for dyn_into.
use web_sys::{console, Window};
use wasm_bindgen_futures::spawn_local;
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
    //TODO: I want to get the # query in the url
    //let location: Location = window.location();
    //window.
    //let pathname: String = unwrap!(location.pathname());
    //pathname.
    //log1(&pathname);

    // Construct a new rendering component.
    let rrc = RootRenderingComponent::new();

    // Mount the component to the `<body>`.
    let vdom = dodrio::Vdom::new(&div_for_virtual_dom, rrc);

    // Run the component forever.
    vdom.forget();
}

impl RootRenderingComponent {
    /// constructor
    fn new() -> RootRenderingComponent {
        //return
        RootRenderingComponent {
            respbody: "".to_owned(),
        }
    }
}

impl Render for RootRenderingComponent {
    /// The `Render` implementation.  
    /// It is called when scheduled to render the vdom.  
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        let bump = cx.bump;
        //return
        div(&cx)
            .children([
                button(&cx)
                    .on("click", |_root, vdom, _event| {
                        let v2 = vdom;
                        //async executor spawn_local is the recommended for wasm
                        let url = "example/t1.html".to_owned();
                        log1(&url);
                        //this will change the rrc.respbody eventually
                        spawn_local(async_fetch_and_rrcwrite(url, v2));
                    })
                    .children([text("fetch 2")])
                    .finish(),
                button(&cx)
                    .on("click", |_root, vdom, _event| {
                        let v2 = vdom;
                        //async executor spawn_local is the recommended for wasm
                        let url = "example/t2.html".to_owned();
                        log1(&url);
                        //this will change the rrc.respbody eventually
                        spawn_local(async_fetch_and_rrcwrite(url, v2));
                    })
                    .children([text("fetch and write")])
                    .finish(),
                button(&cx)
                    .on("click", |_root, vdom, _event| {
                        let v2 = vdom;
                        //async executor spawn_local is the recommended for wasm
                        let url = "example/t3.html".to_owned();
                        log1(&url);
                        //this will change the rrc.respbody eventually
                        spawn_local(async_fetch_and_rrcwrite(url, v2));
                    })
                    .children([text("fetch 3")])
                    .finish(),
                {
                    // html fragment from file
                    if self.respbody.is_empty() {
                        div(&cx).finish()
                    } else {
                        get_root_element(&self.respbody, bump).unwrap()
                    }
                },
            ])
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
                let txt = bumpalo::format!(in bump, "{}",txt).into_bump_str();
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

/// the async fn for executor spawn_local
/// with update the value in struct rrc with await
pub async fn async_fetch_and_rrcwrite(url: String, vdom: VdomWeak) {
    let txt_str: String = fetchmod::async_spwloc_fetch_text(url).await;
    // update values in rrc is async.
    // I can await a fn call or an async block.
    async {
        unwrap!(
            vdom.with_component({
                move |root| {
                    let rrc = root.unwrap_mut::<RootRenderingComponent>();
                    rrc.respbody = txt_str;
                }
            })
            .await
        );
        let window = unwrap!(web_sys::window());
        let _x =
            unwrap!(window.history()).push_state_with_url(&JsValue::from_str(""), "", Some("#t1"));
        vdom.schedule_render();
    }
    .await;

    //log1("end of async_fetch_and_rrcwrite()");
}
