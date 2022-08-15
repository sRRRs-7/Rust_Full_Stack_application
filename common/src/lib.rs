use serde::{Serialize, Deserialize};

// database domain struct

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Owner {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OwnerRequest {
    pub name: String
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OwnerResponse {
    pub id: i32,
    pub name: String,
}

impl OwnerResponse {
    pub fn res(owner: Owner) -> OwnerResponse {
        OwnerResponse {
            id: owner.id,
            name: owner.name
        }
    }
}




#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Device {
    pub id: i32,
    pub product: String,
    pub owner_id: i32,
    pub maker: String,
    pub feature: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceRequest {
    pub product: String,
    pub maker: String,
    pub feature: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DeviceResponse {
    pub id: i32,
    pub product: String,
    pub maker: String,
    pub feature: String,
}

impl DeviceResponse {
    fn res(device: Device) -> DeviceResponse {
        DeviceResponse {
            id: device.id,
            product: device.product,
            maker: device.maker,
            feature: device.feature,
        }
    }
}


