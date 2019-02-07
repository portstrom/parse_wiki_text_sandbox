// Copyright 2019 Fredrik Portström <https://portstrom.com>
// This is free software distributed under the terms specified in
// the file LICENSE at the top-level directory of this distribution.

extern crate parse_wiki_text;
extern crate wasm_bindgen;

macro_rules! html { ($tag_name:ident $(. $class_names:ident)*) => {{
    let element = ::document.create_element(stringify!($tag_name));
    $(element.set_class_names(stringify!($class_names));)*
    element
}}}

mod parse;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type Document;
    type History;
    pub type HTMLTextAreaElement;

    static document: Document;
    static history: History;

    #[wasm_bindgen(method, js_name = append)]
    fn append_element(this: &HTMLTextAreaElement, content: &HTMLTextAreaElement);

    #[wasm_bindgen(method, js_name = append)]
    fn append_text(this: &HTMLTextAreaElement, content: &str);

    #[wasm_bindgen(method, getter)]
    fn body(this: &Document) -> HTMLTextAreaElement;

    #[wasm_bindgen(method, js_name = createElement)]
    fn create_element(this: &Document, tag_name: &str) -> HTMLTextAreaElement;

    #[wasm_bindgen(method)]
    fn focus(this: &HTMLTextAreaElement);

    #[wasm_bindgen(method, js_name = replaceState)]
    fn replace_state(this: &History, state: &str, junk: u32);

    #[wasm_bindgen(method, setter = className)]
    fn set_class_names(this: &HTMLTextAreaElement, class_names: &str);

    #[wasm_bindgen(method, setter = oninput)]
    fn set_input_handler(this: &HTMLTextAreaElement, callback: &Closure<FnMut()>);

    #[wasm_bindgen(method, setter = innerText)]
    fn set_inner_text(this: &HTMLTextAreaElement, text: &str);

    #[wasm_bindgen(method, getter)]
    fn state(this: &History) -> JsValue;

    #[wasm_bindgen(method, getter)]
    fn value(this: &HTMLTextAreaElement) -> String;

    #[wasm_bindgen(method, setter)]
    fn set_value(this: &HTMLTextAreaElement, value: &str) -> String;
}

#[wasm_bindgen]
pub extern "C" fn main() {
    let input_el = std::rc::Rc::new(html!(textarea));
    let result_el = html!(div.result);
    if let Some(wiki_text) = history.state().as_string() {
        input_el.set_value(&wiki_text);
        parse::parse(&wiki_text, &result_el);
    } else {
        let wiki_text = "Hello, [[World]]!";
        input_el.set_value(wiki_text);
        parse::parse(&wiki_text, &result_el);
    }
    let body = document.body();
    body.append_element(&input_el);
    body.append_element(&result_el);
    let cloned_input_el = input_el.clone();
    let handle_input = Closure::wrap(Box::new(move || {
        let wiki_text = cloned_input_el.value();
        history.replace_state(&wiki_text, 0);
        parse::parse(&wiki_text, &result_el);
    }) as Box<FnMut()>);
    input_el.set_input_handler(&handle_input);
    handle_input.forget();
    input_el.focus();
}
