mod db;
mod handlers;
mod models;
mod routes;

use warp::Filter;

#[tokio::main]
async fn main() {
    db::initialize_db();
    let routes = routes::restaurant_routes();

    println!("Running the server");
    // This is the Listen and Serve in Warp framework where you specify the
    // incoming routes, the IP address, the port address and await it to start
    warp::serve(routes.with(warp::trace::request()))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
