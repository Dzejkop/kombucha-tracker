use crate::data::{Fermentation, Kombucha};
use yew::prelude::*;

pub enum Msg {
    Nop,
    UpdateName(String),
    ChangeFermentation(Fermentation),
    Edit,
    StopEdit,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub data: Kombucha,
    pub on_change: Callback<Kombucha>,
}

pub struct KombuchaView {
    link: ComponentLink<Self>,
    is_being_edited: bool,
    data: Kombucha,
    on_change: Callback<Kombucha>,
}

impl Component for KombuchaView {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let Props { on_change, data } = props;

        Self {
            link,
            is_being_edited: false,
            data,
            on_change,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Nop => return false,
            Msg::UpdateName(new_name) => self.data.name = new_name,
            Msg::ChangeFermentation(new_fermentation) => {
                self.data.status = new_fermentation
            }
            Msg::Edit => self.is_being_edited = true,
            Msg::StopEdit => {
                self.is_being_edited = false;
                self.on_change.emit(self.data.clone());
            },
        }

        true
    }

    fn view(&self) -> Html {
        if self.is_being_edited {
            html! {
                <div>
                    <p>{ "Editing" }</p>
                    <input
                        oninput=self.link.callback(move |e: InputData| Msg::UpdateName(e.value))
                        value=self.data.name
                    />
                    <select
                        onchange=self.link.callback(move |e: ChangeData| {
                            if let ChangeData::Select(elem) = e {
                                if let Ok(fermentation) = elem.value().parse::<Fermentation>() {
                                    return Msg::ChangeFermentation(fermentation);
                                }
                            }

                            Msg::Nop
                        })
                    >
                        <option>{ Fermentation::Primary }</option>
                        <option>{ Fermentation::Secondary }</option>
                    </select>
                    <button onclick=self.link.callback(move |_| Msg::StopEdit)>{ "-" }</button>
                </div>
            }
        } else {
            html! {
                <div>
                    <p>{ &self.data.name }</p>
                    <p>{ format!("Added on: {}", self.data.added.to_string()) }</p>
                    <p>{ format!("Status: {}", self.data.status) }</p>
                    <button onclick=self.link.callback(move |_| Msg::Edit)>{ "+" }</button>
                </div>
            }
        }
    }
}
