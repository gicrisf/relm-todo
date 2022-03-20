use adw::prelude::*;
use gtk::prelude::{BoxExt, WidgetExt, OrientableExt, EntryExt, EntryBufferExtManual, CheckButtonExt};
use relm4::factory::{FactoryPrototype, FactoryVec};
use relm4::{adw, gtk, send, Model, AppUpdate, RelmComponent, RelmApp, Widgets, WidgetPlus, Sender};

mod header_component;
use crate::header_component::HeaderModel;

enum AppMode {
    View,
    Edit,
    Export,
}

enum AppMsg {
    SetMode(AppMode),
    SetCompleted((usize, bool)),
    AddEntry(String),
}

struct Task {
    name: String,
    completed: bool,
}

#[derive(Debug)]
struct TaskWidgets {
    label: gtk::Label,
    hbox: gtk::Box,
}

impl FactoryPrototype for Task {
    type View = gtk::ListBox;
    type Msg = AppMsg;
    type Factory = FactoryVec<Task>;
    type Widgets = TaskWidgets;
    type Root = gtk::Box;

    fn init_view(&self, key: &usize, sender: Sender<Self::Msg>) -> Self::Widgets {
        let hbox = gtk::Box::builder().orientation(gtk::Orientation::Horizontal).build();
        let label = gtk::Label::new(Some(&self.name));
        let checkbox = gtk::CheckButton::builder().active(false).build();

        assert!(!self.completed);

        checkbox.set_margin_all(12);
        label.set_margin_all(12);

        hbox.append(&checkbox);
        hbox.append(&label);

        let index = *key;
        checkbox.connect_toggled(move |checkbox| {
            send!(sender, AppMsg::SetCompleted((index, checkbox.is_active())));
        });

        TaskWidgets { label, hbox }
    } // ./init_view

    fn position(&self, _key: &usize) {}

    fn view(&self, _key: &usize, widgets: &Self::Widgets) {
        let attrs = widgets.label.attributes().unwrap_or_default();
        attrs.change(gtk::pango::AttrInt::new_strikethrough(self.completed));
        widgets.label.set_attributes(Some(&attrs));
    } // ./ view

    fn root_widget(widgets: &Self::Widgets) -> &Self::Root {
        &widgets.hbox
    }  // ./root_widgets
} // ./Task's Prototype


// -- Model

#[derive(relm4::Components)]
struct AppComponents {
    header: RelmComponent<HeaderModel, AppModel>
}

struct AppModel {
    tasks: FactoryVec<Task>,
    mode: AppMode,
}

impl Model for AppModel {
    type Msg = AppMsg;
    type Widgets = AppWidgets;
    type Components = AppComponents;
}

// -- Update

impl AppUpdate for AppModel {
    fn update(&mut self, msg: AppMsg, _components: &AppComponents, _sender: Sender<AppMsg>) -> bool {
        match msg {
            AppMsg::SetMode(mode) => {
                self.mode = mode;
            }
            AppMsg::SetCompleted((index, completed)) => {
                if let Some(task) = self.tasks.get_mut(index) {
                    task.completed = completed;
                }
            }
            AppMsg::AddEntry(name) => {
                self.tasks.push(Task {
                    name,
                    completed: false,
                })
            }
        }  // ./msg
        true
    } // ./update
}

// -- View

#[relm4::widget]
impl Widgets<AppModel, ()> for AppWidgets {
    view! {
        main_window = adw::ApplicationWindow {
            set_width_request: 360,
            set_title: Some("To-Do"),

            set_content: main_box = Some(&gtk::Box) {
                set_orientation: gtk::Orientation::Vertical,

                append = &adw::HeaderBar {
                    set_title_widget: Some(&gtk::Label::new(Some("To-Do"))),
                },
                append = &gtk::Entry {
                    set_margin_all: 12,
                    connect_activate(sender) => move |entry| {
                        let buffer = entry.buffer();
                        send!(sender, AppMsg::AddEntry(buffer.text()));
                        buffer.delete_text(0, None);
                    }
                },
                append = &gtk::ScrolledWindow {
                    set_hscrollbar_policy: gtk::PolicyType::Never,
                    set_min_content_height: 360,
                    set_vexpand: true,
                    set_child = Some(&gtk::ListBox) {
                        factory!(model.tasks),
                    }
                }
            }
        } // Application Window
    }
}

// -- Main

fn main() {
    let model = AppModel {
        tasks: FactoryVec::new(),
        mode: AppMode::View,
    };
    let relm = RelmApp::new(model);
    relm.run();
}
