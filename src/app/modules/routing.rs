use crate::app::modules::messages::controller::routes as messages_routes;
use crate::app::modules::tokens::controller::routes as tokens_routes;

pub fn router() -> rocket::fairing::AdHoc {
    rocket::fairing::AdHoc::on_ignite("Routes", |rocket| async {
        rocket
            .mount("/api/v1/messaging/message", messages_routes())
            .mount("/api/v1/messaging/token", tokens_routes())
    })
}
