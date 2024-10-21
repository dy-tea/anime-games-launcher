use adw::prelude::*;
use gtk::prelude::*;

use relm4::prelude::*;

#[derive(Debug)]
struct Variable {
    key: String,
    val: String,
}

#[relm4::factory(async)]
impl AsyncFactoryComponent for Variable {
    type Init = (String, String);
    type Input = EnvironmentPageMsg;
    type Output = EnvironmentPageMsg;
    type ParentWidget = adw::PreferencesGroup;
    type CommandOutput = ();

    view! {
        #[root]
        adw::ActionRow {
            set_title: &self.key,
            set_subtitle: &self.val,

            add_prefix = &gtk::CheckButton {
                set_tooltip_text: Some("Enable"),
            },

            add_suffix = &gtk::Button {
                set_align: gtk::Align::Center,
                set_icon_name: "user-trash-symbolic",
                add_css_class: "flat",
                set_tooltip_text: Some("Delete"),
                connect_clicked[sender, index] => move |_| {
                    sender.output(EnvironmentPageMsg::Remove(index.current_index())).unwrap();
                }
            }
        }
    }

    async fn init_model(init: Self::Init, index: &DynamicIndex, _sender: AsyncFactorySender<Self>) -> Self {
        Self {
            key: init.0,
            val: init.1,
        }
    }
}

#[derive(Debug)]
pub struct EnvironmentPage {
    variables: AsyncFactoryVecDeque<Variable>,

    name_entry: adw::EntryRow,
    value_entry: adw::EntryRow,
}

#[derive(Debug)]
pub enum EnvironmentPageMsg {
    Add,
    Remove(usize),
    SetActive(bool, usize),
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for EnvironmentPage {
    type Init = ();
    type Input = EnvironmentPageMsg;
    type Output = EnvironmentPageMsg;

    view! {
        #[root]
        adw::NavigationPage {
            set_title: "Environment",

            #[wrap(Some)]
            set_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                adw::HeaderBar,

                adw::PreferencesPage {
                    add = &adw::PreferencesGroup {
                        #[local_ref]
                        name_entry -> adw::EntryRow {
                            set_title: "Name"
                        },

                        #[local_ref]
                        value_entry -> adw::EntryRow {
                            set_title: "Value"
                        },

                        gtk::Button {
                            set_label: "Add",
                            set_margin_all: 16,
                            add_css_class: "pill",
                            connect_clicked => EnvironmentPageMsg::Add,
                        },

                        model.variables.widget(),
                    }
                }
            }
        }
    }

    async fn init(init: Self::Init, root: Self::Root, sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let model = Self {
            variables: AsyncFactoryVecDeque::builder().launch_default().forward(sender.input_sender(), std::convert::identity),
            name_entry: adw::EntryRow::new(),
            value_entry: adw::EntryRow::new(),
        };

        let name_entry = &model.name_entry;
        let value_entry = &model.value_entry;

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            EnvironmentPageMsg::Add => {
                let name = self.name_entry.text().trim().to_string();
                let value = self.value_entry.text().trim().to_string();

                if !name.is_empty() && !value.is_empty() {
                    self.variables.guard().push_back((name, value));
                }
            },
            EnvironmentPageMsg::Remove(index) => {
                let mut guard = self.variables.guard();
                if guard.get(index).is_some() {
                    guard.remove(index);
                }
            },
            EnvironmentPageMsg::SetActive(active, index) => todo!(),
        }
    }
}
