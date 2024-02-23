pub mod tideys;
use crate::templates::home::*;
use crate::templates::welcome::*;
use crate::tideys::Tidey;
// use crate::tideys::{PartialTidey, Tidey};
// use crate::utils;
use sqlx::prelude::*;

pub async fn home(mut _request: crate::Request) -> tide::Result {
    let mut tideys = Tidey::all().fetch_all(&_request.state().db).await?;
    tideys.reverse(); // reversing to get the latest tideys on top
    Ok(HomeTemplate::new("Tidey", tideys, "/tideys".into()).into())
}

pub async fn welcome(_request: crate::Request) -> tide::Result {
    Ok(WelcomeTemplate::new("Tidey").into())
}
