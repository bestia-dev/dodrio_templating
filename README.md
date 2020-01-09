# dodrio_templating

*Things are changing fast. This is the situation on 2020-01-09. LucianoBestia*  

I don't know. There is so much confusion and choice around javascript/wasm and html templating.  
I tried to use typed-html. It is macro based. You write a jsx like syntax inside rust functions.  
It looks really nice to code html directly inside rust.  
But for every small change I have to recompile the code. That is not very fast in Rust.  
It looks like the intellisense (RLS or rust-analyzer) have a hard time to understand macros like this.  
When something goes wrong, the error msg is just terrible.

There are a lot of other templating libraries. Maybe I should use one. But I don't know how to embed that in dodrio.  
So I am thinking to try and make something really simple.  

## 1.from html to dodrio

Dodrio build the vdom with only 2 objects: nodes and attributes.  
```rust
use dodrio::builder::*;

