// https://www.tutorialspoint.com/html5/html5_entities.htm
// Characters Entities in HTML5
// HTML5 processors must support the five special characters: " ' & < >
// I will ignore all others for now to keep it simple

pub fn decode_minimum_html_entities(input: &str) -> String {
    //I don't know how slow is replace, but I have really small texts.
    let entity_symbols = vec!["\"", "'", "&", "<", ">"];
    let entity_names = vec!["&quot;", "&apos;", "&amp;", "&lt;", "&gt;"];
    let mut output = input.to_owned();
    for i in 0..entity_symbols.len() {
        output = output.replace(entity_names[i], entity_symbols[i])
    }
    //return
    output
}
