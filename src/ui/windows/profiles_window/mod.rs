use adw::prelude::*;
use relm4::prelude::*;

pub mod component_page;
use component_page::*;

#[derive(Debug)]
pub struct CreateWineProfileApp {
    window: adw::PreferencesWindow,
    wine_page: AsyncController<ComponentPage>,
    dxvk_page: AsyncController<ComponentPage>,
    container_page: AsyncController<ComponentPage>
}

#[derive(Debug, Clone)]
pub enum CreateWineProfileAppMsg {
    Create {
        name: String
    },
    OpenWinePage,
    OpenDxvkPage,
    OpenContainerPage
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for CreateWineProfileApp {
    type Init = ();
    type Input = CreateWineProfileAppMsg;
    type Output = ();

    view! {
        #[root]
        window = adw::PreferencesWindow {
            set_size_request: (700, 560),
            set_title: Some("Modify profile"),

            set_search_enabled: false,
            set_hide_on_close: true,
            set_modal: true,

            add_css_class?: crate::APP_DEBUG.then_some("devel"),

            add = &adw::PreferencesPage {
                add = &adw::PreferencesGroup {
                    set_title: "Profile info",

                    #[name = "profile_name_row"]
                    adw::EntryRow {
                        set_title: "Profile name"
                    },
                    adw::SwitchRow {
                        set_title: "Set default",
                        set_subtitle: "Use this profile by default with newly installed games"
                    }
                },

                add = &adw::PreferencesGroup {
                    set_title: "Runner",

                    adw::SwitchRow {
                        set_title: "Native Linux",
                    },

                    adw::ActionRow {
                        set_title: "Wine",
                        add_suffix = &gtk::Label {
                            set_text: "Wine-Staging-TkG 9.8"
                        },
                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                        },
                        set_activatable: true,
                        connect_activated => CreateWineProfileAppMsg::OpenWinePage,
                    },

                    adw::ExpanderRow {
                        set_title: "Wine tools",
                        add_row = &adw::ActionRow {
                            set_activatable: true,
                            set_title: "Command line",
                            set_subtitle: "wineconsole",
                        },
                        add_row = &adw::ActionRow {
                            set_activatable: true,
                            set_title: "Registry editor",
                            set_subtitle: "regedit",
                        },
                        add_row = &adw::ActionRow {
                            set_activatable: true,
                            set_title: "Explorer",
                            set_subtitle: "explorer",
                        },
                        add_row = &adw::ActionRow {
                            set_activatable: true,
                            set_title: "Task manager",
                            set_subtitle: "taskmgr",
                        },
                        add_row = &adw::ActionRow {
                            set_activatable: true,
                            set_title: "Configuration",
                            set_subtitle: "winecfg",
                        },
                        add_row = &adw::ActionRow {
                            set_activatable: true,
                            set_title: "Debugger",
                            set_subtitle: "start winedbg",
                        }
                    },

                    adw::ComboRow {
                        set_title: "Synchronization",
                        set_subtitle: "Set the synchronization method for wine",
                        set_model: Some(&gtk::StringList::new(&["FSync", "Esync", "None"])),
                    },

                    adw::ComboRow {
                        set_title: "Language",
                        set_subtitle: "Language used in the wine environment. Can fix keyboard layout issues",
                        set_model: Some(&gtk::StringList::new(&["System", "English"]))
                    },

                    adw::SwitchRow {
                        set_title: "Borderless Window",
                        set_active: false
                    },

                    adw::ComboRow {
                        set_title: "Virtual Desktop",
                        set_model: Some(&gtk::StringList::new(&["1920x1080", "1280x720", "1600x900"])),
                        add_suffix = &gtk::Switch {
                            set_valign: gtk::Align::Center,
                            set_active: false
                        },
                    },

                    adw::SwitchRow {
                        set_title: "Map drive C:",
                        set_subtitle: "Automatically symlink drive_c folder from the wine prefix to the dosdevices",
                        set_active: true,
                    },

                    adw::ComboRow {
                        set_title: "Map game folder",
                        set_subtitle: "Automatically symlink game folder to the dosdevices",
                        set_model: Some(&gtk::StringList::new(&["a:", "b:", "c:", "d:", "e:", "f:", "g:", "h:", "i:", "j:", "k:", "l:", "m:", "n:", "o:", "p:", "q:", "r:", "s:", "t:", "u:", "v:", "w:", "x:", "y:", "z:"])),
                        add_suffix = &gtk::Switch {
                            set_valign: gtk::Align::Center,
                            set_active: true
                        },
                    },
                },

                add = &adw::PreferencesGroup {
                    set_title: "DXVK",

                    adw::SwitchRow {
                        set_title: "Use DXVK",
                        set_active: true
                    },

                    adw::ActionRow {
                        set_title: "DXVK",
                        add_suffix = &gtk::Label {
                            set_text: "DXVK 2.1"
                        },
                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                        },
                        set_activatable: true,
                        connect_activated => CreateWineProfileAppMsg::OpenContainerPage,
                    },
                },

                add = &adw::PreferencesGroup {
                    set_title: "Game",

                    adw::ComboRow {
                        set_title: "HUD",
                        set_model: Some(&gtk::StringList::new(&["None", "DXVK", "MangoHud"])),
                    },

                    adw::ComboRow {
                        set_title: "FSR",
                        set_subtitle: "Upscales game to your monitor size. To use select a lower resolution in the game's settings and press Alt+Enter",
                        set_model: Some(&gtk::StringList::new(&["Ultra Quality", "Quality", "Balanced", "Performance"])),
                        add_suffix = &gtk::Switch {
                            set_valign: gtk::Align::Center,
                            set_active: true
                        },
                    },

                    adw::SwitchRow {
                        set_title: "Gamemode",
                        set_subtitle: "Prioritize the game over the rest of the processes",
                    },

                    adw::SwitchRow {
                        set_title: "Gamescope",
                        set_subtitle: "Gamescope is tool from Valve that allows you to run games in a separate X session",
                    }
                },

                add = &adw::PreferencesGroup {
                    set_title: "Containerization",

                    adw::SwitchRow {
                        set_title: "Use Containerization",
                        set_active: false
                    },

                    adw::ActionRow {
                        set_title: "Container system",
                        add_suffix = &gtk::Label {
                            set_text: "Alpine 3.19"
                        },
                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                        },
                        set_activatable: true,
                        connect_activated => CreateWineProfileAppMsg::OpenDxvkPage,
                    }
                },

                add = &adw::PreferencesGroup {
                    set_title: "Environment",

                    adw::EntryRow {
                        set_title: "Run command"
                    }
                },

                add = &adw::PreferencesGroup {
                    gtk::Button {
                        add_css_class: "pill",
                        add_css_class: "suggested-action",

                        set_label: "Save",

                        connect_clicked[sender, profile_name_row] => move |_| {
                            sender.input(CreateWineProfileAppMsg::Create {
                                name: profile_name_row.text().to_string()
                            })
                        }
                    }
                }
            }
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let model = Self {
            window: root.clone(),
            wine_page: ComponentPage::builder().launch(()).detach(),
            dxvk_page: ComponentPage::builder().launch(()).detach(),
            container_page: ComponentPage::builder().launch(()).detach()
        };

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        match msg {
            CreateWineProfileAppMsg::Create { name } => {

            }
            CreateWineProfileAppMsg::OpenWinePage => {
                self.window.push_subpage(self.wine_page.widget());
            }
            CreateWineProfileAppMsg::OpenDxvkPage => {
                self.window.push_subpage(self.dxvk_page.widget());
            }
            CreateWineProfileAppMsg::OpenContainerPage => {
                self.window.push_subpage(self.container_page.widget());
            }
        }
    }
}
