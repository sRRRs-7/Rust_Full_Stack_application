use std::sync::Arc;

use crate::{Anchor, AppRoute};
use common::*;
use yew::prelude::*;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub owner_id: i32,
}

pub struct Detail {
    props: Props,
    link: ComponentLink<Self>,
    owner: Option<OwnerResponse>,
    devices: Option<Vec<DeviceResponse>>,
    fetch_owner: Option<FetchTask>,
    fetch_device: Option<FetchTask>,
    delete_device: Option<FetchTask>,
}

pub enum Msg {
    OwnerReq(i32),
    DeviceReq(i32),
    DeleteDeviceReq(i32),
    OwnerResp(Result<OwnerResponse, anyhow::Error>),
    DevicesResp(Result<Vec<DeviceResponse>, anyhow::Error>),
    DeleteDeviceResp(Response<Json<Result<(), anyhow::Error>>>, i32),
}


impl Detail {
    fn render_detail(&self, owner: &Option<OwnerResponse>, devices: &Option<Vec<DeviceResponse>>) -> Html {
        match owner {
            Some(o) => {
                html! {
                    <div class=classes!("detail")>
                        <h1>
                            { &o.name }
                            {"("}
                                <span class=classes!("id")> { o.id } </span>
                            {")"}
                        </h1>
                        {
                            self.view_device_list(devices)
                        }
                        <br/>
                        <Anchor route=AppRoute::CreateDevice(o.id as i32)>
                            { "create new device" }
                        </Anchor>
                    </div>
                }
            }
            None => {
                html! {
                    <div class=classes!("loading")>
                        { "loading..." }
                    </div>
                }
            }
        }
    }

    fn view_device_list(&self, devices: &Option<Vec<DeviceResponse>>) -> Html {
        match devices {
            Some(d) => {
                html! {
                    d.iter().map(|device| self.view_device(device)).collect::<Html>()
                }
            }
            None => {
                html! {
                    <div class=classes!("loading")>
                        { "loading..." }
                    </div>
                }
            }
        }
    }

    fn view_device(&self, device: &DeviceResponse) -> Html {
        let id = device.id;
        // let owner_id = self.props.owner_id;
        html! {
            <div class=classes!("list-item", "device")>
                <div>
                    <b>{ &device.product }</b>
                    {"("}
                        <button onclick=self.link.callback(move |_| Msg::DeleteDeviceReq(id))>
                            { "delete" }
                        </button>
                    {")"}
                </div>
                <div>
                    { &device.maker }
                </div>
                <div>
                    { &device.feature }
                </div>
            </div>
        }
    }
}


impl Component for Detail {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::OwnerReq(props.owner_id));
        link.send_message(Msg::DeviceReq(props.owner_id));
        Self {
            props,
            link,
            owner: None,
            devices: None,
            fetch_owner: None,
            fetch_device: None,
            delete_device: None,
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>
                { self.render_detail(&self.owner, &self.devices) }
            </div>
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::OwnerReq(id) => {
                let req = Request::get(&format!("http://localhost:7878/owner/{}", id))
                    .body(Nothing)
                    .expect("can make request to server");
                let cb = self.link.callback(
                    |response: Response<Json<Result<OwnerResponse, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::OwnerResp(data)
                    },
                );
                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_device = Some(task);
                ()
            },
            Msg::DeviceReq(id) => {
                let req = Request::get(&format!("http://localhost:7878/device/{}", id))
                    .body(Nothing)
                    .expect("can make request to server");
                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<DeviceResponse>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::DevicesResp(data)
                    },
                );
                let task =FetchService::fetch(req, cb).expect("can create task");
                self.fetch_device = Some(task);
                ()
            },
            Msg::DeleteDeviceReq(id) => {
                let req = Request::delete(&format!("http://localhost:7878/device/{}", id))
                    .body(Nothing)
                    .expect("can make request to server");
                let cb = self.link.callback(
                    move |response: Response<Json<Result<(), anyhow::Error>>>| {
                        Msg::DeleteDeviceResp(response, id)
                    },
                );
                let task = FetchService::fetch(req, cb).expect("can create task");
                self.fetch_device = Some(task);
                ()
            }
            Msg::OwnerResp(resp) => {
                if let Ok(data) = resp {
                    self.owner = Some(data);
                }
            }
            Msg::DevicesResp(resp) => {
                if let Ok(data) = resp {
                    self.devices = Some(data);
                }
            }
            Msg::DeleteDeviceResp(resp, id) => {
                if resp.status().is_success() {
                    self.devices = self.devices.as_ref().map(|device| device.iter().filter(|p| p.id != id).cloned().collect());
                }
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }
}