use web_sys::{wasm_bindgen::JsCast, Document, Element, HtmlElement, HtmlInputElement, Window};

pub fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document(window: Window) -> Document {
    window.document().expect("should have a document on window")
}

pub fn body(document: &Document) -> HtmlElement {
    document.body().expect("document should have a body")
}

pub fn read_value_from_text_field(document: &Document, element_id: &str) -> String {
    document
        .get_element_by_id(element_id)
        .unwrap()
        .clone()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
}

pub fn text_field_is_empty(text_field: &Element) -> bool {
    text_field
        .clone()
        .dyn_into::<HtmlInputElement>()
        .unwrap()
        .value()
        .is_empty()
}
