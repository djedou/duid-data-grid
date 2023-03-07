
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/js/data-grid.js")]
extern "C" {

    pub fn name() -> String;

    #[derive(Debug, PartialEq, Clone)]
    pub type DataGridModel;

    #[wasm_bindgen(constructor)]
    pub fn new(
        id: String,
        width: String,
        height: String,
        rows: Option<u32>,
        columns: Option<u32>
    ) -> DataGridModel;

    #[wasm_bindgen(method, getter)]
    pub fn number(this: &DataGridModel) -> u32;

    #[wasm_bindgen(method, setter)]
    pub fn set_number(this: &DataGridModel, number: u32) -> DataGridModel;
    
    #[wasm_bindgen(method, getter, js_name = gridId)]
    pub fn grid_id(this: &DataGridModel) -> String;

    #[wasm_bindgen(method, setter, js_name = set_gridId)]
    pub fn set_grid_id(this: &DataGridModel, id: String) -> DataGridModel;

    #[wasm_bindgen(method)]
    pub fn render(this: &DataGridModel) -> String;
}


// lifted from the `console_log` example
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}
