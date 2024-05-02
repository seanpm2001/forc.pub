use super::error::DatabaseError;
use super::{models, schema, DbConn};
use super::{string_to_uuid, Database};
use diesel::prelude::*;

use rand::{distributions::Uniform, rngs::OsRng, Rng};
use sha2::{Digest, Sha256};

use uuid::Uuid;

/// NEVER CHANGE THE PREFIX OF EXISTING TOKENS!!! Doing so will implicitly
/// revoke all the tokens, disrupting production users.
const TOKEN_PREFIX: &str = "pub_";
const TOKEN_LENGTH: usize = 32;

impl DbConn {
    /// Creates an API token for the user and returns the token.
    pub fn new_token(
        &mut self,
        user_id: Uuid,
        friendly_name: String,
    ) -> Result<(models::Token, String), DatabaseError> {
        let plain_token = generate_token();
        let token = Sha256::digest(plain_token.as_bytes()).as_slice().to_vec();

        let new_token = models::NewToken {
            user_id,
            friendly_name,
            token,
            expires_at: None,
        };

        // Insert new session
        let saved_token = diesel::insert_into(schema::api_tokens::table)
            .values(&new_token)
            .returning(models::Token::as_returning())
            .get_result(self.inner())
            .map_err(|_| DatabaseError::InsertTokenFailed(user_id.to_string()))?;

        Ok((saved_token, plain_token))
    }

    /// Deletes an API token for the user.
    pub fn delete_token(&mut self, user_id: Uuid, token_id: String) -> Result<(), DatabaseError> {
        let token_uuid = string_to_uuid(token_id.clone())?;

        diesel::delete(
            schema::api_tokens::table
                .filter(schema::api_tokens::id.eq(token_uuid))
                .filter(schema::api_tokens::user_id.eq(user_id)),
        )
        .execute(self.inner())
        .map_err(|_| DatabaseError::NotFound(token_id))?;

        Ok(())
    }

    /// Fetch all tokens for the given user ID.
    pub fn get_tokens_for_user(&mut self, user_id: Uuid) -> Result<Vec<models::Token>, DatabaseError> {
        schema::api_tokens::table
            .filter(schema::api_tokens::user_id.eq(user_id))
            .select(models::Token::as_returning())
            .load(self.inner())
            .map_err(|_| DatabaseError::NotFound(user_id.to_string()))
    }
}

fn generate_secure_alphanumeric_string(len: usize) -> String {
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    OsRng
        .sample_iter(Uniform::from(0..CHARS.len()))
        .map(|idx| CHARS[idx] as char)
        .take(len)
        .collect()
}

fn generate_token() -> String {
    format!(
        "{}{}",
        TOKEN_PREFIX,
        generate_secure_alphanumeric_string(TOKEN_LENGTH)
    )
}
