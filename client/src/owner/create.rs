use common::*;
use yew::prelude::*;
use yew::format::Json;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
    ConsoleService,
};
use yew_router::{
    agent::{RouteAgent, RouteRequest},
    prelude::*,
};

pub struct CreateForm {
    link: ComponentLink<Self>,
    fetch: Option<FetchTask>,
    state_name: String,
}

pub enum Msg {
    Req,
    Resp(Result<OwnerResponse, anyhow::Error>),
    EditName(String),
}

impl CreateForm {
    fn render_form(&self) -> Html {
        let edit_name = self.link.callback(move |e: InputData| Msg::EditName(e.value));
        html! {
            <div class={classes!("owner-form")}>
                <div>
                    <input type="text" value={self.state_name.clone()} oninput={edit_name} />
                </div>
                <div>
                    <button onclick=self.link.callback(move |_| Msg::Req)>
                        { "submit" }
                    </button>
                </div>
            </div>
        }
    }
}


impl Component for CreateForm {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch: None,
            state_name: String::new(),
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_form() }
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Req => {
                let body = OwnerRequest {
                    name: self.state_name.clone(),
                };
                let req = Request::post("http://localhost:7878/owner")
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("can make request to server");
                let cb = self.link.callback(
                    |response: Response<Json<Result<OwnerResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );
                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch = Some(task);
                ()
            },
            Msg::Resp(resp) => {
                ConsoleService::info(&format!("owner create: {:?}", resp));
                if let Ok(_) = resp {
                    RouteAgent::dispatcher().send(RouteRequest::ChangeRoute(Route {
                        route: "/".to_string(),
                        state: (),
                    }));
                }
            }
            Msg::EditName(input) => {
                self.state_name = input;
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }
}