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

## reader for microxml

There are many xml parsers/readers/tokenizers/lexers around, but I need something very small and simple for my templates.  
I found the existence of a standard (or proposal) for *MicroXml* - dramatically simpler then the full Xml standard. Perfect for my use-case. I published a crate:  
<https://crates.io/crates/reader_for_microxml>  

## ChangeLog

2020-01-10 I succeeded to read the html and create the dodrio vdom.  
2020-01-20 MicroXml parsing, local router  
