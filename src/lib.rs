//! **dodrio_templating**  

//region: Clippy
#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    //variable shadowing is idiomatic to Rust, but unnatural to me.
    clippy::shadow_reuse,
    clippy::shadow_same,
    clippy::shadow_unrelated,

)]
#![allow(
    //library from dependencies have this clippy warnings. Not my code.
    //Why is this bad: It will be more difficult for users to discover the purpose of the crate, 
    //and key information related to it.
    clippy::cargo_common_metadata,
    //Why is this bad : This bloats the size of targets, and can lead to confusing error messages when 
    //structs or traits are used interchangeably between different versions of a crate.
    clippy::multiple_crate_versions,
    //Why is this bad : As the edition guide says, it is highly unlikely that you work with any possible 
    //version of your dependency, and wildcard dependencies would cause unnecessary 
    //breakage in the ecosystem.
    clippy::wildcard_dependencies,
    //Rust is more idiomatic without return statement
    //Why is this bad : Actually omitting the return keyword is idiomatic Rust code. 
    //Programmers coming from other languages might prefer the expressiveness of return. 
    //It’s possible to miss the last returning statement because the only difference 
    //is a missing ;. Especially in bigger code with multiple return paths having a 
    //return keyword makes it easier to find the corresponding statements.
    clippy::implicit_return,
    //I have private function inside a function. Self does not work there.
    //Why is this bad: Unnecessary repetition. Mixed use of Self and struct name feels inconsistent.
    clippy::use_self,
    //Cannot add #[inline] to the start function with #[wasm_bindgen(start)]
    //because then wasm-pack build --target web returns an error: export run not found 
    //Why is this bad: In general, it is not. Functions can be inlined across crates when that’s profitable 
    //as long as any form of LTO is used. When LTO is disabled, functions that are not #[inline] 
    //cannot be inlined across crates. Certain types of crates might intend for most of the 
    //methods in their public API to be able to be inlined across crates even when LTO is disabled. 
    //For these types of crates, enabling this lint might make sense. It allows the crate to 
    //require all exported methods to be #[inline] by default, and then opt out for specific 
    //methods where this might not make sense.
    clippy::missing_inline_in_public_items,
    //Why is this bad: This is only checked against overflow in debug builds. In some applications one wants explicitly checked, wrapping or saturating arithmetic.
    //clippy::integer_arithmetic,
    //Why is this bad: For some embedded systems or kernel development, it can be useful to rule out floating-point numbers.
    clippy::float_arithmetic,
    //Why is this bad : Doc is good. rustc has a MISSING_DOCS allowed-by-default lint for public members, but has no way to enforce documentation of private items. This lint fixes that.
    clippy::doc_markdown,
    //Why is this bad : Splitting the implementation of a type makes the code harder to navigate.
    clippy::multiple_inherent_impl,

    clippy::missing_docs_in_private_items,
)]
//endregion

//region: extern and use statements
mod fetchmod;
mod microxmlparser;

//use console_error_panic_hook;
use dodrio::bumpalo::{self, Bump};
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use dodrio::{Node, Render, RenderContext};
//use wasm_bindgen::JsCast; //don't remove this. It is needed for dyn_into.
use web_sys::{console, Window};
use wasm_bindgen_futures::spawn_local;
//endregion

use quick_xml::Reader;
use quick_xml::events::Event;
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
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();

    // Get the document's `<body>`.
    let window: Window = unwrap!(web_sys::window());
    let document = unwrap!(window.document());
    let div_for_virtual_dom = unwrap!(document.get_element_by_id("div_for_virtual_dom"));
    //let location: Location = window.location();
    //window.
    //let pathname: String = unwrap!(location.pathname());
    //pathname.
    //log1(&pathname);

    //test my parser
    microxmlparser::test_my_parser();

    // Construct a new rendering component.
    let rrc = RootRenderingComponent::new();

    // Mount the component to the `<body>`.
    let vdom = dodrio::Vdom::new(&div_for_virtual_dom, rrc);

    // Run the component forever.
    vdom.forget();
}

impl RootRenderingComponent {
    fn new() -> RootRenderingComponent {
        //return
        RootRenderingComponent {
            respbody: "".to_owned(),
        }
    }
}

// The `Render` implementation. It is called for every Dodrio animation frame to render the vdom.
impl Render for RootRenderingComponent {
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        use dodrio::builder::*;
        let bump = cx.bump;
        //return
        div(&cx)
            .children([
                button(&cx)
                    .on("click", |_root, vdom, _event| {
                        let v2 = vdom;
                        //async executor spawn_local is the recommanded for wasm
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
                        //async executor spawn_local is the recommanded for wasm
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
                        //async executor spawn_local is the recommanded for wasm
                        let url = "example/t3.html".to_owned();
                        log1(&url);
                        //this will change the rrc.respbody eventually
                        spawn_local(async_fetch_and_rrcwrite(url, v2));
                    })
                    .children([text("fetch 3")])
                    .finish(),
                parse_xml_create_node(&self.respbody, bump),
            ])
            .finish()
    }
}

