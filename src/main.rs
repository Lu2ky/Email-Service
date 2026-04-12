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
    horaFinal: String,    // snake_case: renombrado de horaFinal
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

    let url = "https://www.upbplanner.online/";
    let body = format!(
    r#"
    <!DOCTYPE html>
    <html lang="es">
    <head>
        <meta charset="UTF-8">
        <style>
            body {{ font-family: Arial, sans-serif; background-color: #ffffff; margin: 0; padding: 0; }}
            .container {{ max-width: 520px; margin: 0 auto; padding: 40px 30px; }}
            .logo {{ text-align: center; margin-bottom: 30px; }}
            .logo img {{ width: 80px; }}
            .content {{ color: #222222; font-size: 15px; line-height: 1.6; }}
            .actividad {{ border-left: 4px solid #cc2d7e; background-color: #f9f9f9; padding: 14px 18px; margin: 20px 0; border-radius: 6px; font-size: 14px; color: #333; }}
            .actividad p {{ margin: 4px 0; }}
            .btn-container {{ text-align: center; margin: 30px 0; }}
            .btn {{ display: inline-block; background: linear-gradient(to right, #cc2d7e, #a020c0); color: white !important; padding: 14px 36px; border-radius: 30px; text-decoration: none; font-size: 15px; font-weight: bold; }}
            .footer {{ text-align: center; font-size: 11px; color: #aaaaaa; margin-top: 30px; line-height: 1.6; }}
        </style>
    </head>
    <body>
        <div class="container">
            <div class="logo">
                <img src="https://www.upbplanner.online/assets/logo-XQtAi6hP.png" alt="UPB Logo">
            </div>
            <div class="content">
                <p>Hola <strong>{}</strong>,</p>
                <p>Te informamos que tienes una actividad próxima a vencer.</p>
                <div class="actividad">
                    <p>📌 <strong>Actividad:</strong> {}</p>
                    <p>📅 <strong>Día:</strong> {}</p>
                    <p>⏰ <strong>Hora:</strong> {}</p>
                </div>
                <p>No dejes que se te pase. Revísala en detalle aquí:</p>
            </div>
            <div class="btn-container">
                <a class="btn" href="{}">Ir a UPB Planner</a>
            </div>
            <div class="footer">
                <p>Este mensaje fue generado automáticamente por UPB Planner.<br>Por favor no respondas a este correo.</p>
            </div>
        </div>
    </body>
    </html>
    "#,
    email.0.user,      
    email.0.actividad, 
    dia,               
    email.0.horaFinal, 
    url                
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
    r#"
    <!DOCTYPE html>
    <html lang="es">
    <head>
        <meta charset="UTF-8">
        <style>
            body {{ font-family: Arial, sans-serif; background-color: #ffffff; margin: 0; padding: 0; }}
            .container {{ max-width: 520px; margin: 0 auto; padding: 40px 30px; }}
            .logo {{ text-align: center; margin-bottom: 30px; }}
            .logo img {{ width: 80px; }}
            .content {{ color: #222222; font-size: 15px; line-height: 1.6; }}
            .token-box {{ border-left: 4px solid #cc2d7e; background-color: #f9f9f9; padding: 14px 18px; margin: 20px 0; border-radius: 6px; text-align: center; }}
            .token {{ font-size: 28px; font-weight: bold; letter-spacing: 8px; color: #a020c0; margin: 8px 0; }}
            .warning {{ font-size: 13px; color: #cc2d7e; margin-top: 6px; }}
            .footer {{ text-align: center; font-size: 11px; color: #aaaaaa; margin-top: 30px; line-height: 1.6; }}
        </style>
    </head>
    <body>
        <div class="container">
            <div class="logo">
                <img src="https://www.upbplanner.online/assets/logo-XQtAi6hP.png" alt="UPB Logo">
            </div>
            <div class="content">
                <p>Hola <strong>{}</strong>,</p>
                <p>Te enviamos el token de restablecimiento de contraseña que solicitaste:</p>
                <div class="token-box">
                    <p>🔐 <strong>Tu token es:</strong></p>
                    <p class="token">{}</p>
                    <p class="warning">⏳ Este token vence en <strong>{} minutos</strong>.</p>
                </div>
                <p>Si el tiempo caduca, deberás realizar una nueva solicitud para obtener otro token.</p>
                <p>Si no solicitaste este cambio, ignora este correo.</p>
            </div>
            <div class="footer">
                <p>Este mensaje fue generado automáticamente por UPB Planner.<br>Por favor no respondas a este correo.</p>
            </div>
        </div>
    </body>
    </html>
    "#,
    email.user,    // Hola {}
    email.token,   // Token
    email.minutos  // Minutos
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
        .header(ContentType::TEXT_HTML)
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
