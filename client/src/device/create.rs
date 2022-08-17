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

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub owner_id: i32,
}

pub struct DeviceForm {
    props: Props,
    link: ComponentLink<Self>,
    fetch: Option<FetchTask>,
    state_product: String,
    state_maker: String,
    state_feature: String,
}

pub enum Msg {
    Req(i32),
    Resp(Result<DeviceResponse, anyhow::Error>),
    EditProduct(String),
    EditMaker(String),
    EditFeature(String),
}


impl DeviceForm {
    fn render_form(&self, owner_id: i32) -> Html {
        let edit_prod = self.link.callback(move |e: InputData| Msg::EditProduct(e.value));
        let edit_maker = self.link.callback(move |e: InputData| Msg::EditMaker(e.value));
        let edit_feature = self.link.callback(
            move |e: ChangeData| match e {
                ChangeData::Select(elem) => Msg::EditFeature(elem.value()),
                _ => unreachable!("only used on select field"),
        });

        html! {
            <div class=classes!("device-form")>
                <div>
                    <input type="text" value={self.state_product.clone()} oninput={edit_prod} />
                </div>
                <div>
                    <input type="text" value={self.state_maker.clone()} oninput={edit_maker} />
                </div>
                <div>
                    <select onchange={edit_feature}>
                        <option value="RAM 8GB" selected=true>{ "RAM 8GB" }</option>
                        <option value="RAM 16GB" selected=true>{ "RAM 16GB" }</option>
                        <option value="RAM 32GB" selected=true>{ "RAM 32GB" }</option>
                        <option value="RAM 64GB" selected=true>{ "RAM 63GB" }</option>
                    </select>
                </div>
                <div>
                    <button onclick=self.link.callback(move |_| Msg::Req(owner_id))>{ "submit" }</button>
                </div>
            </div>
        }
    }
}


impl Component for DeviceForm {
    type Properties = Props;
    type Message = Msg;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            fetch: None,
            state_product: String::new(),
            state_maker: String::new(),
            state_feature: String::new(),
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_form(self.props.owner_id) }
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Req(id) => {
                let body = DeviceRequest {
                    product: self.state_product.clone(),
                    owner_id: id,
                    maker: self.state_maker.clone(),
                    feature: self.state_feature.clone(),
                };
                let req = Request::post("http://localhost:7878/divice")
                    .header("Content-Type", "application/json")
                    .body(Json(&body))
                    .expect("can make request to server");
                let cb = self.link.callback(
                    |response: Response<Json<Result<DeviceResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::Resp(data)
                    },
                );
                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch = Some(task);
                ()
            },
            Msg::Resp(resp) => {
                ConsoleService::info(&format!("device create: {:?}", resp));
                if let Ok(_) = resp {
                    RouteAgent::dispatcher().send(RouteRequest::ChangeRoute(Route {
                        route: format!("/app/{}", self.props.owner_id),
                        state: {},
                    }));
                }
            },
            Msg::EditProduct(input) => {
                self.state_product = input
            },
            Msg::EditMaker(input) => {
                self.state_maker = input
            },
            Msg::EditFeature(input) => {
                self.state_feature = input
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}