#[macro_use]
extern crate rocket;
extern crate dotenv;

use std::env;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use rocket::serde::{json::Json, Deserialize, Serialize};
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Email {
    user: String,
    hora_inicio: String,   // snake_case: renombrado de horaInicio
    hora_final: String,    // snake_case: renombrado de horaFinal
    dia: u8,
    destinatario: String,
    actividad: String,
}
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct EmailToken {
    user: String,
    token: String,
    minutos: String,
    destinatario: String,
}
#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ApiResponse {
    message: String,
}
fn build_smtp() -> Result<SmtpTransport, String> {
    let user = env::var("Email")
        .map_err(|_| "Variable de entorno 'Email' no encontrada".to_string())?;
    let pass = env::var("Password")
        .map_err(|_| "Variable de entorno 'Password' no encontrada".to_string())?;

    SmtpTransport::relay("smtp.gmail.com")
        .map_err(|e| format!("Error configurando SMTP: {:?}", e))
        .map(|t| t.credentials(Credentials::new(user, pass)).build())
}
#[post("/sendEmail", data = "<email>")]
fn send_email(email: Json<Email>) -> Result<Json<ApiResponse>, Json<ApiResponse>> {
    let dia = match email.dia {
        1 => "lunes",
        2 => "martes",
        3 => "miércoles",
        4 => "jueves",
        5 => "viernes",
        6 => "sábado",
        7 => "domingo",
        _ => {
            return Err(Json(ApiResponse {
                message: "Día inválido. Use un valor entre 1 (lunes) y 7 (domingo).".into(),
            }))
        }
    };

    let url = "http://proyectointegrador.playit.plus/";
    let body = format!(
        r#"<!DOCTYPE html>
<html lang="es">
<head>
    <meta charset="UTF-8">
    <style>
        body {{ font-family: Arial, sans-serif; background-color: #f4f4f4; margin: 0; padding: 0; }}
        .container {{ max-width: 600px; margin: 30px auto; background: #ffffff; border-radius: 8px; padding: 30px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }}
        .header {{ background-color: #003087; color: white; padding: 20px; border-radius: 8px 8px 0 0; text-align: center; }}
        .content {{ padding: 20px 0; color: #333333; }}
        .actividad {{ background-color: #f0f4ff; border-left: 4px solid #003087; padding: 10px 15px; margin: 15px 0; border-radius: 4px; }}
        .btn {{ display: inline-block; background-color: #003087; color: white; padding: 12px 24px; border-radius: 6px; text-decoration: none; margin-top: 20px; }}
        .footer {{ text-align: center; font-size: 12px; color: #999999; margin-top: 30px; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header"><h2>📅 UPB Planner</h2></div>
        <div class="content">
            <p>Hola, <strong>{}</strong></p>
            <p>Te recordamos que tienes una actividad próxima a vencer:</p>
            <div class="actividad">
                <strong>📌 Actividad:</strong> {}<br>
                <strong>📆 Día:</strong> {}<br>
                <strong>⏰ Hora límite:</strong> {}
            </div>
            <p>No dejes que se te pase. Revísala en detalle aquí:</p>
            <a class="btn" href="{}">Ir a UPB Planner</a>
        </div>
        <div class="footer">
            <p>Este mensaje fue generado automáticamente por UPB Planner.<br>Por favor no respondas a este correo.</p>
        </div>
    </div>
</body>
</html>"#,
        email.user, email.actividad, dia, email.hora_final, url
    );

    let message = Message::builder()
        .from(
            "UPB Planner recordatorio <upbplanner@gmail.com>"
                .parse()
                .map_err(|e| Json(ApiResponse { message: format!("Remitente inválido: {:?}", e) }))?,
        )
        .to(email
            .destinatario
            .parse()
            .map_err(|e| Json(ApiResponse { message: format!("Destinatario inválido: {:?}", e) }))?)
        .subject("Recordatorio de actividad - UPB Planner")
        .header(ContentType::TEXT_HTML)
        .body(body)
        .map_err(|e| Json(ApiResponse { message: format!("Error construyendo mensaje: {:?}", e) }))?;

    let smtp = build_smtp()
        .map_err(|e| Json(ApiResponse { message: e }))?;

    smtp.send(&message)
        .map(|_| Json(ApiResponse { message: "Correo enviado correctamente".into() }))
        .map_err(|e| Json(ApiResponse { message: format!("Error enviando correo: {:?}", e) }))
}

#[post("/sendEmailToken", data = "<email>")]
fn send_email_token(email: Json<EmailToken>) -> Result<Json<ApiResponse>, Json<ApiResponse>> {
    let body = format!(
        "Hola {}, desde la página de UPB Planner le enviamos el token de restablecimiento \
         de la contraseña que usted solicitó:\n\n{}\n\nEste token fue programado para vencerse \
         en {} minutos. Recuerda que si el tiempo caduca deberás realizar una solicitud nueva \
         para obtener otro token.",
        email.user, email.token, email.minutos
    );

    let message = Message::builder()
        .from(
            "UPB Planner recordatorio <upbplanner@gmail.com>"
                .parse()
                .map_err(|e| Json(ApiResponse { message: format!("Remitente inválido: {:?}", e) }))?,
        )
        .to(email
            .destinatario
            .parse()
            .map_err(|e| Json(ApiResponse { message: format!("Destinatario inválido: {:?}", e) }))?)
        .subject("Token de recuperación - UPB Planner")
        .header(ContentType::TEXT_PLAIN)
        .body(body)
        .map_err(|e| Json(ApiResponse { message: format!("Error construyendo mensaje: {:?}", e) }))?;

    let smtp = build_smtp()
        .map_err(|e| Json(ApiResponse { message: e }))?;

    smtp.send(&message)
        .map(|_| Json(ApiResponse { message: "Correo de token enviado correctamente".into() }))
        .map_err(|e| Json(ApiResponse { message: format!("Error enviando correo: {:?}", e) }))
}
#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok(); // <------ Prod
    rocket::build().mount("/api", routes![send_email, send_email_token])
}
