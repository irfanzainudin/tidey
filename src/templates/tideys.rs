use crate::tideys::{PartialTidey, Tidey};
use askama::Template;

#[derive(Template)]
#[template(path = "tideys/form.html")]
pub struct TideyForm<'a> {
    body: &'a str,
    action: String,
}

impl<'a> TideyForm<'a> {
    pub fn for_partial_tidey(tidey: &'a PartialTidey) -> Self {
        Self {
            body: tidey.body.as_deref().unwrap_or_default(),
            action: "/tideys".into(),
        }
    }

    // pub fn for_tidey(tidey: &'a Tidey) -> Self {
    //     Self {
    //         body: &tidey.body,
    //         action: format!("/tideys/{}", tidey.id),
    //     }
    // }
}

#[derive(Template)]
#[template(path = "tideys/index.html")]
pub struct IndexTemplate<'a> {
    tideys: &'a [Tidey],
}

impl<'a> IndexTemplate<'a> {
    pub fn for_tideys(tideys: &'a [Tidey]) -> Self {
        Self { tideys }
    }
}

#[derive(Template)]
#[template(path = "tideys/show.html")]
pub struct ShowTemplate<'a> {
    body: &'a str,
}

impl<'a> ShowTemplate<'a> {
    pub fn for_tidey(tidey: &'a Tidey) -> Self {
        Self { body: &tidey.body }
    }
}
