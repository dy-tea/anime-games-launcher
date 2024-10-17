use adw::prelude::*;
use gtk::prelude::*;

use relm4::{prelude::*, factory::*};

#[derive(Debug)]
struct ComponentVersionsFactory {
    name: String,
    downloaded: bool,
}

#[relm4::factory(async)]
impl AsyncFactoryComponent for ComponentVersionsFactory {
    type Init = ComponentVersionsFactory;
    type Input = ();
    type Output = ();
    type ParentWidget = adw::ExpanderRow;
    type CommandOutput = ();

    view! {
        #[root]
        adw::ActionRow {
            set_title: &self.name,
            add_suffix = &gtk::Button {
                add_css_class: "flat",
                #[watch]
                set_icon_name: if self.downloaded {"user-trash-symbolic"} else {"download-symbolic"},
                set_align: gtk::Align::Center
            }
        }
    }

    async fn init_model(init: Self::Init, index: &DynamicIndex, sender: AsyncFactorySender<Self>) -> Self {
        init
    }
}

#[derive(Debug)]
struct ComponentVersions {
    name: String,
    versions: AsyncFactoryVecDeque<ComponentVersionsFactory>,
}

#[relm4::factory(async)]
impl AsyncFactoryComponent for ComponentVersions {
    type Init = String;
    type Input = ();
    type Output = ();
    type ParentWidget = gtk::ListBox;
    type CommandOutput = ();

    view! {
        #[root]
        gtk::ListBoxRow { // TODO: Fix this abomination, ideally we should not have this line
            self.versions.widget() {
                set_title: &self.name,
            }
        }
    }

    async fn init_model(init: Self::Init, _index: &DynamicIndex, _sender: AsyncFactorySender<Self>) -> Self {
        let mut versions: AsyncFactoryVecDeque<ComponentVersionsFactory> = AsyncFactoryVecDeque::builder().launch_default().detach();

        versions.guard().push_back(ComponentVersionsFactory {
            name: "Wine-Staging-TkG 9.8".to_string(),
            downloaded: true,
        });

        versions.guard().push_back(ComponentVersionsFactory {
            name: "Wine-Staging-TkG 9.1".to_string(),
            downloaded: false,
        });


        versions.guard().push_back(ComponentVersionsFactory {
            name: "Wine-Staging-TkG 8.0".to_string(),
            downloaded: false,
        });

        Self {
            name: init,
            versions,
        }
    }
}

#[derive(Debug)]
pub struct ComponentPage {
    name: String,
    versions: AsyncFactoryVecDeque<ComponentVersions>
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for ComponentPage {
    type Init = ();
    type Input = ();
    type Output = ();

    view! {
        #[root]
        adw::NavigationPage {
            set_title: &model.name,

            #[wrap(Some)]
            set_child = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                adw::HeaderBar,
                adw::PreferencesPage {
                    adw::PreferencesGroup {
                        adw::ComboRow {
                            set_title: "Selected version",
                            set_subtitle: "Select the version of wine you want to use",
                            set_model: Some(&gtk::StringList::new(&["Wine-Staging-TkG 9.8", "Wine-Staging-TkG 8.1", "Wine-Staging-TkG 8.0"])),
                        },
                        adw::SwitchRow {
                            set_active: true,
                            set_title: "Recommended only",
                            set_subtitle: "Show only recommended wine versions"
                        }
                    },

                    adw::PreferencesGroup {
                        set_title: "Available Versions",
                        model.versions.widget() {
                            set_selection_mode: gtk::SelectionMode::None,
                            add_css_class: "boxed-list"
                        }
                    }
                }
            }
        }
    }

    async fn init(init: Self::Init, root: Self::Root, sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let mut model = Self {
            name: String::from("Wine"),
            versions: AsyncFactoryVecDeque::builder().launch_default().detach()
        };
        let widgets = view_output!();

        for _ in 0..5 {
            model.versions.guard().push_back(String::from("Wine-Staging-TkG"));
        }

        AsyncComponentParts { model, widgets }
    }
}
