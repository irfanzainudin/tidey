pub mod tideys;

pub mod home {
    use crate::tideys::Tidey;
    use askama::Template;
    #[derive(Template)]
    #[template(path = "home.html")]
    pub struct HomeTemplate<'a> {
        name: &'a str,
        tideys: Vec<Tidey>,
        action: String,
    }

    impl<'a> HomeTemplate<'a> {
        pub fn new(name: &'a str, tideys: Vec<Tidey>, action: String) -> Self {
            Self {
                name,
                tideys,
                action,
            }
        }
    }
}

pub mod welcome {
    use askama::Template;
    #[derive(Template)]
    #[template(path = "welcome.html")]
    pub struct WelcomeTemplate<'a> {
        name: &'a str,
    }

    impl<'a> WelcomeTemplate<'a> {
        pub fn new(name: &'a str) -> Self {
            Self { name }
        }
    }
}
