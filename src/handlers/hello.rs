use crate::handlers::handlerconstants::SERVER_GREETING;

pub async fn hello() -> &'static str {
    SERVER_GREETING
}
