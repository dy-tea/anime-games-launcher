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
                },

                add = &adw::PreferencesGroup {
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
