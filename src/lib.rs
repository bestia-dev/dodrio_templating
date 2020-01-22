//! **dodrio_templating**  

//region: extern and use statements
mod fetchmod;
mod routermod;
mod fncallermod;
mod htmltemplatemod;

//use console_error_panic_hook;
use unwrap::unwrap;
use wasm_bindgen::prelude::*;
use dodrio::{Node, Render, RenderContext};
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
    ///the html template for this page
    pub html_template: String,
    ///the # local route for this page
    pub local_route: String,
    /// some test data. All the data must be in this struct.
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
        test1: "tst1".to_owned(),
    };

    // Mount the component to the `<body>`.
    let vdom = dodrio::Vdom::new(&div_for_virtual_dom, rrc);

    // Start the URL router.
    routermod::start_router(vdom.weak());

    // Run the component forever.
    vdom.forget();
}

impl Render for RootRenderingComponent {
    /// The `Render` implementation.  
    /// It is called when scheduled to render the vdom.  
    fn render<'a>(&self, cx: &mut RenderContext<'a>) -> Node<'a> {
        let bump = cx.bump;
        //return
        // html fragment from html_template defined in # local_route
        if self.html_template.is_empty() {
            htmltemplatemod::empty_div(cx)
        } else {
            htmltemplatemod::get_root_element(&self, bump).unwrap()
        }
    }
}
