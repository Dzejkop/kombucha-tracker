use crate::components::{ErrorView, KombuchaPanel, KombuchaView};
use anyhow::Error;
use data_types::{EntryId, Kombucha, KombuchaId};
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

    error: Option<String>,
    selected_idx: Option<usize>,
    entries: Rc<Mutex<Vec<Kombucha>>>,
}

pub enum Msg {
    Unimplemented,
    Nop,
    AddKombucha,
    Reload,
    LoadKombuchas(Vec<Kombucha>),
    DeleteEntry(KombuchaId, EntryId),
    NewEntry(KombuchaId),
    Select(Option<usize>),
    UpdateKombucha(usize, Kombucha),
    ShowError(Error),
    CloseError,
}

impl App {
    fn add_kombucha(&mut self) {
        let req = Request::post("http://localhost:8080/api/1/kombucha")
            .body(Nothing)
            .unwrap();

        let task = self
            .fetch_service
            .fetch(
                req,
                self.link.callback(
                    |_: Response<Json<Result<KombuchaId, Error>>>| Msg::Reload,
                ),
            )
            .unwrap();

        self.jobs.push_front(Box::new(task));
    }

    fn delete_entry(&mut self, id: KombuchaId, entry_id: EntryId) {
        let url = format!(
            "http://localhost:8080/api/1/kombucha/{}/entry/{}",
            id, entry_id
        );
        let req = Request::delete(url).body(Nothing).unwrap();

        let task = self
            .fetch_service
            .fetch(
                req,
                self.link.callback(
                    |_: Response<Json<Result<KombuchaId, Error>>>| Msg::Reload,
                ),
            )
            .unwrap();

        self.jobs.push_front(Box::new(task));
    }

    fn new_entry(&mut self, id: KombuchaId) {
        let url = format!("http://localhost:8080/api/1/kombucha/{}/entry", id);
        let req = Request::post(url).body(Nothing).unwrap();

        let task = self
            .fetch_service
            .fetch(
                req,
                self.link.callback(
                    |_: Response<Json<Result<KombuchaId, Error>>>| Msg::Reload,
                ),
            )
            .unwrap();

        self.jobs.push_front(Box::new(task));
    }

    fn get_kombuchas(&mut self) {
        let req = Request::get("http://localhost:8080/api/1/kombucha/all")
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

    fn update_kombucha(&mut self, kombucha: &Kombucha) {
        let req = Request::put("http://localhost:8080/api/1/kombucha")
            .header("content-type", "application/json")
            .body(Json(&kombucha))
            .unwrap();

        let task = self
            .fetch_service
            .fetch(
                req,
                self.link.callback(
                    |response: Response<Result<String, Error>>| {
                        match response.into_body() {
                            Ok(data) => log::info!("Got response {}", data),
                            Err(err) => log::error!("Got error {}", err),
                        };

                        Msg::Nop
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
            Msg::Reload => {
                drop(entries);
                self.get_kombuchas();
            }
            Msg::Nop => return false,
            Msg::LoadKombuchas(kombuchas) => {
                *entries = kombuchas;
            }
            Msg::Select(idx) => self.selected_idx = idx,
            Msg::ShowError(err) => {
                log::error!("Error: {}", err);
                self.error = Some(err.to_string());
            }
            Msg::CloseError => self.error = None,
            Msg::AddKombucha => {
                drop(entries);
                self.add_kombucha();
            }
            Msg::UpdateKombucha(idx, new_kombucha) => {
                if let Some(kombucha) = entries.get_mut(idx) {
                    *kombucha = new_kombucha.clone();
                }

                drop(entries);

                self.update_kombucha(&new_kombucha);
            }
            Msg::Unimplemented => {
                self.error = Some("Unimplemented :(".to_string());
            }
            Msg::DeleteEntry(kombucha_id, entry_id) => {
                drop(entries);
                self.delete_entry(kombucha_id, entry_id);
            }
            Msg::NewEntry(kombucha_id) => {
                drop(entries);
                self.new_entry(kombucha_id);
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
                        on_delete_entry=self.link.callback(|(kombucha_id, entry_id)| Msg::DeleteEntry(kombucha_id, entry_id))
                        on_new_entry=self.link.callback(|kombucha_id| Msg::NewEntry(kombucha_id))
                    />
                }
            } else {
                html! {}
            }
        } else {
            html! {}
        };

        let error = if let Some(err) = self.error.clone() {
            html! { <ErrorView msg=err on_close=self.link.callback(|_| Msg::CloseError)/> }
        } else {
            html! {}
        };

        html! {
            <div class="container is-fluid kombucha-container">
            <div class="columns">
                <div class="column is-one-third">
                    <KombuchaPanel
                        kombuchas=self.entries.clone()
                        on_select=self.link.callback(|idx| Msg::Select(idx))
                        on_add=self.link.callback(|_| Msg::AddKombucha)
                    />
                </div>
                <div class="column is-two-thirds">
                    { inner }
                </div>
            </div>
            { error }
            </div>
        }
    }
}
