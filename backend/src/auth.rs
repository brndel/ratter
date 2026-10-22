use std::str::FromStr;

use base64::{Engine, engine::general_purpose::STANDARD};
use dioxus::{
    fullstack::HeaderMap,
    server::http::header::{AUTHORIZATION, COOKIE},
};
use persist::SessionToken;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct UserId {
    pub id: Option<Uuid>
}

pub enum AuthHeader {
    Session { token: SessionToken },
    Basic { user_name: String, password: String },
}

impl AuthHeader {
    pub fn from_headers(headers: &HeaderMap) -> Option<Self> {
        if let Some(session_cookie) = Self::get_session_cookie(headers) {
            let token = SessionToken::from_str(session_cookie).ok()?;
            return Some(Self::Session { token })
        }

        let auth = headers.get(AUTHORIZATION)?.to_str().ok()?;

        let (auth_scheme, auth_params) = auth.split_once(' ')?;

        match auth_scheme {
            "Basic" => {
                let decoded = STANDARD
                    .decode(auth_params)
                    .ok()
                    .and_then(|bytes| String::from_utf8(bytes).ok())?;

                let (user_name, password) = decoded.split_once(':')?;

                Some(Self::Basic {
                    user_name: user_name.to_owned(),
                    password: password.to_owned(),
                })
            }
            _ => None,
        }
    }

    fn get_session_cookie(headers: &HeaderMap) -> Option<&str> {
        let cookies = headers.get(COOKIE)?.to_str().ok()?;

        cookies
            .split("; ")
            .filter_map(|cookie| cookie.split_once("="))
            .find(|(name, _session)| *name == "session")
            .map(|(_name, session)| session)
    }
}
