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
            <div class="container is-fluid kombucha-container">
            <div class="columns">
                <div class="column is-one-third">
                    <nav class="panel kombucha-panel">
                        <p class="panel-heading">
                            {"My Kombuchas"}
                        </p>
                        <div class="panel-block">
                            <div class="control has-icons-left">
                                <input class="input" type="text" placeholder="Search" />
                                <span class="icon is-left">
                                    <i class="fas fa-search" aria-hidden="true"></i>
                                </span>
                            </div>
                        </div>
                        <div class="">
                            <a class="panel-block is-active">
                                <span class="panel-icon">
                                <i class="fas fa-coffee" aria-hidden="true"></i>
                                </span>
                                {"Banana 7 day brew"}
                            </a>
                            {
                                for (0..3).map(|_| html! {
                                    <a class="panel-block">
                                        <span class="panel-icon">
                                        <i class="fas fa-coffee" aria-hidden="true"></i>
                                        </span>
                                        {"New 3l"}
                                    </a>
                                })
                            }

                        </div>
                        <div class="panel-block">
                            <button class="button is-link is-outlined is-fullwidth">
                                <i class="fa fa-plus fa-lg" aria-hidden="true"></i>
                            </button>
                        </div>
                    </nav>
                </div>
                <div class="column is-two-thirds">
                    <div class="card kombucha-panel">
                        <div class="card-content">
                            <div class="media">
                                <div class="media-content">
                                    <p class="title is-4">{"Banana 7 day brew"}</p>
                                </div>
                            </div>

                            <div class="content">
                                {"Lorem ipsum dolor sit amet, consectetur adipiscing elit.
                                Phasellus nec iaculis mauris."}
                                <a>{"@bulmaio"}</a>{". "}
                                <a href="#">{"#css"}</a>
                                <a href="#">{"#responsive"}</a>
                                <br />
                                <time datetime="2016-1-1">{"11:09 PM - 1 Jan 2016"}</time>
                                <p>
                                <hr />
                                <p class="title is-6">{"Fermentation status"}</p>
                                <p>{"Primary"}</p>
                                <progress class="progress is-primary" value="100" max="100">{"15%"}</progress>
                                <p>{"Secondary"}</p>
                                <progress class="progress is-info" value="15" max="100">{"15%"}</progress>
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
                // <div class="new-kombucha-form">
                //     <p>{ "Add new kombucha" }</p>
                //     <div>
                //         <p>{ "Name" }</p>
                //         <input
                //             oninput=self.link.callback(|e: InputData| Msg::UpdateNewKombuchaName(e.value))
                //             value=self.state.kombucha_form_name
                //         />
                //         <br />
                //         <button onclick=self.link.callback(|_| Msg::AddKombucha)>{ "Add" }</button>
                //     </div>
                // </div>
                // <div>
                //     {
                //         for self.state.entries
                //             .iter()
                //             .enumerate()
                //             .map(|(idx, kombucha)| html! {
                //                 <KombuchaView
                //                     data = { kombucha.clone() }
                //                     on_change = { self.link.callback(move |value| Msg::UpdateKombucha(idx, value))}
                //                 />
                //             })
                //     }
                // </div>
            </div>
            </div>
        }
    }
}
