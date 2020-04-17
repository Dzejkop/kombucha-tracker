use crate::data::{Entry, Kombucha};
use yew::prelude::*;

pub enum Msg {
    Nop,
    UpdateName(String),
    StartEditingSection(usize),
    StopEditingSection,
    UpdateSectionText(usize, String),
    NewSection,
    DeleteSection(usize),
    StartEditingName,
    StopEditingName,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub kombucha: Kombucha,
    pub on_change: Callback<Kombucha>,
}

pub struct KombuchaView {
    link: ComponentLink<Self>,
    is_editing_name: bool,
    edited_entry: Option<usize>,
    kombucha: Kombucha,
    on_change: Callback<Kombucha>,
}

impl Component for KombuchaView {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let Props {
            on_change,
            kombucha,
        } = props;

        Self {
            link,
            kombucha,
            on_change,
            is_editing_name: false,
            edited_entry: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Nop => return true,
            Msg::UpdateName(new_name) => {
                self.kombucha.name = new_name;
            }
            Msg::StartEditingSection(idx) => match self.edited_entry {
                None => {
                    if idx < self.kombucha.entries.len() {
                        self.edited_entry = Some(idx);
                    }
                }
                Some(_) => {
                    self.edited_entry = Some(idx);
                }
            },
            Msg::StopEditingSection => {
                self.edited_entry = None;
                self.on_change.emit(self.kombucha.clone());
            }
            Msg::UpdateSectionText(idx, new_text) => {
                if let Some(section) = self.kombucha.entries.get_mut(idx) {
                    section.text = new_text;
                }
            }
            Msg::NewSection => {
                self.kombucha.entries.push(Entry::default());
                self.edited_entry = Some(self.kombucha.entries.len() - 1);
            }
            Msg::DeleteSection(idx) => {
                if idx < self.kombucha.entries.len() {
                    self.kombucha.entries.remove(idx);
                    self.on_change.emit(self.kombucha.clone());
                }
            }
            Msg::StartEditingName => {
                self.is_editing_name = true;
            }
            Msg::StopEditingName => {
                self.is_editing_name = false;
                self.on_change.emit(self.kombucha.clone());
            }
        }

        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let Props {
            on_change,
            kombucha,
        } = props;

        self.on_change = on_change;
        self.kombucha = kombucha;

        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="card kombucha-panel">
                <div class="card-content">
                    <div class="media">
                        <div class="media-content">
                            { self.view_name() }
                        </div>
                    </div>
                    <div class="content">
                        <div class="kombucha-entries">
                            { self.view_entries() }
                        </div>
                        <hr />
                        <div class="field is-grouped is-grouped-centered">
                            <p class="control">
                                <button
                                    class="button is-info"
                                    onclick=self.link.callback(|_| Msg::NewSection)
                                >
                                    <span class="icon is-large">
                                        <i class="fas fa-plus fa-2x"></i>
                                    </span>
                                </button>
                            </p>
                        </div>
                        <hr />
                        <p>
                            <p class="title is-6">{"Fermentation status"}</p>
                            <p>{"Primary"}</p>
                                <progress class="progress is-primary" value="100" max="100">{"15%"}</progress>
                            <p>{"1 Jan 2016 - 7 Jan 2016"}</p>
                            <hr />
                            <p>{"Secondary"}</p>
                                <progress class="progress is-info" value="15" max="100">{"15%"}</progress>
                            <p>{"7 Jan 2016 - 10 Jan 2016"}</p>
                        </p>
                    </div>
                </div>
            </div>
        }
    }
}

impl KombuchaView {
    fn view_name(&self) -> Html {
        if self.is_editing_name {
            html! {
                <div>
                    <input
                        class="title is-4" value=self.kombucha.name
                        onchange=self.link.callback(|e: ChangeData| match e {
                            ChangeData::Value(value) => Msg::UpdateName(value),
                            x => { log::error!("Invalid change data, expected value, found {:?}", x); Msg::Nop }
                        })
                    />
                    {"\u{00a0}"}
                    <a
                        class="icon is-small"
                        onclick=self.link.callback(|_| Msg::StopEditingName)
                    ><i class="fas fa-check"/></a>
                </div>
            }
        } else {
            html! {
                <div>
                    <span class="title is-4">{ &self.kombucha.name }</span>
                    {"\u{00a0}"}
                    <a
                        class="icon is-small"
                        onclick=self.link.callback(|_| Msg::StartEditingName)
                    ><i class="fas fa-edit"/></a>
                </div>
            }
        }
    }

    fn view_entries(&self) -> Html {
        let entries = self
            .kombucha
            .entries
            .iter()
            .enumerate()
            .map(|(idx, entry)| self.view_entry(idx, entry));

        html! { for entries }
    }

    fn view_entry(&self, idx: usize, entry: &Entry) -> Html {
        if let Some(edited_idx) = self.edited_entry {
            if edited_idx == idx {
                return self.view_edited_entry(idx, entry);
            }
        }

        self.view_regular_entry(idx, entry)
    }

    fn view_edited_entry(&self, idx: usize, entry: &Entry) -> Html {
        html! {
            <div class="kombucha-entry">
                <p>
                    <textarea
                        class="textarea"
                        value=entry.text
                        oninput=self.link.callback(move |e: InputData| Msg::UpdateSectionText(idx, e.value))
                    >
                        { &entry.text }
                    </textarea>
                    <br />
                    <time datetime={entry.added}>{ entry.added.to_string() }</time>
                </p>
                <p class="kombucha-content-control">
                    <div class="field is-grouped is-grouped-centered">
                        <p class="control">
                            <button
                                class="button is-primary"
                                onclick=self.link.callback(|_| Msg::StopEditingSection)
                            >
                                <span class="icon is-medium">
                                <i class="fas fa-check"></i>
                                </span>
                            </button>
                        </p>
                        <p class="control">
                            <button
                                class="button is-danger"
                                onclick=self.link.callback(move |_| Msg::DeleteSection(idx))
                            >
                                <span class="icon is-medium">
                                <i class="fas fa-trash"></i>
                                </span>
                            </button>
                        </p>
                    </div>
                </p>
            </div>
        }
    }

    fn view_regular_entry(&self, idx: usize, entry: &Entry) -> Html {
        html! {
            <div class="kombucha-entry">
                <p>
                    { &entry.text }
                    <br />
                    <time datetime={entry.added}>{ entry.added.to_string() }</time>
                </p>
                <p class="kombucha-content-control">
                    <div class="field is-grouped is-grouped-centered">
                        <p class="control">
                            <button
                                class="button"
                                onclick=self.link.callback(move |_| Msg::StartEditingSection(idx))
                            >
                                <span class="icon is-medium">
                                <i class="fas fa-edit"></i>
                                </span>
                            </button>
                        </p>
                        <p class="control">
                            <button
                                class="button is-danger"
                                onclick=self.link.callback(move |_| Msg::DeleteSection(idx))
                            >
                                <span class="icon is-medium">
                                <i class="fas fa-trash"></i>
                                </span>
                            </button>
                        </p>
                    </div>
                </p>
            </div>
        }
    }
}
