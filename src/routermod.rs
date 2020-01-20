//! A simple `#`-fragment router.

use crate::RootRenderingComponent;
use crate::fetchmod;

use dodrio::VdomWeak;
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::spawn_local;
use unwrap::unwrap;

/// Start the router.
pub fn start(vdom: VdomWeak) {
    // Callback fired whenever the URL hash fragment changes. Keeps the rrc.local_route
    // in sync with the `#` fragment.
    let on_hash_change = move || {
        let window = unwrap!(web_sys::window());
        let location = window.location();
        let local_route = unwrap!(location.hash());

        wasm_bindgen_futures::spawn_local({
            let vdom = vdom.clone();
            async move {
                let _ = vdom
                    .with_component({
                        let vdom = vdom.clone();
                        move |root| {
                            let rrc = root.unwrap_mut::<RootRenderingComponent>();
                            // If the rrc local_route already matches the event's
                            // local_route, then there is nothing to do (ha). If they
                            // don't match, then we need to update the rrc' local_route
                            // and re-render.
                            if rrc.local_route != local_route {
                                rrc.local_route = local_route;
                                let url =
                                    format!("example/{}.html", rrc.local_route.replace("#", ""));
                                let v2 = vdom.clone();
                                spawn_local(async_fetch_and_rrcwrite(url, v2));
                                vdom.schedule_render();
                            }
                        }
                    })
                    .await;
            }
        });
    };

    // Call it once to handle the initial `#` fragment.
    on_hash_change();

    // Now listen for hash changes forever.
    //
    // Note that if we ever intended to unmount our app, we would want to
    // provide a method for removing this router's event listener and cleaning
    // up after ourselves.
    let on_hash_change = Closure::wrap(Box::new(on_hash_change) as Box<dyn FnMut()>);
    let window = unwrap!(web_sys::window());
    window
        .add_event_listener_with_callback("hashchange", on_hash_change.as_ref().unchecked_ref())
        .unwrap_throw();
    on_hash_change.forget();
}

/// The async fn for executor spawn_local.  
/// It updates the value in struct rrc with await.  
/// example how to use it in on_click:  
/// '''
/// .on("click", |_root, vdom, _event| {
///     let v2 = vdom;
///     //async executor spawn_local is the recommended for wasm
///     let url = "example/t1.html".to_owned();
///     //this will change the rrc.respbody eventually
///     spawn_local(async_fetch_and_rrcwrite(url, v2));
/// })
/// ```
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
        // an example how to change the local_route from code
        //let window = unwrap!(web_sys::window());
        //let _x =
        //    unwrap!(window.history()).push_state_with_url(&JsValue::from_str(""), "", Some("#t1"));
        vdom.schedule_render();
    }
    .await;

    //log1("end of async_fetch_and_rrcwrite()");
}