pub fn parse_xml_create_node<'a>(xml_html: &str, bump: &'a Bump) -> Node<'a> {
    use dodrio::builder::*;

    struct ChildParent {
        id: usize,
        parent_id: usize,
    }

    //parse to nodes (element,text) and attributes
    let mut id_num = 0;
    //create a dummy node
    let name1 = bumpalo::format!(in bump, "{}",
 "dummy")
    .into_bump_str();
    let eee = ElementBuilder::new(bump, name1);
    let mut root_node = eee.finish();
    //
    let mut vec_child_parent: Vec<ChildParent> = Vec::new();
    let mut vec_elem = Vec::new();
    let mut vec_children = Vec::new();
    let mut reader = Reader::from_str(xml_html);
    reader.trim_text(true);

    // The `Reader` does not implement `Iterator` because it outputs borrowed data (`Cow`s)
    loop {
        let mut buf = Vec::new();
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref e)) => {
                #[allow(clippy::integer_arithmetic)]
                {
                    id_num += 1;
                }
                //maybe it must be inside the bump
                let name = bumpalo::format!(in bump, "{}",
            unwrap!(str::from_utf8(e.name())))
                .into_bump_str();
                //log1(&format!("START id_num {} name {}", id_num, name));
                let mut eee = ElementBuilder::new(bump, name);
                for attx in e.attributes() {
                    let att = unwrap!(attx);
                    let key = bumpalo::format!(in bump, "{}",
            unwrap!(str::from_utf8(att.key)))
                    .into_bump_str();
                    let value = bumpalo::format!(in bump, "{}",
            unwrap!(str::from_utf8(&att.value)))
                    .into_bump_str();
                    //log1(&format!("key {} value {}", &key, &value));
                    //wow! because of the dot concatenation fancy programming style
                    //the variable is moved and then returned. Terrible for non dot concat style.
                    eee = eee.attr(key, value);
                }
                //this must be a parent, because the sibling is already finished and poped
                let parent_id = if vec_child_parent.is_empty() {
                    0
                } else {
                    unwrap!(vec_child_parent.last()).id
                };
                //log1(&format!("parent_id {}", &parent_id));
                vec_elem.push(eee);
                vec_child_parent.push(ChildParent {
                    id: id_num,
                    parent_id,
                });
                vec_children.push(bumpalo::collections::Vec::new_in(bump));
            }
            Ok(Event::End(ref _e)) => {
                //let name = str::from_utf8(&e.name()).unwrap();
                let mut epop = unwrap!(vec_elem.pop());
                let ch = unwrap!(vec_children.pop());
                //add the children
                epop = epop.children(ch);
                let node = epop.finish();
                //log1(&format!("END {:?}", &node));
                //now add me to my parent
                let ch_par = unwrap!(vec_child_parent.pop());
                if ch_par.parent_id == 0 {
                    //this is the end of the root element
                    root_node = node;
                    break;
                } else {
                    //fond parent (only one) and add it to him
                    for (i, x) in vec_child_parent.iter().enumerate() {
                        if x.id == ch_par.parent_id {
                            //the child function moves the elem out of the vector.
                            //that is not allowed. I must found a workaround
                            //vec_elem[i].child(node);
                            unwrap!(vec_children.get_mut(i)).push(node);
                            //log1(&format!("added me to my parent{}", ""));
                            break;
                        }
                    }
                }
            }
            Ok(Event::Text(e)) => {
                let txt = unwrap!(e.unescape_and_decode(&reader));
                //text is also a children, but it cannot have children
                let txt = bumpalo::format!(in bump, "{}",txt).into_bump_str();
                //log1(&format!("TEXT {:?}", &txt));
                //add me to my parent
                let last_el = unwrap!(vec_children.last_mut());
                last_el.push(text(txt));
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
        // if we don't keep a borrow elsewhere, we can clear the buffer to keep memory usage low
        buf.clear();
    }

    //log1(&format!("{:?}", &root_node));

    //return
    root_node
}

/// the async fn for executor spawn_local
/// with update the value in struct rrc with await
pub async fn async_fetch_and_rrcwrite(url: String, vdom: dodrio::VdomWeak) {
    let text_jsvalue: JsValue = fetchmod::async_spwloc_fetch(url).await;
    let txt_str: String = unwrap!(JsValue::as_string(&text_jsvalue));
    //update values in rrc is async
    //I can use a fn call or an async block
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
