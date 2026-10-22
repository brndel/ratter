use std::{fmt::Display, str::FromStr, time::Duration};

use base64::Engine;
use jiff::Timestamp;
use rand::{Rng, rngs::ThreadRng};
use sha2::{Digest, Sha256};
use toasty::{Deferred, create, stmt::Uuid};

use crate::{Result, persist_db::PersistDb};

#[derive(toasty::Model)]
pub struct User {
    #[key]
    #[auto]
    pub id: Uuid,
    #[unique]
    pub name: String,
    pub role: UserRole,
    pub salt: Vec<u8>,
    pub password: Vec<u8>,
    #[auto]
    pub created_at: Timestamp,
}
#[derive(toasty::Embed)]
pub enum UserRole {
    Admin,
    User
}

#[derive(toasty::Model)]
pub struct Session {
    #[key]
    pub token: Vec<u8>,
    pub user_id: Option<Uuid>,
    #[belongs_to]
    pub user: Deferred<Option<User>>,
    pub created_at: Timestamp,
    pub valid_until: Timestamp,
}

pub struct SessionToken(Vec<u8>);

impl SessionToken {
    fn new() -> Self {
        let mut token = vec![0; 32];
        ThreadRng::default().fill_bytes(token.as_mut());
        Self(token)
    }
}

impl Display for SessionToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        base64::engine::general_purpose::STANDARD
            .encode(&self.0)
            .fmt(f)
    }
}

impl FromStr for SessionToken {
    type Err = base64::DecodeError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        base64::engine::general_purpose::STANDARD
            .decode(s)
            .map(SessionToken)
    }
}

impl User {
    fn random_salt() -> Vec<u8> {
        let mut salt = vec![0; 256];
        ThreadRng::default().fill_bytes(salt.as_mut());
        salt
    }

    pub fn hash_password(password: &str, salt: &[u8]) -> Vec<u8> {
        let hasher = Sha256::new();
        let password_hash = hasher
            .chain_update(&salt)
            .chain_update(password.as_bytes())
            .finalize()
            .to_vec();

        password_hash
    }

    fn check_password(&self, password: &str) -> bool {
        self.password == Self::hash_password(password, &self.salt)
    }
}

impl PersistDb {
    pub async fn get_user_count(&self) -> Result<u64> {
        let count = User::all().count().exec(&mut self.db.clone()).await?;

        Ok(count)
    }

    pub async fn get_all_users(&self) -> Result<Vec<(Uuid, String)>> {
        let users = User::all().select((User::fields().id(), User::fields().name())).exec(&mut self.db.clone()).await?;

        Ok(users)
    }

    pub async fn create_user(&self, name: &str, password: &str, role: UserRole) -> Result<Uuid> {
        let salt = User::random_salt();
        let password = User::hash_password(&password, &salt);

        let user = create!(User {
            name,
            role,
            salt,
            password,
        })
        .exec(&mut self.db.clone())
        .await?;

        Ok(user.id)
    }

    async fn auth_user(&self, name: &str, password: &str) -> Result<Uuid> {
        let user = User::filter_by_name(name)
            .one()
            .exec(&mut self.db.clone())
            .await?;

        user.check_password(password)
            .then_some(user.id)
            .ok_or(crate::Error::WrongPassword)
    }

    pub async fn create_user_session(
        &self,
        user_name: &str,
        password: &str,
    ) -> Result<(Uuid, SessionToken, Duration)> {
        let user_id = self.auth_user(user_name, password).await?;

        let max_age = Duration::from_hours(24 * 365);
        let token = self.create_session(Some(user_id), max_age).await?;

        Ok((user_id, token, max_age))
    }

    pub async fn create_anonymous_session(&self) -> Result<SessionToken> {
        let max_age = Duration::from_hours(24);

        let token = self.create_session(None, max_age).await?;

        Ok(token)
    }

    pub async fn get_user_of_session(&self, session: &SessionToken) -> Result<Option<Uuid>> {
        let id = Session::filter_by_token(&session.0)
            .select(Session::fields().user_id())
            .one()
            .exec(&mut self.db.clone())
            .await?;

        Ok(id)
    }

    async fn create_session(
        &self,
        user_id: Option<Uuid>,
        max_age: Duration,
    ) -> Result<SessionToken> {
        let token = SessionToken::new();

        let now = Timestamp::now();
        let valid_until = now + max_age;

        create!(Session {
            token: &token.0,
            user_id,
            created_at: now,
            valid_until
        })
        .exec(&mut self.db.clone())
        .await?;

        Ok(token)
    }
}
