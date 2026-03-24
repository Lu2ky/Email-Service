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
    horaFinal: String,
    dia: u8,
    destinatario: String,
    actividad: String
}

// Email de token
struct EmailToken {
    user: String,
    token: String,
    minutos: String,
    destinatario: String
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ApiResponse {
    message: String,
}

#[launch]
fn rocket() -> _ {

    //dotenv::from_path("./../../../config/rustEmailServiceconfig.env").expect("Error cargando variables de entorno");  // <----- Custom ruta / local
    dotenv::dotenv().ok();  // <----- Producción
    rocket::build()
        .mount("/api", routes![send_email])
}
#[post("/sendEmail", data = "<email>")]
fn send_email(email: Json<Email>) -> Result<Json<ApiResponse>, Json<ApiResponse>> {

    let dia = match email.0.dia {
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
    let url = "https://www.upbplanner.online/";
    let messagetosend = format!(
        "Hola {}, desde la página de UPB Planner queremos recordarte que ya casi se acerca la fecha límite de tu actividad: {}, esta fue programad@ con una fecha de vencimiento: {} a las {}. Si necesitas revisarlo en detalle no olvides visitar la página oficial de UPB Planner, tu aliado de confianza en la U: {}.",  
        email.0.user,
        email.0.actividad,
        dia,
        email.0.horaFinal,
        url
    );

    let emailsend = Message::builder()
        .from("UPB Planner recordatorio <upbplanner@gmail.com>".parse().unwrap())
        .to(email.destinatario.parse().unwrap())
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

#[post("/sendEmailToken", data = "<email>")]
fn send_email(email: Json<EmailToken>) -> Result<Json<ApiResponse>, Json<ApiResponse>> {
    
    let messagetosend = format!(
        "Hola {}, desde la página de UPB Planner le enviamos el token de restablecimiento de la contraseña
        que usted solicitó: 
        \n\ {}. 
        \n\ Este token fue programad@ para vencerse en {} minutos. Recuerda que si el tiempo caduca
        deberás realizar una solicitud nueva para obetener otro token.",
        email.0.user,
        email.0.token,
        email.0.minutos,
    );

    let emailsend = Message::builder()
        .from("UPB Planner recordatorio <upbplanner@gmail.com>".parse().unwrap())
        .to(email.destinatario.parse().unwrap())
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