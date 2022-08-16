use crate::{db::{mod_device, mod_owner}, DBPool, Result};
use common::*;
use warp::{http::StatusCode, reject, reply::json, Reply};



// owner handler

pub async fn create_owner_handler(body: OwnerRequest, pool: DBPool) -> Result<impl Reply> {
    Ok(json(&OwnerResponse::res(
        mod_owner::insert(&pool, body).await.map_err(reject::custom)?,
    )))
}

pub async fn get_owner_handler(id: i32, pool: DBPool) -> Result<impl Reply> {
    let owner = mod_owner::get_one(&pool, id).await.map_err(reject::custom)?;
    Ok(json(&OwnerResponse::res(owner)))
}

pub async fn list_owner_handler(pool: DBPool) -> Result<impl Reply> {
    let owners = mod_owner::get_all(&pool).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &owners.into_iter().map(OwnerResponse::res).collect(),
    ))
}


// device handler

pub async fn create_device_handler(body: DeviceRequest, pool: DBPool) -> Result<impl Reply> {
    Ok(json(&DeviceResponse::res(
        mod_device::insert(&pool, body).await.map_err(reject::custom)?,
    )))
}

pub async fn list_device_handler(owner_id: i32, pool: DBPool) -> Result<impl Reply> {
    let devices = mod_device::get_all(&pool, owner_id).await.map_err(reject::custom)?;
    Ok(json::<Vec<_>>(
        &devices.into_iter().map(&DeviceResponse::res).collect(),
    ))
}

pub async fn delete_device_handler(id: i32, pool: DBPool) -> Result<impl Reply> {
    mod_device::delete(&pool, id).await.map_err(reject::custom)?;
    Ok(StatusCode::OK)
}
