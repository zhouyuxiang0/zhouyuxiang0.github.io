#![recursion_limit = "1024"]
use yew::{html, Component, ComponentLink, Html, ShouldRender};

mod header;
mod main_component;
mod blog;
use header::Header;
use main_component::MainComponent;

struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        App {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        unimplemented!()
    }

    fn view(&self) -> Html {
        html! {
            <>
                <Header />
                <MainComponent />
            </>
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
