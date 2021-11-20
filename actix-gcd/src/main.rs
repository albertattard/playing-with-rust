use actix_web::{App, HttpServer, web};

mod gcd;
mod view;

fn main() {
    let serve = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(view::get_index))
            .route("/gcd", web::post().to(view::post_gcd))
    });

    println!("Serving on http://localhost:3000");
    serve.bind("127.0.0.1:3000").expect("Failed to bind to address")
        .run().expect("Failed to run server");
}
