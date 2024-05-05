use tide::Server;
use crate::handlers::handlers;



pub fn setup_routes(app : &mut Server<()>){
    app.at("/token").get(handlers::get_token);

}

