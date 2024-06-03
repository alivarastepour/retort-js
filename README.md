# What is retort-js?
retort-js is a rust-based front-end library which compiles to `wasm` using `wasm-pack` and `wasm-bindgen`. It aims to provide utility for writing modern front-end
applications. You can think of it as a minimal `Reactjs`.

### Rust? I only know JavaScript though.
retort-js is written in rust, but that doesn't mean you should know *any* rust to use it. As mentioned earlier, `rust` code is complied to `wasm` and `.wasm` files
can be used inside JavaScript modules(with a bit of glue code, courtesy of `wasm-bindgen`). So all a developer is interfaced with, is good old JavaScript.

### JavaScript? I only know Rust though.
That's even better. You could contribute to this project if you'd like by doing one of the items mentioned in `tbd` section.

## Dive deeper
I'm going to get a little more technical here, explaining how it all works.
#### Tokenizer module
This module consists of 3 parts; utility functions, a publicly available wrapper function and unit tests for all previous functions. The wrapper function, `tokenizer`, takes a `String` as a parameter and returns a closure. Each successful call to the returened closuer will return the next tokenized value and its type, which is a variant of `TokenizerState` enum:
```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TokenizerState {
    Uninitialized,
    OpenAngleBracket,        // <
    CloseAngleBracket,       // >
    SelfClosingAngleBracket, // />
    ClosingAngleBracket,     // </
    TagNameOpen,
    TagNameClose,
    Component,
    Props,
    Text,
    Finalized,
}
```


## Outline
- [x] Tokenizer
- [x] Parser
- [x] JavaScript Evaluator 
- [x] DOM initialization
- [x] Conditional rendering
- [ ] Rendering lists
- [x] Prop handling 
- [x] Error handling
- [ ] effect handling -> on going
- [ ] state management -> on going
- [ ] DOM updates -> up comming
- [ ] Unit tests -> on going


