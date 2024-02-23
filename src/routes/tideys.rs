use crate::templates::tideys::*;
use crate::tideys::{PartialTidey, Tidey};
use crate::utils;
use sqlx::prelude::*;

pub async fn index(request: crate::Request) -> tide::Result {
    let tideys = Tidey::all().fetch_all(&request.state().db).await?;
    Ok(IndexTemplate::for_tideys(tideys.as_slice()).into())
}

pub async fn show(request: crate::Request) -> tide::Result {
    let tidey = Tidey::find_by_id(request.param("tidey_id")?.parse()?)
        .fetch_one(&request.state().db)
        .await?;

    Ok(ShowTemplate::for_tidey(&tidey).into())
}

pub async fn delete(request: crate::Request) -> tide::Result {
    Tidey::delete_by_id(request.param("tidey_id")?.parse()?)
        .execute(&request.state().db)
        .await?;

    // if we had sessions, we'd set a flash message with whether this was successful
    Ok(tide::Redirect::new("/").into())
}

pub async fn update(mut request: crate::Request) -> tide::Result {
    let tidey: PartialTidey = utils::deserialize_body(&mut request).await?;
    let tidey_id = request.param("tidey_id")?.parse()?;
    let rows_updated = tidey
        .update_by_id(tidey_id)
        .execute(&request.state().db)
        .await?;

    if rows_updated == 1 {
        Ok(tide::Redirect::new(format!("/tideys/{}", tidey_id)).into())
    } else {
        Ok(TideyForm::for_partial_tidey(&tidey).into())
    }
}

pub async fn create(mut request: crate::Request) -> tide::Result {
    let db = &request.state().db;
    let mut tx = db.begin().await?;
    let tidey: PartialTidey = utils::deserialize_body(&mut request).await?;
    let created = tidey.create().execute(&mut tx).await?;

    if created == 1 {
        let (_last_id,) = Tidey::last_id().fetch_one(&mut tx).await?;
        tx.commit().await?;

        // Ok(tide::Redirect::new(format!("/tideys/{}", last_id)).into())
        Ok(tide::Redirect::new("/home").into())
    } else {
        Ok(TideyForm::for_partial_tidey(&tidey).into())
    }
}

pub async fn new(_request: crate::Request) -> tide::Result {
    let tidey = PartialTidey::default();
    Ok(TideyForm::for_partial_tidey(&tidey).into())
}
