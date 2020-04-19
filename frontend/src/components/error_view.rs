use yew::prelude::*;

pub struct ErrorView {
    link: ComponentLink<Self>,
    props: Props,
}

#[derive(Clone, Properties)]
pub struct Props {
    #[prop_or("Something went wrong :(".to_string())]
    pub msg: String,
    #[prop_or_default]
    pub on_close: Callback<()>,
}

impl Component for ErrorView {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        self.props.on_close.emit(());

        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <div class="kombucha-error-bg"></div>
                <div class="kombucha-error message is-danger">
                    <div class="message-header">
                        <p>{"Danger"}</p>
                        <button
                            class="delete"
                            aria-label="delete"
                            onclick=self.link.callback(|_| ())
                        >
                        </button>
                    </div>
                    <div class="kombucha-error-msg message-body">
                        { &self.props.msg }
                    </div>
                </div>
            </div>
        }
    }
}
