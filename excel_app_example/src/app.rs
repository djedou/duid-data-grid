use duid::{
    html::{
        div, canvas, text, button, nodes::Node,
        attributes::{classes, selectors, Attribute, AttributeValue, Value},
    },
    duid_events::{NodeMapMsg, Cmd,Sub},
    events::{on_click, on_mount},
};


// Messages
#[derive(Debug, PartialEq, Clone)]
pub enum AppMsg {
    Msg,
}


#[derive(Debug, Clone, PartialEq)]
pub struct AppModel {
    canvas: String
}


impl AppModel {
    pub fn new() -> Self {
        AppModel {
            canvas: String::with_capacity(0)
        }
    }
}

pub fn app_view(app_model: &AppModel) -> Node<AppMsg> {
    
    div(
        &[],
        &[
            text("Mount Canvas")
        ]
    )
}


pub fn app_update(model: &mut AppModel, msg: AppMsg) -> Cmd<AppMsg> {
    Cmd::none()
}

pub fn app_subscription(model: &AppModel) -> Sub<AppMsg> {
    Sub::none()
}