use adw::prelude::*;
use relm4::prelude::*;

pub mod component_page;
use component_page::*;

pub mod environment_page;
use environment_page::*;

#[relm4::widget_template(pub)]
impl WidgetTemplate for ComboSwitchRow {
    view! {
        adw::ComboRow {
            add_suffix = &gtk::Switch {
                set_valign: gtk::Align::Center
            }
        }
    }
}

#[derive(Debug)]
pub struct CreateWineProfileApp {
    window: adw::PreferencesWindow,
    wine_page: AsyncController<ComponentPage>,
    dxvk_page: AsyncController<ComponentPage>,
    container_page: AsyncController<ComponentPage>,
    environment_page: AsyncController<EnvironmentPage>,

    is_native: bool,
}

#[derive(Debug, Clone)]
pub enum CreateWineProfileAppMsg {
    Create {
        name: String
    },
    OpenWinePage,
    OpenDxvkPage,
    OpenContainerPage,
    OpenEnvironmentPage,

    SetNative(bool),
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
                        #[watch]
                        set_active: model.is_native,
                        connect_active_notify[sender] => move |switch| {
                            sender.input(CreateWineProfileAppMsg::SetNative(switch.is_active()));
                        }
                    },

                    adw::ActionRow {
                        set_title: "Wine",
                        #[watch]
                        set_visible: !model.is_native,

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
                        #[watch]
                        set_visible: !model.is_native,

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

                    #[template]
                    ComboSwitchRow {
                        #[watch]
                        set_visible: !model.is_native,
                        set_title: "Synchronization",
                        set_subtitle: "Set the synchronization method for wine",
                        set_model: Some(&gtk::StringList::new(&["FSync", "Esync"])),
                    },

                    adw::ComboRow {
                        #[watch]
                        set_visible: !model.is_native,
                        set_title: "Language",
                        set_subtitle: "Language used in the wine environment. Can fix keyboard layout issues",
                        set_model: Some(&gtk::StringList::new(&["System", "English"]))
                    },

                    adw::SwitchRow {
                        #[watch]
                        set_visible: !model.is_native,
                        set_title: "Borderless Window",
                        set_active: false
                    },

                    #[template]
                    ComboSwitchRow {
                        #[watch]
                        set_visible: !model.is_native,
                        set_title: "Virtual Desktop",
                        set_model: Some(&gtk::StringList::new(&["1920x1080", "1280x720", "1600x900"])),
                    },

                    adw::SwitchRow {
                        #[watch]
                        set_visible: !model.is_native,
                        set_title: "Map drive C:",
                        set_subtitle: "Automatically symlink drive_c folder from the wine prefix to the dosdevices",
                        set_active: true,
                    },

                    #[template]
                    ComboSwitchRow {
                        #[watch]
                        set_visible: !model.is_native,
                        set_title: "Map game folder",
                        set_subtitle: "Automatically symlink game folder to the dosdevices",
                        set_model: Some(&gtk::StringList::new(&["a:", "b:", "c:", "d:", "e:", "f:", "g:", "h:", "i:", "j:", "k:", "l:", "m:", "n:", "o:", "p:", "q:", "r:", "s:", "t:", "u:", "v:", "w:", "x:", "y:", "z:"])),
                    },

                    adw::SwitchRow {
                        #[watch]
                        set_visible: !model.is_native,
                        set_title: "Use wine shared libraries",
                        set_subtitle: "Set LD_LIBRARY_PATH variable to load system libraries from selected wine build",
                        set_active: true,
                    },

                    adw::SwitchRow {
                        #[watch]
                        set_visible: !model.is_native,
                        set_title: "Use gstreamer shared libraries",
                        set_subtitle: "Set GST_PLUGIN_PATH variable to load gstreamer libraries from selected wine build",
                        set_active: true,
                    },
                },

                add = &adw::PreferencesGroup {
                    set_title: "DXVK",
                    #[watch]
                    set_visible: !model.is_native,

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

                    #[template]
                    ComboSwitchRow {
                        set_title: "HUD",
                        set_model: Some(&gtk::StringList::new(&["DXVK", "MangoHud"])),
                    },

                    #[template]
                    ComboSwitchRow {
                        set_title: "FSR",
                        set_subtitle: "Upscales game to your monitor size. To use select a lower resolution in the game's settings and press Alt+Enter",
                        set_model: Some(&gtk::StringList::new(&["Ultra Quality", "Quality", "Balanced", "Performance"])),
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
                    set_description: Some("Command used to launch the game. Placeholder %command% is generated automatically by the launcher. For example: gamemoderun '%command%'"),

                    adw::EntryRow {
                        set_title: "%command%",
                    },

                    adw::ActionRow {
                        set_title: "Environment variables",
                        add_suffix = &gtk::Image {
                            set_icon_name: Some("go-next-symbolic"),
                        },
                        set_activatable: true,
                        connect_activated => CreateWineProfileAppMsg::OpenEnvironmentPage,
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
            container_page: ComponentPage::builder().launch(()).detach(),
            environment_page: EnvironmentPage::builder().launch(()).detach(),

            // TODO: Maybe load this from a default config
            is_native: false,
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
            CreateWineProfileAppMsg::OpenEnvironmentPage => {
                self.window.push_subpage(self.environment_page.widget());
            }
            CreateWineProfileAppMsg::SetNative(state) => {
                self.is_native = state;
            }
        }
    }
}
