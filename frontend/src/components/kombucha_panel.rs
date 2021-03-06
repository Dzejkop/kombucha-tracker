use crate::data::Kombucha;
use std::{rc::Rc, sync::Mutex};
use yew::prelude::*;

pub enum Msg {
    Nop,
    AddKombucha,
    UpdateSearchString(String),
    Select(usize),
}

#[derive(Clone, Properties)]
pub struct Props {
    pub kombuchas: Rc<Mutex<Vec<Kombucha>>>,
    #[prop_or_default]
    pub on_select: Callback<Option<usize>>,
    #[prop_or_default]
    pub on_add: Callback<()>,
}

#[derive(Default)]
pub struct KombuchaPanel {
    link: ComponentLink<Self>,
    search_string: String,
    selected_kombucha: Option<usize>,
    kombuchas: Rc<Mutex<Vec<Kombucha>>>,
    on_select: Callback<Option<usize>>,
    on_add: Callback<()>,
}

impl Component for KombuchaPanel {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let Props {
            kombuchas,
            on_select,
            on_add,
        } = props;
        Self {
            link,
            kombuchas,
            on_select,
            on_add,
            ..Default::default()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Nop => (),
            Msg::UpdateSearchString(s) => {
                self.search_string = s;
            }
            Msg::AddKombucha => self.on_add.emit(()),
            Msg::Select(idx) => {
                let select_idx = match self.selected_kombucha {
                    None => {
                        self.selected_kombucha = Some(idx);
                        Some(idx)
                    }
                    Some(old_idx) => {
                        if old_idx == idx {
                            self.selected_kombucha = None;
                            None
                        } else {
                            self.selected_kombucha = Some(idx);
                            Some(idx)
                        }
                    }
                };
                self.on_select.emit(select_idx);
            }
        }

        true
    }

    fn view(&self) -> Html {
        html! {
            <nav class="panel kombucha-panel">
                <p class="panel-heading">
                    {"My Kombuchas"}
                </p>
                <div class="panel-block">
                    <div class="control has-icons-left">
                        <input
                            class="input"
                            type="text"
                            value=self.search_string
                            placeholder="Search"
                            oninput=self.link.callback(|e: InputData| Msg::UpdateSearchString(e.value))
                        />
                        <span class="icon is-left">
                            <i class="fas fa-search" aria-hidden="true"></i>
                        </span>
                    </div>
                </div>
                <div class="">
                    { self.view_kombucha_entries() }
                </div>
                <div class="panel-block">
                    <button
                        class="button is-link is-outlined is-fullwidth"
                        onclick=self.link.callback(|_| Msg::AddKombucha)
                    >
                        <i class="fa fa-plus fa-lg" aria-hidden="true"></i>
                    </button>
                </div>
            </nav>
        }
    }
}

impl KombuchaPanel {
    fn view_kombucha_entries(&self) -> Html {
        let kombuchas = self.kombuchas.lock().unwrap();

        let kombuchas = kombuchas.iter().enumerate();

        if self.search_string.is_empty() {
            kombuchas
                .map(|(idx, kombucha)| self.view_kombucha_entry(idx, kombucha))
                .collect()
        } else {
            kombuchas
                .filter(|(_, kombucha)| {
                    kombucha.name.contains(&self.search_string)
                })
                .map(|(idx, kombucha)| self.view_kombucha_entry(idx, kombucha))
                .collect()
        }
    }

    fn view_kombucha_entry(&self, idx: usize, kombucha: &Kombucha) -> Html {
        let class = match self.selected_kombucha {
            Some(selected_idx) if selected_idx == idx => {
                "panel-block is-active"
            }
            _ => "panel-block",
        };

        html! {
            <a
                class={ class }
                onclick=self.link.callback(move |_| Msg::Select(idx))
            >
                <span class="panel-icon"><i class="fas fa-coffee" aria-hidden="true"></i></span>
                { &kombucha.name }
            </a>
        }
    }
}
