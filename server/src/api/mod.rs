use rocket::Route;

mod champion;
mod matches;
mod summoner;

pub use champion::Champion;

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

pub fn routes() -> Vec<Route> {
    let mut routes = routes![ping];

    routes.extend(matches::routes());
    routes.extend(champion::routes());
    routes.extend(summoner::routes());

    routes
}
