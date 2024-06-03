# What is retort-js?
retort-js is a rust-based front-end library which compiles to `wasm` using `wasm-pack` and `wasm-bindgen`. It aims to provide utility for writing modern front-end
applications. You can think of it as a minimal `Reactjs`.

### Rust? I only know JavaScript though.
retort-js is written in rust, but that doesn't mean you should know *any* rust to use it. As mentioned earlier, `rust` code is complied to `wasm` and `.wasm` files
can be used inside JavaScript modules(with a bit of glue code, courtesy of `wasm-bindgen`). So all a developer is interfaced with, is good old JavaScript.
<!---
### JavaScript? I only know Rust though.
That's even better. You could contribute to this project if you'd like by doing one of the items mentioned in `tbd` section.
--->
## A deeper dive
I'm going to get a little more technical here, explaining the general idea on how it works.
#### Component module*
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
Using objects of this struct, a user can create `Component` objects in JavaScript. As you can see, the only properties which are needed at the moment of initialization, are `state` and `presenter`. `state` is normally a JavaScript object, and represents the state of a component. It is updated on predefined events using the following method:
```rust
#[wasm_bindgen]
pub fn set_state(&mut self, callback: Function) // Function type, from js_sys. represents a JavaScript callback.
```
which in JavaScript, would look like:
```JavaScript
 fetch(`https://jsonplaceholder.typicode.com/todos/${state.age}`)
   .then((res) => res.json())
   .then((res) => component.set_state((prevState) => ({ ...prevState, info: res })));
```
as you can see, the `set_state` function on `Component` instances take a callback which its parameter is current state of component. Pretty JavaScript-ish!

`presenter` is a string, which is basically the markup template for the component. A valid presenter may look like this:
```
import HelloWorld from "/test/HelloWorld/HelloWorld.js";
<main>
  <h1>My name is {state.name} and i'm {state.age} years old.</h1>
  <HelloWorld />
</main> 
```
An important thing to notice here is the use of curly brackets to indicate use of state or prop value. Other kinds of variables like those defined with `const` keyword or event callbacks like `onclick={callback}` are not yet supported.

Other properties are later added on demand using `setter` functions; for instance, the following function allows you to register a callback which will be called
when component mounts:
```rust
#[wasm_bindgen]
pub fn register_component_did_mount(&mut self, callback: Function)
```
example usage:
```JavaScript
component.register_component_did_mount(
(intialProps, props, initialState, state) => {
  const element = document.getElementById("click");
  if (!element) return;
  element.addEventListener("click", clickCallback);

  function clickCallback() {
    component.set_state((prev) => ({ ...prev, age: prev.age + 1 }));
  }
}
);
```
Other than the `Component` struct, this module has 2 other publicly available members; `mount` and `render`. `mount` is used only on the root component and is basically
the starting point of our applications written with retort. `render` though, must be called for every component that is going to be used in the application, because
it creates and populates the VDOM representation of the component, the one that we left out during the initialization of our component.

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
This module provides utility functions to parse the `presenter` of a component. Each presenter consists of at most 2 parts; the import statements and the markup template.

#### Parser module*
This module consists of a driver function for the functionality provided by `tokenizer` module. the `parse_vdom_from_string` function transforms meaningless tokens
into `VirtualNode` objects and returns a single virtual node.

#### dom module*
This module provides functionality to build up the DOM according to the context of components and their VDOM representation:
```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum NodeType {
    Component(Component), //component object
    Tag(String),          // tag name
    Text(String),         // text content
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VirtualNode {
    pub node_type: NodeType,
    pub attributes: HashMap<String, String>,
    pub children: Vec<VirtualNode>,
}
```
The `construct_dom` function will decide which utility function to call based on `NodeType` for current `VirtualNode` object.

#### evaluator module
This is one of the most imporant modules in retort. We saw earlier that retort supports usage of some types of expressions in `presenter`. The thing is, these
expressions are JavaScript expressions and need to(are expected to) be evaluated with JavaScript runtime behaviors. So, after retort detects an expression, that
expression is evaluated using the `new Function()` syntax. But that doesn't mean we are sending expression back and forth to JavaScript. This is done using one of
the most exiciting features of `wasm-bindgen`:
```rust
#[wasm_bindgen(js_namespace=window)]
extern "C" {
    fn Function(arg1: String, arg2: String, function_string: String) -> Function;
}
```
This syntax is basically empowering us to use the `new Function` syntax of JavaScript inside rust environment and get the same result. [Read more](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Function/Function) on what the
`Function` constructor does in JavaScript and [why](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/eval#never_use_direct_eval!) we are not using the `eval` function.

#### Error module
This is a module to improve DX. It visualizes the encountered errors during developement for the developer:
![image](https://github.com/alivarastepour/retort-js/assets/81034797/8e1ec052-8bc8-41c1-8a9d-38bd8b923eac)


*->these modules are not yet stable.
<!---
### Contribution
If you are enthusiastic about Rust, or have knowledge both on Rust and on modern front-end libraries(any library would probably do) You are more than welcome to
contribute to this project.

You can:
- write unit tests for modules that are not tested.
- Help out on compeleting tasks in the Outline section

### Run locally
In order to run this project on your machine, you need to have [`Rust`](https://www.rust-lang.org/tools/install) toolchain and [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/) installed. Then clone the repo and run:
```shell
wasm-pack build --target web
```
and to run tests:
```shell
cargo test
```
-->

## Outline
- [x] Tokenizer
- [x] Parser
- [x] JavaScript Evaluator 
- [x] DOM initialization
- [x] Conditional rendering
- [ ] Rendering lists
- [ ] Prop handling 
- [x] Error handling
- [ ] effect handling -> on going
- [ ] state management -> on going
- [ ] DOM updates -> up comming
- [ ] Unit tests -> on going


