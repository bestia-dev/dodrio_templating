//! fncallermod  
//! calls a function named with a string. The function returs a string to insert in the templating.  
//!

use crate::log1;
use crate::RootRenderingComponent;

pub fn call_function(rrc: &RootRenderingComponent, sx: &str) -> String {
    log1(&format!("call_function: {}", &sx));
    match sx {
        "get_text" => return get_text(rrc),
        "get_red" => return get_red(),
        "test1" => return rrc.test1.to_owned(),
        _ => {
            let x = format!("Error: Unrecognized call_function: {}", sx);
            log1(&x);
            return x;
        }
    }
}

pub fn get_text(rrc: &RootRenderingComponent) -> String {
    let gt = format!("gt: {}", &rrc.test1);
    gt
}

pub fn get_red() -> String {
    "red".to_owned()
}
