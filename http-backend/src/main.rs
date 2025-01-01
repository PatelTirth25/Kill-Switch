use actix_cors::Cors;
use actix_web::{get, http::header, post, web, App, HttpResponse, HttpServer, Responder};
mod models;
mod pg;
mod schema;
mod schema_type;

use diesel::prelude::*;
use diesel::{BoolExpressionMethods, ExpressionMethods};
use models::NewUser;
use schema_type::{Login, LoginResponse, Room, RoomResponse, SignUp, SignUpResponse};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(format!("Hello World"))
}

#[post("/room")]
async fn room(data: web::Json<Room>) -> impl Responder {
    println!("Creating room: {:#?}", data);

    // TODO: Make a room and send back the url

    HttpResponse::Ok().json(RoomResponse {
        success: true,
        message: "http://127.0.0.1:3001/".to_string(),
    })
}

#[post("/login")]
async fn login(data: web::Json<Login>) -> impl Responder {
    let conn = &mut pg::establish_connection();

    use schema::users::dsl::*;
    // check if user exists and then return if it exits else err
    let user = users
        .filter(username.eq(&data.username).and(password.eq(&data.password)))
        .select((id, username, password))
        .first::<models::User>(conn)
        .optional()
        .unwrap();

    if Option::is_some(&user) {
        return HttpResponse::Ok().json(LoginResponse {
            success: true,
            id: user.unwrap().id,
        });
    } else {
        return HttpResponse::NotFound().json(LoginResponse {
            success: false,
            id: -1,
        });
    }
}

#[post("/signup")]
async fn signup(data: web::Json<SignUp>) -> impl Responder {
    let conn = &mut pg::establish_connection();

    if data.password != data.confirm_password {
        return HttpResponse::BadRequest().json("Password does not match");
    }

    use schema::users::dsl::*;
    // check if user exists and then return if it exits else err
    let user = users
        .filter(username.eq(&data.username).and(password.eq(&data.password)))
        .select((id, username, password))
        .first::<models::User>(conn)
        .optional()
        .unwrap();

    if Option::is_some(&user) {
        return HttpResponse::Ok().json(SignUpResponse {
            message: "User already exists".to_string(),
            success: false,
        });
    } else {
        // create the user and send back
        let new_user = NewUser {
            username: &data.username,
            password: &data.password,
        };
        let created_user = diesel::insert_into(schema::users::table)
            .values(&new_user)
            .get_result::<models::User>(conn)
            .optional()
            .unwrap();

        println!("User created: {:#?}", created_user);

        if Option::is_none(&created_user) {
            return HttpResponse::InternalServerError().json(SignUpResponse {
                message: "Failed to create user".to_string(),
                success: false,
            });
        }

        return HttpResponse::Ok().json(SignUpResponse {
            message: "User created successfully".to_string(),
            success: true,
        });
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                origin.as_bytes().starts_with(b"http://localhost")
            })
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                header::AUTHORIZATION,
                header::ACCEPT,
                header::CONTENT_TYPE,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(hello)
            .service(login)
            .service(signup)
            .service(room)
    })
    .bind(("127.0.0.1", 3002))?
    .run()
    .await
}
