use duid::{
    html::{
        div, canvas, text, button, nodes::Node,
        attributes::{classes, selectors, Attribute, AttributeValue, Value},
    },
    duid_events::{NodeMapMsg, Cmd,Sub},
    events::{on_click, on_mount},
};
use duid_data_grid::{DataGridModel, DataGridMsg, data_grid_view, log};

// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum AppMsg {
    Msg,
    DataGrid(DataGridMsg)
}


#[derive(Debug, Clone, PartialEq)]
pub struct AppModel {
    data_grid_model: DataGridModel
}


impl AppModel {
    pub fn new() -> Self {
        AppModel {
            data_grid_model: DataGridModel::new(
                "duid-data-grid".to_string(),
                "100%".to_string(),
                "500.0px".to_string(),
                Some(200),
                None
            )
        }
    }
}

pub fn app_view(app_model: &AppModel) -> Node<AppMsg> {
    
    div(
        &[],
        &[
            text("Arnaud"),
            data_grid_view(&app_model.data_grid_model).map_msg(|m| AppMsg::DataGrid(m))
        ]
    )
}


pub fn app_update(model: &mut AppModel, msg: AppMsg) -> Cmd<AppMsg> {
    
    match msg {
        AppMsg::DataGrid(grid) => {
            Cmd::none()
        },
        _ => Cmd::none()
    }
}

pub fn app_subscription(model: &AppModel) -> Sub<AppMsg> {
    Sub::none()
}