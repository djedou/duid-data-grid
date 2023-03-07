use duid::{
    html::{
        nodes::{Node, create_element},
        attributes::{classes, selectors, Attribute, AttributeValue, Value},
    },
    //duid_events::{NodeMapMsg, Cmd,Sub},
    events::{on_click, on_mount},
};
use crate::{DataGridModel, DataGridMsg, name, log};




pub fn data_grid_view(model: &DataGridModel) -> Node<DataGridMsg> {
    /*
    log(&format!("Hello from {}!", name()));
    log(&format!("Initial number is {}!", model.number()));
    log(&format!("Canvas id {}!", model.grid_id()));
    model.set_number(10);
    log(&model.render());
*/

    create_element(
        None,
        "canvas-datagrid",
        &[
            Attribute::new(None, "id", AttributeValue::from_value(Value::String(model.grid_id().to_string()))),
            //Attribute::new(None, "width", AttributeValue::from_value(Value::F32(700.0))),
            //Attribute::new(None, "height", AttributeValue::from_value(Value::F32(500.0)))
        ],
        &[]
    )
}