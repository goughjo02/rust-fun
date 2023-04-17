use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParams {
    n: u64,
    m: u64,
}

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });
    println!("Server running at http://localhost:8080");
    server.bind("127.0.0.1:8080").unwrap().run().unwrap();
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <html>
            <head>
                <title>GCD Calculator</title>
            </head>
            <body>
                <form action="/gcd" method="post">
                    <input type="text" name="n" />
                    <input type="text" name="m" />
                    <button type="submit">Compute GCD</button>
                </form>
            </body>
    "#,
    )
}

fn post_gcd(params: web::Form<GcdParams>) -> HttpResponse {
    if params.n == 0 || params.m == 0 {
        return HttpResponse::BadRequest().body("Both n and m must be positive");
    }
    let mut n = params.n;
    let mut m = params.m;
    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }
    HttpResponse::Ok().body(format!("The greatest common divisor of the numbers {} and {} is {}", params.n, params.m, n))
}