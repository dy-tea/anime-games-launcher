use adw::prelude::*;

use relm4::{prelude::*, factory::*};

use crate::ui::windows::profiles_window::*;

#[derive(Debug)]
struct Profile {
    name: String,
    subtitle: String,
    check_button: gtk::CheckButton,
}

#[relm4::factory(async)]
impl AsyncFactoryComponent for Profile {
    type Init = Profile;
    type Input = ProfilePageAppMsg;
    type Output = ProfilePageAppMsg;
    type ParentWidget = adw::PreferencesGroup;
    type CommandOutput = ();

    view! {
        #[root]
        adw::ActionRow {
            set_title: &self.name,
            set_subtitle: &self.subtitle,
            add_prefix: &self.check_button.clone(),
            set_activatable: true,

            set_tooltip_text: Some("Set default profile"),

            add_suffix = &gtk::Button {
                set_align: gtk::Align::Center,
                add_css_class: "circular",
                set_icon_name: "edit-symbolic",
                set_tooltip_text: Some("Edit profile"),
            },

            add_suffix = &gtk::Button {
                set_align: gtk::Align::Center,
                add_css_class: "circular",
                set_icon_name: "user-trash-symbolic",
                set_tooltip_text: Some("Delete profile"),
                connect_clicked[sender, index] => move |_| {
                    sender.output(ProfilePageAppMsg::Delete(index.current_index())).unwrap();
                }
            },


            connect_activated[sender, index] => move |_| {
                sender.output(ProfilePageAppMsg::SetDefault(index.current_index())).unwrap();
            },
        }
    }

    async fn init_model(init: Self::Init, index: &DynamicIndex, _sender: AsyncFactorySender<Self>) -> Self {
        init
    }
}

#[derive(Debug, Clone)]
pub enum ProfilePageAppMsg {
    New,
    Delete(usize),
    SetDefault(usize),
}

#[derive(Debug)]
pub struct ProfilePageApp {
    profile_window: Option<AsyncController<CreateWineProfileApp>>,
    profiles_root_widget: gtk::CheckButton,
    profiles: AsyncFactoryVecDeque<Profile>,
}

#[relm4::component(pub, async)]
impl SimpleAsyncComponent for ProfilePageApp {
    type Init = ();
    type Input = ProfilePageAppMsg;
    type Output = ();

    view! {
        #[root]
        adw::PreferencesPage {
            add = &adw::PreferencesGroup {
                set_title: "Profiles",

                #[wrap(Some)]
                set_header_suffix = &gtk::Button {
                    set_align: gtk::Align::Center,
                    add_css_class: "flat",
                    set_icon_name: "list-add-symbolic",
                    set_tooltip_text: Some("Create new profile"),
                    connect_clicked => ProfilePageAppMsg::New,
                },

                model.profiles.widget(),
            }
        }
    }

    async fn init(_init: Self::Init, root: Self::Root, sender: AsyncComponentSender<Self>) -> AsyncComponentParts<Self> {
        let mut model = Self {
            profile_window: None,
            profiles_root_widget: gtk::CheckButton::new(),
            profiles: AsyncFactoryVecDeque::builder().launch_default().forward(sender.input_sender(), std::convert::identity),
        };

        // Test profiles
        let profiles = [
            (String::from("Default"), String::from("Wine-Staging-TkG 9.0 ∙ DXVK 2.1")),
            (String::from("Profile2"), String::from("Wine-Staging-TkG 8.1 ∙ DXVK 1.8")),
            (String::from("Native"), String::from("Linux Native"))
        ];

        for (name, subtitle) in profiles {
            let check_button = gtk::CheckButton::new();
            check_button.set_group(Some(&model.profiles_root_widget));

            if name == "Default" {
                check_button.set_active(true);
            }

            model.profiles.guard().push_back(Profile {
                name,
                subtitle,
                check_button,
            });
        }

        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update(&mut self, msg: Self::Input, sender: AsyncComponentSender<Self>) {
        let mut guard = self.profiles.guard();

        match msg {
            ProfilePageAppMsg::New => {
                self.profile_window = Some(CreateWineProfileApp::builder().launch(()).detach());
                if let Some(window) = &self.profile_window {
                    window.widget().present();
                }
            }

            ProfilePageAppMsg::Delete(index) => {
                let active = if let Some(profile) = guard.get(index) {
                    profile.check_button.is_active()
                } else { false };

                if guard.len() > 1 {
                    guard.remove(index);

                    // Set the first profile as default if the deleted profile was the default one
                    if active {
                        sender.input(ProfilePageAppMsg::SetDefault(0));
                    }
                }
            }

            ProfilePageAppMsg::SetDefault(index) => {
                if let Some(profile) = guard.get(index) {
                    profile.check_button.set_active(true);
                }
            }
        }
    }
}
