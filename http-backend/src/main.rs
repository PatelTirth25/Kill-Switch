use actix_cors::Cors;
use actix_web::{get, http::header, post, web, App, HttpResponse, HttpServer, Responder};
use bollard::container::{
    Config, CreateContainerOptions, RemoveContainerOptions, StartContainerOptions,
};
use bollard::models::{HostConfig, PortBinding};
use bollard::Docker;
use std::collections::HashMap;
mod models;
mod pg;
mod schema;
mod schema_type;

use diesel::prelude::*;
use diesel::{BoolExpressionMethods, ExpressionMethods};
use models::{NewRoom, NewUser};
use schema_type::{Login, LoginResponse, Room, RoomResponse, SignUp, SignUpResponse};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body(format!("Hello World"))
}

#[post("/room/create")]
async fn createroom(data: web::Json<Room>) -> impl Responder {
    println!("Creating room: {:#?}", data);

    use schema::rooms::dsl::*;
    let conn = &mut pg::establish_connection();

    let room_available = rooms
        .filter(room.eq(&data.id))
        .select((id, room, url))
        .first::<models::Room>(conn)
        .optional()
        .unwrap();

    if Option::is_some(&room_available) {
        return HttpResponse::Ok().json(RoomResponse {
            success: true,
            message: "ws://127.0.0.1:".to_string() + &(room_available.unwrap().url + "/"),
        });
    } else {
        let docker = Docker::connect_with_local_defaults().unwrap();

        let room_ports: Vec<String> = rooms.select(url).load::<String>(conn).unwrap();
        let mut port: Option<String> = None;

        let all_ports = ["8001", "8002", "8003", "8004", "8005"];

        for p in all_ports.iter() {
            if !room_ports.contains(&p.to_string()) {
                port = Some(p.to_string());
                break;
            }
        }

        let room_id = &data.id;
        println!("Port {} and Room Id {} ", port.clone().unwrap(), room_id);

        if Option::is_none(&port) {
            return HttpResponse::NotFound().json(RoomResponse {
                success: false,
                message: "Maximum Room Limit hit".to_string(),
            });
        }

        let port = port.unwrap();

        let mut port_bindings = HashMap::new();
        port_bindings.insert(
            "3001/tcp".to_string(),
            Some(vec![PortBinding {
                host_ip: Some("127.0.0.1".to_string()),
                host_port: Some(port.to_string()),
            }]),
        );

        let host_config = HostConfig {
            port_bindings: Some(port_bindings),
            nano_cpus: Some(1_000_000_000),
            memory: Some(256 * 1024 * 1024),
            ..Default::default()
        };

        let config = Config {
            image: Some("game-ws-backend"),
            host_config: Some(host_config),
            ..Default::default()
        };

        let options = CreateContainerOptions {
            name: format!("room-{}", room_id),
            platform: Some("linux/amd64".to_string()),
        };

        docker
            .create_container(Some(options), config)
            .await
            .unwrap();
        docker
            .start_container(
                &format!("room-{}", room_id),
                None::<StartContainerOptions<String>>,
            )
            .await
            .unwrap();

        let new_room = NewRoom {
            room: room_id,
            url: &port,
        };

        let created_room = diesel::insert_into(schema::rooms::table)
            .values(&new_room)
            .get_result::<models::User>(conn)
            .optional()
            .unwrap();

        println!("Created room: {:#?}", created_room);

        return HttpResponse::NotFound().json(RoomResponse {
            success: true,
            message: "ws://127.0.0.1:".to_string() + &port + "/",
        });
    }
}

#[post("/room/stop")]
async fn stoproom(data: web::Json<Room>) -> impl Responder {
    let docker = Docker::connect_with_local_defaults().unwrap();

    use schema::rooms::dsl::*;
    let conn = &mut pg::establish_connection();

    let _ = diesel::delete(schema::rooms::table)
        .filter(room.eq(data.clone().id))
        .execute(conn);

    let container_name = format!("room-{}", data.id);

    docker
        .remove_container(
            &container_name,
            Some(RemoveContainerOptions {
                force: true,
                ..Default::default()
            }),
        )
        .await
        .unwrap();

    return HttpResponse::NotFound().json(RoomResponse {
        success: true,
        message: "Container Removed Successfully".to_string(),
    });
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
            .service(createroom)
            .service(stoproom)
    })
    .bind(("127.0.0.1", 3002))?
    .run()
    .await
}
