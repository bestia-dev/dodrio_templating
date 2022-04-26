# dodrio_templating

**description**  
***version: 0.2.0  date: 2020-01-02 author: [bestia.dev](https://bestia.dev) repository: [GitHub](https://github.com/bestia-dev/dodrio_templating)***  

[![Hits](https://hits.seeyoufarm.com/api/count/incr/badge.svg?url=https%3A%2F%2Fgithub.com%2Fbestia-dev%2Fdodrio_templating&count_bg=%2379C83D&title_bg=%23555555&icon=&icon_color=%23E7E7E7&title=hits&edge_flat=false)](https://hits.seeyoufarm.com)

I don't know. There is so much confusion and choice around javascript/wasm and html templating.  
I tried to use typed-html. It is macro based. You write a jsx like syntax inside rust functions.  
It looks really nice to code html directly inside rust.  
But for every small change I have to recompile the code. And that is not very fast in Rust.  
It looks also that intellisense (RLS or rust-analyzer) have a hard time to understand macros like this.  
When something goes wrong, the error msg is just terrible.  
There are a lot of other templating libraries. Maybe I should use one.  
But I don't know how to embed that in dodrio.  
So I am thinking to try and make something really simple. To learn more rusty stuff.  

## Separate two step development time and runtime

In development time I want the possibility to quickly modify the html and css code.  
Then just refresh the page (partially refresh) and the result should be visible.  
The process of rust compiling and then starting the app from start is too slow.  
When dealing with beauty there are a lot of small iterations. Much more that with programing data stuff.  
The html and css files will be on the server. It is easy to modify them there. The app will always download the new file and then interpret it to dodrio vdom.  
This will be probably very slow for the runtime.  
For the same html_template I plan to generate a rs file with rust code, that can be compiled and embedded inside the app. For this I will use the build.rs functionality of the rust compiler. I will build one file with rust code for every html. And then this files will be integrated into the code. And after that they will be compiled. Probably this will work much faster in runtime.  

## templating

I want that a graphic designer to make a complete well-formed and beautiful html5/svg page and saves it. So I will be sure it looks great on a smartphone. If I do the design part it would look boring. So I want to split the graphic design part and the programming part.  
Than I intend to add invisible attributes and comments into html/svg to make it possible later to dynamically change the content. That way I preserve the static file that looks good.  

### attributes

For attributes I will add a custom attribute like this `data-t-01="function_name"`.  
It must be positioned exactly before the actual attribute - because of the parsing.  
The `data-` means custom attributes for html5.  
The `-t-` is for now to distinguish attributes for templating.  
The last part of the name is there only to not repeat the same attribute name. It is never used for anything else.  
The function must return a string for the value of the attribute.  

### text node

For the text node I will add a html comment before the text node like this `<!--t=get_text()-->`.  
The name `t=` is to distinguish the function for templating.  
The function must return a string to replace the original text node.  
Example:  

```html
<text data-t-01="get_color" fill="white">
<!--t=get_text-->hello
</text>
```

### TODO: node

I plan to add functions that replace a whole dom node.  

## reader for microxml

There are many xml parsers/readers/tokenizers/lexers around, but I need something very small and simple for my templates.  
I found the existence of a standard (or proposal) for *MicroXml* - dramatically simpler then the full Xml standard. Perfect for my use-case. I published this crate:  
<https://crates.io/crates/reader_for_microxml>  

## encoding

Strings in html are encoded with html entities. There is a lot of them: named, decimal and hex.  
But only 5 are required to not mess with the html code: " ' & < >.  
These 5 I decode from the html template. All other characters must be already in utf-8 without html entities.  

## svg

Svg elements are slightly different from html elements.  
Svg must be well-formed xml (more-or-less).  
And it has a terrible namespace !! `http://www.w3.org/2000/svg` that MUST be used when creating the HtmlElements.  
Names of elements and attributes are case-sensitive !!! and are not all lowercase, like html.  
I will mix html and svg as much as possible.
So I need to know where am I inside a tree: html or svg to add that namespace.  

## font size

I lost badly the battle for font-size in html. I didn't find a way to force android chrome to make fonts the size I wanted. If the user used the accessibility option for bigger fonts there was no way I can override that.  
It just ruined the layout od the game.  
Sure it is ok for html pages that show news or forms to change the font-size. But please not for games. Total defeat.  
So I retreated from html font-size and choose SVG to make my font-size harder to ruin from the user. But also here I didn't find it very easy.  
There is no point for the game to be smaller than 300px or larger than 600px width.  
In between the font should be relative to the width of the body/viewport/device.  
The unit vw is created exactly for that purpose. And it DOES NOT work properly.  
There are small differences that I had to correct using a strange hack:  
`font-size: calc(6vw - 2px);`  
Horrible, but that is not all. The font size is different for every font-family.  

## font family

Fonts are completely crazy on the web.  
Every platform have different fonts even if they call them the same.  
The only way to have really equal fonts is download them.  
Google fonts are the best choice I think. I choose `Roboto`.  

## local router

Applications are divided into pages. Every page has a distinct html_template.  
The page is changed with the hash local route # like:  
`//myurl/#mylocal_route`  
The routermod listens to the `ChangeHash` event and calls the appropriate html_template.  
Templates are in the folder /html_templates/. The name of the file is the same as the hash local route with added extension `.html`.  

## fncaller

The string I get inside html_templates are the names of functions to be called. There is no easy way to dynamically call a functions by its name in Rust. So I created a functions that compares the string and calls the appropriate function in the `fncallermod` module.

## data

All the data for the applications must be inside the rrc RootRenderingComponent struct. There the data can be changed and read for use everywhere.  

## build and run

In the dodrio crate there is a bug with className for svg elements. It will error if class is used on any svg element. I wrote to the author and I hope it will be corrected soon. This is why I use path to a local copy of dodrio in cargo.toml where I corrected the bug.

```bash
clear; cargo make release
```

In browser open  
<http://127.0.0.1:8186>  

## cargo crev reviews and advisory

It is recommended to always use [cargo-crev](https://github.com/crev-dev/cargo-crev)  
to verify the trustworthiness of each of your dependencies.  
Please, spread this info.  
On the web use this url to read crate reviews. Example:  
<https://bestia.dev/cargo_crev_web/query/num-traits>  

