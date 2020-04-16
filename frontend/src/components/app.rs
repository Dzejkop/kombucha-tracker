use crate::components::KombuchaView;
use crate::data::{Fermentation, Kombucha};
use chrono::Utc;
use yew::prelude::*;

#[derive(Default)]
pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

#[derive(Default)]
pub struct State {
    kombucha_form_name: String,
    entries: Vec<Kombucha>,
}

pub enum Msg {
    AddKombucha,
    UpdateNewKombuchaName(String),
    UpdateKombucha(usize, Kombucha),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            ..Self::default()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddKombucha => {
                if self.state.kombucha_form_name.is_empty() {
                    return false;
                }

                self.state.entries.push(Kombucha {
                    name: self.state.kombucha_form_name.clone(),
                    added: Utc::now(),
                    status: Fermentation::Primary,
                });

                self.state.kombucha_form_name.clear();
            }
            Msg::UpdateNewKombuchaName(name) => {
                self.state.kombucha_form_name = name;
            }
            Msg::UpdateKombucha(idx, new_kombucha) => {
                if let Some(kombucha) = self.state.entries.get_mut(idx) {
                    *kombucha = new_kombucha;
                }
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div class="new-kombucha-form">
                    <p>{ "Add new kombucha" }</p>
                    <div>
                        <p>{ "Name" }</p>
                        <input
                            oninput=self.link.callback(|e: InputData| Msg::UpdateNewKombuchaName(e.value))
                            value=self.state.kombucha_form_name
                        />
                        <br />
                        <button onclick=self.link.callback(|_| Msg::AddKombucha)>{ "Add" }</button>
                    </div>
                </div>
                <div>
                    {
                        for self.state.entries
                            .iter()
                            .enumerate()
                            .map(|(idx, kombucha)| html! {
                                <KombuchaView
                                    data = { kombucha.clone() }
                                    on_change = { self.link.callback(move |value| Msg::UpdateKombucha(idx, value))}
                                />
                            })
                    }
                </div>
            </div>
        }
    }
}
