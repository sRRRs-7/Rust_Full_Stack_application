#![recursion_limit = "256"]

mod owner;
mod device;
mod detail;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::{components::RouterAnchor, router::Router, Switch};

pub type Anchor = RouterAnchor<AppRoute>;

struct Client {}    // entry point

pub enum Msg {}

#[derive(Switch, Clone, Debug, PartialEq)]
pub enum AppRoute {
    #[to = "/"]
    Home,
    #[to = "/app/{id}"]
    Detail(i32),
    #[to = "/app/create-owner"]
    CreateOwner,
    #[to = "/app/create-device/{i32}"]
    CreateDevice(i32),
}

impl Component for Client {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class={classes!("app")}>
                <div class={classes!("nav")}>
                    <Anchor route={AppRoute::Home}>
                        { "Home" }
                    </Anchor>
                </div>
                <div class={classes!("content")}>
                    <Router<AppRoute, ()>
                        render = Router::render(move |switch: AppRoute| -> Html {
                            match switch {
                                AppRoute::Home => {
                                    html! {
                                        <div>
                                            <owner::list::List />
                                            <br />
                                            <Anchor route=AppRoute::CreateOwner>
                                                {"create new owner"}
                                            </Anchor>
                                        </div>
                                    }
                                },
                                AppRoute::Detail(owner_id) => {
                                    html! {
                                        <div>
                                            <detail::Detail owner_id=owner_id />
                                        </div>
                                    }
                                },
                                AppRoute::CreateOwner => {
                                    html! {
                                        <div>
                                            <owner::create::CreateForm />
                                        </div>
                                    }
                                },
                                AppRoute::CreateDevice(owner_id) => {
                                    html! {
                                        <div>
                                            <device::create::DeviceForm owner_id=owner_id />
                                        </div>
                                    }
                                },
                            }
                        })
                    />
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Client>::new().mount_to_body();
}