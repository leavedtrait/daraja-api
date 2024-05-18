use crate::handlers::handlers;
use tide::Server;

pub fn setup_routes(app: &mut Server<()>) {
    app.at("/token").get(handlers::get_token);
    app.at("/stk").post(handlers::stk_push);
    app.at("/validation").post(handlers::handle_stk_response);
}
