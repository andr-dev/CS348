use rocket::Route;

mod champ;
mod matches;
mod summoner;

#[get("/ping")]
fn ping() -> &'static str {
    "pong"
}

pub fn routes() -> Vec<Route> {
    let mut routes = routes![ping];

    routes.extend(matches::routes());
    routes.extend(champ::routes());
    routes.extend(summoner::routes());

    routes
}
