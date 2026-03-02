use std::env;
extern crate dotenv;
use::lettre::message::header::ContentType;
use::lettre::transport::smtp::authentication::Credentials;
use::lettre::{Message,SmtpTransport,Transport};
#[macro_use] extern crate rocket;

use rocket::serde::{Deserialize, Serialize, json::Json};
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Email {
    user: String,
    HoraInicio: String,
    HoraFinal: String,
    dia: u8,
    Destinatario: String,
    Activivdad: String
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ApiResponse {
    message: String,
}

#[launch]
fn rocket() -> _ {
    dotenv::from_path("./../../config/rustEmailServiceconfig.env").expect("Error cargando variables de entorno");
    rocket::build()
        .mount("/api", routes![send_email])
}
#[post("/sendEmail", data = "<email>")]
fn send_email(email: Json<Email>) -> Result<Json<ApiResponse>, Json<ApiResponse>> {

    let dia = match email.dia {
        1 => "lunes",
        2 => "martes",
        3 => "miercoles",
        4 => "jueves",
        5 => "viernes",
        6 => "sabado",
        7 => "domingo",
        _ => {
            return Err(Json(ApiResponse {
                message: "Dia invalido".into(),
            }))
        }
    };

    let messagetosend = format!(
        "Hola {}, desde la página de UPB Planner queremos recordarte que ya casi se acerca la fecha límite de tu actividad: {}, 
        esta fue programad@ con una fecha de vencimiento: {} a las {}. 
        Si necesitas revisarlo en detalle no olvides visitar la página oficial de UPB Planner, tu aliado de confianza en la U: {}.
        ",  
        email.user,
        email.Activivdad,
        dia,
        email.HoraFinal,
        "http://proyectointegrador.playit.plus/"
    );

    let emailsend = Message::builder()
        .from("UPB Planner recordatorio <upbplanner@gmail.com>".parse().unwrap())
        .to(email.Destinatario.parse().unwrap())
        .subject("Recordatorio")
        .header(ContentType::TEXT_PLAIN)
        .body(messagetosend)
        .unwrap();

    let credentials = Credentials::new(
        env::var("Email").expect("env error").to_owned(),
        env::var("Password").expect("env error")
    );

    let smtp = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(credentials)
        .build();

    match smtp.send(&emailsend) {
        Ok(_) => Ok(Json(ApiResponse {
            message: "Correo enviado correctamente".into(),
        })),
        Err(e) => Err(Json(ApiResponse {
            message: format!("Error enviando correo: {:?}", e),
        })),
    }
}