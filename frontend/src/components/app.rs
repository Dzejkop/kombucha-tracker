use crate::components::{KombuchaPanel, KombuchaView};
use crate::data::Kombucha;
use anyhow::Error;
use std::{collections::VecDeque, rc::Rc, sync::Mutex};
use yew::{
    format::Nothing,
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

    kombucha_form_name: String,
    selected_idx: Option<usize>,
    entries: Rc<Mutex<Vec<Kombucha>>>,
}

pub enum Msg {
    Nop,
    AddKombucha,
    Select(Option<usize>),
    UpdateKombucha(usize, Kombucha),
    ShowError(Error),
}

impl App {
    pub fn do_something(&mut self) {
        let req = Request::get("http://localhost:8080/")
            .body(Nothing)
            .unwrap();
        let task = self
            .fetch_service
            .fetch(
                req,
                self.link.callback(
                    |response: Response<Result<String, Error>>| match response
                        .into_body()
                    {
                        Ok(content) => {
                            log::trace!("{}", content);
                            Msg::Nop
                        }
                        Err(error) => Msg::ShowError(error),
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
        Self {
            link,
            entries: Rc::new(Mutex::new(vec![
                Kombucha::default_new(),
                Kombucha::test_default(),
                Kombucha::default_new(),
                Kombucha::test_default(),
            ])),
            ..Self::default()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        self.jobs = self.jobs.drain(..).filter(|job| job.is_active()).collect();
        let mut entries = self.entries.lock().unwrap();

        match msg {
            Msg::Nop => return false,
            Msg::Select(idx) => self.selected_idx = idx,
            Msg::ShowError(err) => log::error!("Error: {}", err),
            Msg::AddKombucha => {
                if self.kombucha_form_name.is_empty() {
                    return false;
                }
                entries.push(Kombucha::default_new());

                self.kombucha_form_name.clear();
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
