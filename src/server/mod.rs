use actix_session::Session;
use actix_web::web;
use lemmy_client::LemmyClient;
use leptos::ServerFnError;
use leptos_actix::extract;

pub mod cookie_middleware;

/// The client and session are going to be needed together most of the time
pub async fn get_client_and_session() -> (
  Result<web::Data<LemmyClient>, ServerFnError>,
  Result<Session, ServerFnError>,
) {
  tokio::join!(extract::<web::Data<LemmyClient>>(), extract::<Session>())
}
