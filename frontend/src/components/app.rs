use crate::components::{KombuchaPanel, KombuchaView};
use crate::data::Kombucha;
use anyhow::Error;
use std::{collections::VecDeque, rc::Rc, sync::Mutex};
use yew::{
    format::{Json, Nothing},
    prelude::*,
    services::{
        fetch::{Request, Response},
        FetchService, Task,
    },
};

#[derive(Default)]
pub struct App {
    fetch_service: FetchService,
    link: ComponentLink<Self>,
    jobs: VecDeque<Box<dyn Task>>,

    selected_idx: Option<usize>,
    entries: Rc<Mutex<Vec<Kombucha>>>,
}

pub enum Msg {
    Nop,
    AddKombucha,
    LoadKombuchas(Vec<Kombucha>),
    Select(Option<usize>),
    UpdateKombucha(usize, Kombucha),
    ShowError(Error),
}

impl App {
    pub fn get_kombuchas(&mut self) {
        let req = Request::get("http://localhost:8080/kombucha/all")
            .body(Nothing)
            .unwrap();

        let task = self
            .fetch_service
            .fetch(
                req,
                self.link.callback(
                    |response: Response<Json<Result<Vec<Kombucha>, Error>>>| {
                        match response.into_body().0 {
                            Ok(content) => Msg::LoadKombuchas(content),
                            Err(error) => Msg::ShowError(error),
                        }
                    },
                ),
            )
            .unwrap();

        self.jobs.push_front(Box::new(task));
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut app = Self {
            link,
            entries: Rc::new(Mutex::new(Vec::new())),
            ..Self::default()
        };

        app.get_kombuchas();

        app
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.jobs = self.jobs.drain(..).filter(|job| job.is_active()).collect();
        let mut entries = self.entries.lock().unwrap();

        match msg {
            Msg::Nop => return false,
            Msg::LoadKombuchas(kombuchas) => {
                *entries = kombuchas;
            }
            Msg::Select(idx) => self.selected_idx = idx,
            Msg::ShowError(err) => log::error!("Error: {}", err),
            Msg::AddKombucha => {
                entries.push(Kombucha {
                    id: 0.into(),
                    name: "New Kombucha".to_string(),
                    added: chrono::Utc::now(),
                    entries: vec![],
                });
            }
            Msg::UpdateKombucha(idx, new_kombucha) => {
                if let Some(kombucha) = entries.get_mut(idx) {
                    *kombucha = new_kombucha;
                }
            }
        }
        true
    }

    fn view(&self) -> Html {
        let entries = self.entries.lock().unwrap();

        let inner = if let Some(selected_idx) = self.selected_idx {
            if let Some(kombucha) = entries.get(selected_idx) {
                html! {
                    <KombuchaView
                        kombucha=kombucha
                        on_change=self.link.callback(move |kombucha| Msg::UpdateKombucha(selected_idx, kombucha))
                    />
                }
            } else {
                html! {}
            }
        } else {
            html! {}
        };

        html! {
            <div class="container is-fluid kombucha-container">
            <div class="columns">
                <div class="column is-one-third">
                    <KombuchaPanel
                        kombuchas=self.entries.clone()
                        on_select=self.link.callback(|idx| { log::info!("Selecing {:?}", idx); Msg::Select(idx) })
                        on_add=self.link.callback(|_| Msg::AddKombucha)
                    />
                </div>
                <div class="column is-two-thirds">
                    { inner }
                </div>
            </div>
            </div>
        }
    }
}
