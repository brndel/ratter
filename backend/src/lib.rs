use std::time::Duration;

use dioxus::{
    fullstack::{StatusCode, body::Body, extract::Request, response::Response},
    logger::tracing::info,
    server::{
        axum::{self, middleware::Next},
        http::header::{AUTHORIZATION, COOKIE, SET_COOKIE, WWW_AUTHENTICATE},
    },
};
use persist::{PersistDb, SessionToken, UserRole};
use uuid::Uuid;

use crate::auth::{AuthHeader, UserId};

pub mod matter;
// mod from_endpoint;
pub mod asset;
mod auth;
mod automation_action_runner;
mod controls;
pub mod event_bus;
mod node_connections;
mod ota;
mod read_only_arc;
mod thread;

pub async fn add_router_layers(router: axum::Router) -> axum::Router {
    use dioxus::server::axum::Extension;

    let matter = matter::MatterManager::new().await.unwrap();
    let db = matter.db().clone();

    router
        .layer(Extension(matter.clone()))
        .layer(axum::middleware::from_fn(
            move |mut request: Request, next: Next| {
                let db = db.clone();

                async move {
                    let uri = request.uri();
                    info!("request: {}", uri);
                    info!("cookie header: {:?}", request.headers().get(COOKIE));
                    info!("authorization: {:?}", request.headers().get(AUTHORIZATION));

                    if let Some(auth) = AuthHeader::from_headers(request.headers()) {
                        if let Ok(session) = get_session_info(&auth, &db).await {
                            request.extensions_mut().insert(UserId {
                                id: session.user_id,
                            });

                            let mut response = next.run(request).await;
                            if let Some((token, max_age)) = session.set_cookie {
                                response.headers_mut().insert(
                                    SET_COOKIE,
                                    format!(
                                        "session={}; SameSite=Lax; Secure; Max-Age={}",
                                        token,
                                        max_age.as_secs()
                                    )
                                    .parse()
                                    .unwrap(),
                                );
                            }

                            return response;
                        }
                    }

                    info!("no (or invalid) auth header/cookie specified");
                    return Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .header(WWW_AUTHENTICATE, "Basic")
                        .body(Body::empty())
                        .unwrap();
                }
            },
        ))
}

struct SessionInfo {
    user_id: Option<Uuid>,
    set_cookie: Option<(SessionToken, Duration)>,
}

async fn get_session_info(auth: &AuthHeader, db: &PersistDb) -> anyhow::Result<SessionInfo> {
    match auth {
        AuthHeader::Session { token } => {
            let user_id = db.get_user_of_session(&token).await?;
            Ok(SessionInfo {
                user_id,
                set_cookie: None,
            })
        }
        AuthHeader::Basic {
            user_name,
            password,
        } => {
            let (user_id, token, duration) =
                match db.create_user_session(&user_name, &password).await {
                    Ok((a, b, c)) => (a, b, c),
                    Err(err) => {
                        if db.get_user_count().await? == 0 {
                            db.create_user(user_name, password, UserRole::Admin).await?;
                            db.create_user_session(user_name, password).await?
                        } else {
                            return Err(err.into());
                        }
                    }
                };
            Ok(SessionInfo {
                user_id: Some(user_id),
                set_cookie: Some((token, duration)),
            })
        }
    }
}
