//! fncallermod  

use crate::log1;
use crate::RootRenderingComponent;

/// html_templating functions that return a String
pub fn call_function_string(rrc: &RootRenderingComponent, sx: &str) -> String {
    log1(&format!("call_function_string: {}", &sx));
    match sx {
        "first_text" => return "this is first text replaced".to_owned(),
        "first_attr" => return "this is first attr replaces".to_owned(),
        "get_text" => return get_text(rrc),
        "get_red" => return get_red(),
        "test1" => return rrc.test1.to_owned(),
        _ => {
            let x = format!("Error: Unrecognized call_function_string: {}", sx);
            log1(&x);
            return x;
        }
    }
}

//TODO: html_templating functions that return a Node

pub fn get_text(rrc: &RootRenderingComponent) -> String {
    let gt = format!("get_text from rrc: {}", &rrc.test1);
    gt
}

pub fn get_red() -> String {
    "red".to_owned()
}
