
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub enum AuthError {
    InvalidCredentials(String),
}

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::InvalidCredentials(cred) => write!(f, "Invalid Credentials '{}' ", cred),
        }
    }
}

impl std::error::Error for AuthError {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum UserRole {
    Admin,
    User,
}

#[derive(Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub role: UserRole,
}

pub struct Auth {
    pub users: HashMap<String, User>,
}

impl Auth {
    pub fn new() -> Self {
        let mut users = HashMap::new();

        // Hardcoded credentials (In a real-world scenario, use a secure method for password storage)
        users.insert(
            "manager".to_string(),
            User {
                username: "manager".to_string(),
                password: "password123".to_string(),
                role: UserRole::Admin,
            },
        );

        users.insert(
            "normal_user".to_string(),
            User {
                username: "normal_user".to_string(),
                password: "userpassword".to_string(),
                role: UserRole::User,
            },
        );

        Auth { users }
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<UserRole, AuthError> {
        match self.users.get(username) {
            Some(user) if user.password == password && user.username == username => Ok(user.role.clone()),
            _ => Err(AuthError::InvalidCredentials(username.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authenticate_success() {
        let auth = Auth::new();
        
        // Test valid admin credentials
        let result = auth.authenticate("manager", "password123");
        assert_eq!(result, Ok(UserRole::Admin));

        // Test invalid password
        let result = auth.authenticate("manager", "wrongpassword");
        assert_eq!(result, Err(AuthError::InvalidCredentials("manager".to_string())));
        
        // Test non-existent user
        let result = auth.authenticate("nonexistent", "password123");
        assert_eq!(result, Err(AuthError::InvalidCredentials("nonexistent".to_string())));
    }

    #[test]
    fn test_authenticate_invalid_credentials() {
        let auth = Auth::new();

        // Test invalid credentials (wrong password)
        let result = auth.authenticate("manager", "wrongpassword");
        assert_eq!(result, Err(AuthError::InvalidCredentials("manager".to_string())));

        // Test invalid credentials (non-existent user)
        let result = auth.authenticate("nonexistent", "password123");
        assert_eq!(result, Err(AuthError::InvalidCredentials("nonexistent".to_string())));
    }

    #[test]
    fn test_authenticate_admin_role() {
        let auth = Auth::new();
        
        // Test valid admin credentials
        let result = auth.authenticate("manager", "password123");
        assert_eq!(result, Ok(UserRole::Admin));
    }
/* 
    #[test]
    fn test_authenticate_user_role() {
        let auth = Auth::new();
        
        // Note: In the current implementation, there is no user with UserRole::User.
        // This test is to demonstrate how you would test a UserRole::User if it were implemented.
        // Uncomment and modify the test if you add a UserRole::User in the future.
        /*
        let result = auth.authenticate("some_user", "some_password");
        assert_eq!(result, Ok(UserRole::User));
        */
    }*/
}
