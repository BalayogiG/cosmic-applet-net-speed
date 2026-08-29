use std::sync::LazyLock;
use std::time::Duration;

use cosmic::app::{Core, Task};
use cosmic::iced::core::window;
use cosmic::iced::stream;
use cosmic::iced::window::Id;
use cosmic::iced::{Rectangle, Subscription};
use cosmic::surface::action::{app_popup, destroy_popup};
use cosmic::widget::{autosize, list_column, settings, text};
use cosmic::Element;
use futures::SinkExt;

use crate::net;

const ID: &str = "io.github.balayogig.cosmic-ext-applet-net-speed";

static AUTOSIZE_ID: LazyLock<cosmic::widget::Id> =
    LazyLock::new(|| cosmic::widget::Id::new("cosmic-applet-net-speed-autosize"));

pub struct Window {
    core: Core,
    popup: Option<Id>,
    iface: Option<String>,
    last_bytes: Option<(u64, u64)>,
    down_bps: u64,
    up_bps: u64,
}

impl Default for Window {
    fn default() -> Self {
        let iface = net::active_interface();
        let last_bytes = iface.as_deref().and_then(net::rx_tx_bytes);

        Self {
            core: Core::default(),
            popup: None,
            iface,
            last_bytes,
            down_bps: 0,
            up_bps: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Message {
    PopupClosed(Id),
    Surface(cosmic::surface::Action),
    Tick,
}

impl cosmic::Application for Window {
    type Executor = cosmic::SingleThreadExecutor;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = ID;

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: Self::Flags) -> (Self, Task<Message>) {
        let window = Window {
            core,
            ..Default::default()
        };
        (window, Task::none())
    }

    fn on_close_requested(&self, id: window::Id) -> Option<Message> {
        Some(Message::PopupClosed(id))
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::PopupClosed(id) => {
                if self.popup.as_ref() == Some(&id) {
                    self.popup = None;
                }
            }
            Message::Surface(a) => {
                return cosmic::task::message(cosmic::Action::Cosmic(
                    cosmic::app::Action::Surface(a),
                ));
            }
            Message::Tick => {
                if self.iface.is_none() {
                    self.iface = net::active_interface();
                    self.last_bytes = self.iface.as_deref().and_then(net::rx_tx_bytes);
                }

                if let Some(iface) = self.iface.clone() {
                    match net::rx_tx_bytes(&iface) {
                        Some((rx, tx)) => {
                            if let Some((last_rx, last_tx)) = self.last_bytes {
                                self.down_bps = rx.saturating_sub(last_rx);
                                self.up_bps = tx.saturating_sub(last_tx);
                            }
                            self.last_bytes = Some((rx, tx));
                        }
                        None => {
                            self.iface = None;
                            self.last_bytes = None;
                            self.down_bps = 0;
                            self.up_bps = 0;
                        }
                    }
                }
            }
        };
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::run(|| {
            stream::channel(1, |mut output: futures::channel::mpsc::Sender<Message>| async move {
                let mut interval = tokio::time::interval(Duration::from_secs(1));
                loop {
                    interval.tick().await;
                    if output.send(Message::Tick).await.is_err() {
                        break;
                    }
                }
            })
        })
    }

    fn view(&self) -> Element<'_, Message> {
        let have_popup = self.popup.clone();
        let label = format!(
            "↓{} ↑{}",
            net::format_speed(self.down_bps),
            net::format_speed(self.up_bps)
        );

        let btn = self
            .core
            .applet
            .text_button(
                self.core
                    .applet
                    .text(label)
                    .wrapping(cosmic::iced::core::text::Wrapping::None),
                Message::Tick,
            )
            .on_press_with_rectangle(move |offset, bounds| {
                if let Some(id) = have_popup {
                    Message::Surface(destroy_popup(id))
                } else {
                    Message::Surface(app_popup::<Window>(
                        |_| Default::default(),
                        move |state: &mut Window| {
                            let new_id = Id::unique();
                            state.popup = Some(new_id);
                            let mut popup_settings = state.core.applet.get_popup_settings(
                                state.core.main_window_id().unwrap(),
                                new_id,
                                None,
                                None,
                                None,
                            );

                            popup_settings.positioner.anchor_rect = Rectangle {
                                x: (bounds.x - offset.x) as i32,
                                y: (bounds.y - offset.y) as i32,
                                width: bounds.width as i32,
                                height: bounds.height as i32,
                            };

                            popup_settings
                        },
                        Some(Box::new(move |state: &Window| {
                            let content_list = list_column()
                                .add(settings::item(
                                    "Interface",
                                    text(state.iface.clone().unwrap_or_else(|| "none".into())),
                                ))
                                .add(settings::item(
                                    "Download",
                                    text(net::format_speed(state.down_bps)),
                                ))
                                .add(settings::item(
                                    "Upload",
                                    text(net::format_speed(state.up_bps)),
                                ));
                            Element::from(state.core.applet.popup_container(content_list))
                                .map(cosmic::Action::App)
                        })),
                    ))
                }
            });

        let tooltip = Element::from(self.core.applet.applet_tooltip::<Message>(
            btn,
            "Network speed",
            self.popup.is_some(),
            Message::Surface,
            None,
        ));

        autosize::autosize(tooltip, AUTOSIZE_ID.clone()).into()
    }

    fn view_window(&self, _id: Id) -> Element<'_, Message> {
        "oops".into()
    }

    fn style(&self) -> Option<cosmic::iced::theme::Style> {
        Some(cosmic::applet::style())
    }
}
