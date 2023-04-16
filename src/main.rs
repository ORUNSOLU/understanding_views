use actix_web::{HttpServer, App, web};
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(views::views_factory).route("/", web::to(main_menu))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


pub async fn main_menu() -> String {
    format!(
        "This programme is just to basically understand how views & factory works.\n\n
        To access the login view: 'http://127.0.0.1:8080/v1/auth/login'\n\n
        To access the logout view: 'http://127.0.0.1:8080/v1/auth/logout'\n

        Cheers
        "
    )
}