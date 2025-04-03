// use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use tera::{Tera, Context};

// struct variables
// struct WebState {
//     username: String,
//     darkmode: bool,
// }

async fn goto(tera: web::Data<Tera>, page: &str) -> impl Responder {
    let mut ctx = Context::new();
    let rendered = tera.render(page, &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn index(tera: web::Data<Tera>) -> impl Responder {
    goto(tera, "index.html").await
}

async fn about(tera: web::Data<Tera>) -> impl Responder {
    goto(tera, "about.html").await
}

async fn projects(tera: web::Data<Tera>) -> impl Responder {
    goto(tera, "projects.html").await
}

async fn gallery(tera: web::Data<Tera>) -> impl Responder {
    goto(tera, "gallery.html").await
}

async fn contact(tera: web::Data<Tera>) -> impl Responder {
    goto(tera, "contact.html").await
}

// main function
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting HTTP server at http://localhost:8080");
    
    // setup global data

    // setup user state: local data
    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera templates");

    HttpServer::new(move || {
        let tera_clone = tera.clone(); 
        App::new()
            .app_data(web::Data::new(tera_clone))
            .route("/", web::get().to(index))
            .route("/about", web::get().to(about))
            .route("/projects", web::get().to(projects))
            .route("/gallery", web::get().to(gallery))
            .route("/contact", web::get().to(contact)) 
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}