use actix_web::{web, App, HttpResponse, HttpServer};
use chrono::{Datelike, Local};

async fn index() -> HttpResponse {
    let now = Local::now();
    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <title>dd-mm-yyyy</title>
    <meta charset="UTF-8">
    <style>
        body {{
            background-color: #2b2d30;
            color: #ffffff;
        }}
    </style>
</head>
<body>
    <div id="date">{:02}.{:02}.{}</div>
</body>
</html>"#,
        now.day(),
        now.month(),
        now.year()
    );

    HttpResponse::Ok()
        .content_type(mime::TEXT_HTML)
        .body(html)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
