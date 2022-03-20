use gtk::prelude::{BoxExt, ButtonExt, WidgetExt, ToggleButtonExt};
use relm4::{send, ComponentUpdate, Widgets, Sender, Model};

use crate::{AppModel, AppMsg, AppMode};

pub enum HeaderMsg {
    View,
    Edit,
    Export,
}

pub struct HeaderModel {}

impl Model for HeaderModel {
    type Msg = HeaderMsg;
    type Widgets = HeaderWidgets;
    type Components = ();
}

impl ComponentUpdate<AppModel> for HeaderModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        HeaderModel {}
    }

    fn update(
        &mut self,
        msg: HeaderMsg,
        _components: &(),
        _sender: Sender<HeaderMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {
            HeaderMsg::View => {
                send!(parent_sender, AppMsg::SetMode(AppMode::View));
            }
            HeaderMsg::Edit => {
                send!(parent_sender, AppMsg::SetMode(AppMode::Edit));
            }
            HeaderMsg::Export => {
                send!(parent_sender, AppMsg::SetMode(AppMode::Export));
            }
        }
    }
}

#[relm4::widget(pub)]
impl Widgets<HeaderModel, AppModel> for HeaderWidgets {
    view! {
        gtk::HeaderBar {
            set_title_widget = Some(&gtk::Box) {
                add_css_class: "linked",
                append: group = &gtk::ToggleButton {
                    set_label: "View",
                    set_active: true,
                    connect_toggled(sender) => move |btn| {
                        if btn.is_active() {
                            send!(sender, HeaderMsg::View);
                        }
                    }
                },
                append = &gtk::ToggleButton {
                    set_label: "Edit",
                    set_group: Some(&group),
                    connect_toggled(sender) => move |btn| {
                        if btn.is_active() {
                            send!(sender, HeaderMsg::Edit);
                        }
                    }
                },
                append = &gtk::ToggleButton {
                    set_label: "Export",
                    set_group: Some(&group),
                    connect_toggled(sender) => move |btn| {
                        if btn.is_active() {
                            send!(sender, HeaderMsg::Export);
                        }
                    }
                }
            }
        }

    }
}
