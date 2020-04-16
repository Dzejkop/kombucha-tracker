use crate::fermentation::Fermentation;
use chrono::{DateTime, Utc};
use yew::prelude::*;

#[derive(Default)]
pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

struct Kombucha {
    name: String,
    added: DateTime<Utc>,
    status: Fermentation,
}

#[derive(Default)]
pub struct State {
    kombucha_form_name: String,
    entries: Vec<Kombucha>,
    sub_state: SubState,
}

impl State {
    fn is_editing(&self) -> bool {
        matches!(self.sub_state, SubState::EditingEntry { .. })
    }

    fn is_editing_idx(&self, index: usize) -> bool {
        if let SubState::EditingEntry { idx, .. } = self.sub_state {
            index == idx
        } else {
            false
        }
    }
}

pub enum SubState {
    Base,
    EditingEntry { idx: usize },
}

impl Default for SubState {
    fn default() -> Self {
        SubState::Base
    }
}

pub enum Msg {
    AddKombucha,
    KombuchaMsg(KombuchaMsg),
}

pub enum KombuchaMsg {
    UpdateName(String),
    Edit(usize),
    EditName(usize, String),
    StopEdit(usize),
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
            Msg::KombuchaMsg(msg) => match msg {
                KombuchaMsg::UpdateName(name) => {
                    self.state.kombucha_form_name = name;
                }
                KombuchaMsg::Edit(idx) => {
                    if self.state.is_editing() {
                        return false;
                    }

                    if self.state.entries.len() > idx {
                        self.state.sub_state = SubState::EditingEntry { idx };
                    } else {
                        return false;
                    }
                }
                KombuchaMsg::EditName(idx, name) => {
                    if let Some(kombucha) = self.state.entries.get_mut(idx) {
                        kombucha.name = name;
                    }
                }
                KombuchaMsg::StopEdit(idx) => {
                    if self.state.is_editing_idx(idx) {
                        self.state.sub_state = SubState::Base;
                    }
                }
            },
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
                            oninput=self.link.callback(|e: InputData| Msg::KombuchaMsg(KombuchaMsg::UpdateName(e.value)))
                            value=self.state.kombucha_form_name
                        />
                        <br />
                        <button onclick=self.link.callback(|_| Msg::AddKombucha)>{ "Add" }</button>
                    </div>
                </div>
                <div>
                    { for self.state.entries.iter().enumerate().map(|(idx, k)| self.view_kombucha(idx, k)) }
                </div>
            </div>
        }
    }
}

impl App {
    fn view_kombucha(&self, idx: usize, kombucha: &Kombucha) -> Html {
        if self.state.is_editing_idx(idx) {
            html! {
                <div>
                    <p>{ "Editing" }</p>
                    <input
                        oninput=self.link.callback(move |e: InputData| Msg::KombuchaMsg(KombuchaMsg::EditName(idx, e.value)))
                        value=kombucha.name
                    />
                    <select>
                        <option>{ Fermentation::Primary }</option>
                        <option>{ Fermentation::Secondary }</option>
                    </select>
                    <button onclick=self.link.callback(move |_| Msg::KombuchaMsg(KombuchaMsg::StopEdit(idx)))>{ "-" }</button>
                </div>
            }
        } else {
            html! {
                <div>
                    <p>{ &kombucha.name }</p>
                    <p>{ format!("Added on: {}", kombucha.added.to_string()) }</p>
                    <p>{ format!("Status: {}", kombucha.status) }</p>
                    <button onclick=self.link.callback(move |_| Msg::KombuchaMsg(KombuchaMsg::Edit(idx)))>{ "+" }</button>
                </div>
            }
        }
    }
}
