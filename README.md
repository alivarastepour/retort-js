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
#### Component module
Component module is the only module which user directly interacts with. The `Component` struct and its constructor are the most important bits in this module:
```rust
#[derive(Serialize, Deserialize, Debug)]
#[wasm_bindgen]
pub struct Component {
    state: String,
    presenter: String,
    props: String,
    vdom: Box<VirtualNode>,
    #[serde(with = "serde_wasm_bindgen::preserve")]
    effects: Array,
    #[serde(with = "serde_wasm_bindgen::preserve")]
    component_did_mount: Array,
    #[serde(with = "serde_wasm_bindgen::preserve")]
    component_will_unmount: Array,
}

#[wasm_bindgen(constructor)]
pub fn new(state: String, presenter: String) -> Component {
    let empty_vdom = Box::new(VirtualNode {
        // no need to have a valid vdom at this point
        attributes: HashMap::new(),
        children: Vec::new(),
        node_type: NodeType::Tag(" ".to_owned()),
    });
    Component {
        state,
        presenter,
        props: "{}".to_owned(),
        vdom: empty_vdom,
        effects: Array::new(),
        component_will_unmount: Array::new(),
        component_did_mount: Array::new(),
    }
}
```
Using objects of this struct, a user can create Component objects in JavaScript. As you can see, the only properies which are needed at the moment of initialization, are `state` and `presenter`. `state` is normally a JavaScript object, and represents the state of a component. they are updated on predefined events using the following method:
```rust
#[wasm_bindgen]
pub fn set_state(&mut self, callback: Function)
```
which in JavaScript, would look like:
```JavaScript
 fetch(`https://jsonplaceholder.typicode.com/todos/${state.age}`)
   .then((res) => res.json())
   .then((res) => component.set_state((p) => ({ ...p, info: res })));
```
as you can see, the `set_state` function on `Component` instances take a callback which its parameter is current state of component. Pretty JavaScript-ish!

`presenter` is a string, which is basically the markup
template for the component. A valid presenter may look like this:
```
import HelloWorld from "/test/HelloWorld/HelloWorld.js";
<main>
  <h1>My name is {state.name} and i'm {state.age} years old.</h1>
  <HelloWorld />
</main> 
```
Other properties are later added on demand using `setter` functions.

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
#### Presenter module
#### Parser module
This module consists of a driver function for the functionality provided by `tokenizer` module. the `parse_vdom_from_string` function transforms meaningless tokens
into `VirtualNode` objects and returns a single virtual node, which is the root of our DOM hierarchy.

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


