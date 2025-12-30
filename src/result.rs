use chrono::{DateTime, TimeZone, Utc};
use std::fmt;

pub fn result_main() {
    attempt_login("bob_is_secure");     // Insufficient permissions
    attempt_login("craig_is_secure");   // Expired Token
    attempt_login("hack3r");            // Invalid Token
    attempt_login("alice_is_secure");   // Success   
}

fn attempt_login(token: &str) {
    let request = LoginRequest {
        token: String::from(token)
    };

    match admin_login(&request) {
        Ok(_) => println!("{token} is logged in!"),
        Err(e) => println!("{token} failed to login: {e}")
    }
}

enum AccessError {
    InvalidToken,
    ExpiredToken,
    InsufficientPermissions
}

impl fmt::Display for AccessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AccessError::InvalidToken => write!(f, "Invalid Token"),
            AccessError::ExpiredToken => write!(f, "Expired Token"),
            AccessError::InsufficientPermissions => write!(f, "Insufficient Permissions")
        }
    }
}

#[derive(PartialEq)]
enum UserRole {
    User,
    Admin
}

struct LoginRequest {
    token: String
}

struct UserToken {
    token: String,
    role: UserRole,
    not_after: DateTime<Utc>
}

fn admin_login(request: &LoginRequest) -> Result<bool, AccessError> {
    let user_token_db = vec![
        UserToken { token: String::from("bob_is_secure"), role: UserRole::User, not_after: Utc.with_ymd_and_hms(2025, 12, 29, 12, 00, 00).unwrap() },    // Insufficient permissions (user)
        UserToken { token: String::from("craig_is_secure"), role: UserRole::Admin, not_after: Utc.with_ymd_and_hms(2025, 12, 28, 12, 00, 00).unwrap() }, // expired token
        UserToken { token: String::from("alice_is_secure"), role: UserRole::Admin, not_after: Utc.with_ymd_and_hms(2025, 12, 29, 12, 00, 00).unwrap() }  // success
    ];

    let now = Utc.with_ymd_and_hms(2025, 12, 29, 12, 00, 00).unwrap();

    let matching_token = user_token_db.iter().find(|t| t.token == request.token);

    // we can use pattern matching or ok_or syntax to transform the option into a result.
    // our pattern matching example relies on the fact each arm is evaluated in order, top to bottom.

    // match matching_token {
    //     None => Err(AccessError::InvalidToken),
    //     Some(t) if now > t.not_after => Err(AccessError::ExpiredToken),
    //     Some(t) if t.role != UserRole::Admin => Err(AccessError::InsufficientPermissions),
    //     _ => Ok(true)
    // }

    // or 

    matching_token.ok_or(AccessError::InvalidToken).and_then(|t| {
        if now > t.not_after { Err(AccessError::ExpiredToken) }
        else if t.role != UserRole::Admin { Err(AccessError::InsufficientPermissions) }
        else { Ok(true) }
    })
}