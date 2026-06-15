mod models {
    #[derive(Default)]
    pub struct Model {
        pub address: [u8; 4],
        pub port: u16,
        pub status: String,
    }

    impl Model {
        pub fn address(&self) -> String {
            format!(
                "{}.{}.{}.{}:{}",
                self.address[0], self.address[1], self.address[2], self.address[3], self.port,
            )
        }
    }
}

use foxtk::prelude::*;
use std::{
    net::{SocketAddr, TcpStream},
    time::Duration,
};

pub enum Msg {
    Address(usize, u8),
    Status(String),
    Port(u16),
    Run,
}

#[derive(Default)]
pub struct Nmap {
    status: foxtk::Label,
}

impl Component for Nmap {
    type Event = Msg;
    type State = models::Model;
    fn handle(msg: Self::Event, model: &mut Self::State, sender: Sender<Self::Event>) -> bool {
        match msg {
            Msg::Address(idx, value) => model.address[idx] = value,
            Msg::Port(value) => model.port = value,
            Msg::Status(value) => {
                model.status = value;
                return true;
            }
            Msg::Run => {
                let address = model.address();
                if address.parse::<SocketAddr>().is_ok() {
                    std::thread::spawn({
                        let sender = sender.clone();
                        move || {
                            let value = match TcpStream::connect_timeout(
                                &address.parse::<SocketAddr>().unwrap(),
                                Duration::from_secs(8),
                            )
                            .is_ok()
                            {
                                true => "Open",
                                false => "Closed",
                            }
                            .to_string();
                            sender.send(Msg::Status(value)).unwrap();
                        }
                    });
                }
            }
        };
        false
    }
    fn update(&self, model: &Self::State) {
        self.status.set_text(&model.status);
    }
    fn view(&mut self, prt: &impl CompositeExt, sender: Sender<Self::Event>) {
        const WIDTH: i32 = 30;
        foxtk::HorizontalFrame::new(prt)
            .inside(|prt| {
                foxtk::Label::new(prt, "IP");
                for idx in 0..4 {
                    foxtk::TextField::new(prt).with_width(WIDTH).set_callback({
                        let sender = sender.clone();
                        move |wgt| {
                            if wgt.has_focus() {
                                let value = wgt.text().parse::<u8>().unwrap_or_default();
                                sender.send(Msg::Address(idx, value)).unwrap();
                            }
                            false
                        }
                    });
                }
                foxtk::Label::new(prt, ":");
                foxtk::TextField::new(prt).set_callback({
                    let sender = sender.clone();
                    move |wgt| {
                        if wgt.has_focus() {
                            let value = wgt.text().parse::<u16>().unwrap_or_default();
                            sender.send(Msg::Port(value)).unwrap();
                        }
                        false
                    }
                });
                foxtk::Button::new(prt, "Run")
                    .with_width(WIDTH)
                    .set_callback({
                        let sender = sender.clone();
                        move |_| {
                            sender.send(Msg::Run).unwrap();
                            false
                        }
                    });
                self.status = foxtk::Label::new(prt, "");
            });
    }
}
