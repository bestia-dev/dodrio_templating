# dodrio_templating

| *Things are changing fast. This is the situation on 2020-01-09. LucianoBestia*  

I don't know. There is so much confusion and choice around javascript/wasm and html templating.  
I tried to use typed-html. It is macro based. You write a jsx like syntax inside rust functions.  
It looks really nice to code html directly inside rust.  
But for every small change I have to recompile the code. And that is not very fast in Rust.  
It looks like the intellisense (RLS or rust-analyzer) have a hard time to understand macros like this.  
When something goes wrong, the error msg is just terrible.

There are a lot of other templating libraries. Maybe I should use one.  
But I don't know how to embed that in dodrio.  
So I am thinking to try and make something really simple.  

## Separate two step development time and runtime

In development time I want the possibility to quickly modify the html and css code.  
Then just refresh the page (partially refresh) and the result should be visible.  
The process of rust compiling and then starting the app from start is too slow.  
When dealing with beauty there are a lot of small iterations.  
The html and css files will be on the server. There is easy to modify them.  
The app will always download the new file and then interpret it to dodrio vdom.  

But in runtime this is slow.  
The same html file is possible to generate a file with rust code, that can be compiled and embedded inside the app. For this I will use the build.rs functionality of the rust compiler. I will build one file with rust code for every html. And then this files will be integrated into the code. And after that they will be compiled.  

## templating

I want that a graphinc designer to make a complete well-formed and beautiful html5/svg page and saves it. So I will be sure it will look great on a smartphone. If I do the design part it would look boring. So I want to split the graphical design part and the programming part.  
Than I intend to add invisible attributes and comments into html/svg to make it possible later to dynamically change the content. That way I preserve the static file that looks good.  

### attributes

For attributes I will add a custom attribute like this `data-attrname-t="function_name"`.  
It must be positioned exactly before the actual attribute - because of the parsing.  
The `data-` means custom attributes for html5.  
The mid part of the name is the original attribute name to change.  
The `-t` is for now to distinguish attributes for templating.  
The function must return a string for the value of the attribute.  

### text

For the text node I will add a html comment before the node like this `<!--t=get_text()-->`.  
The name `t=` is to distinguish the function for templating.  
The function must return a string to replace the original text node.  

```html
<text data-t-01="get_color" fill="white">
<!--t=get_text-->hello
</text>
```

## reader for microxml

There are many xml parsers/readers/tokenizers/lexers around, but I need something very small and simple for my templates.  
I found the existence of a standard (or proposal) for *MicroXml* - dramatically simpler then the full Xml standard. Perfect for my use-case. I published a crate:  
<https://crates.io/crates/reader_for_microxml>  

## encoding

String in html are encoded with html entities. There is a lot of them: named, decimal and hex.  
But finally only 5 are required to not mess with the html code: " ' & < >.
These 5 I decode from the html template. All other must be already in utf-8 without html entities.  

## svg

Svg elements are slightly different from html elements.  
Svg must be well-formed xml (more-or-less).  
And it has a terrible namespace !! `http://www.w3.org/2000/svg` that MUST be used when creating the HtmlElements.  
Names of elements and attributes are case-sensitive !!! and are not all lowercase, like html.  
I will mix html and svg as much as possible.
So I need to know where am I inside a tree: html or svg to add that namespace.  

## Troubles

I want to use svg. But it looks like dodrio can't deal with that.  
If I use the same html statically, the browser renders it fine.  
The same with dodrio is just nothing.  

## ChangeLog

2020-01-10 I succeeded to read the html and create the dodrio vdom.  
2020-01-20 MicroXml parsing, local router  
2021-01-21 templating, replace variables  
