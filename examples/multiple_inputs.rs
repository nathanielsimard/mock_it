use mock_it::*;

trait Validator {
    fn validate_credentials(&self, username: &String, password: &String) -> Result<(), String>;
}

struct ValidatorMock {
    validate_credentials: Mock<(String, String), Result<(), String>>,
}

impl Validator for ValidatorMock {
    fn validate_credentials(&self, username: &String, password: &String) -> Result<(), String> {
        self.validate_credentials
            .called((username.clone(), password.clone()))
    }
}

impl ValidatorMock {
    fn new() -> ValidatorMock {
        ValidatorMock {
            validate_credentials: Mock::new(),
        }
    }
}

fn main() {
    let mock = ValidatorMock::new();
    let valid_username = "username".to_string();
    let valid_password = "1234".to_string();
    let invalid_username = "".to_string();
    let invalid_password = "".to_string();

    mock.validate_credentials
        .given((valid_username.clone(), valid_password.clone()))
        .will_return(Ok(()));
    mock.validate_credentials
        .given((valid_username.clone(), invalid_password.clone()))
        .will_return(Err("Invalid password".to_string()));
    mock.validate_credentials
        .given((invalid_username.clone(), valid_password.clone()))
        .will_return(Err("Invalid username".to_string()));
    mock.validate_credentials
        .given((invalid_username.clone(), invalid_password.clone()))
        .will_return(Err("Invalid username and password".to_string()));

    let validator = Box::new(mock);

    assert_eq!(
        Err("Invalid password".to_string()),
        validator.validate_credentials(&valid_username, &invalid_password)
    );
    assert_eq!(
        Err("Invalid username and password".to_string()),
        validator.validate_credentials(&invalid_username, &invalid_password)
    );
    assert_eq!(
        Err("Invalid username".to_string()),
        validator.validate_credentials(&invalid_username, &valid_password)
    );
    assert_eq!(
        Ok(()),
        validator.validate_credentials(&valid_username, &valid_password)
    );

    assert!(verify(validator.validate_credentials.was_called_with((
        valid_username.clone(),
        valid_password.clone()
    ))));
    assert!(verify(validator.validate_credentials.was_called_with((
        valid_username.clone(),
        invalid_password.clone()
    ))));
    assert!(verify(validator.validate_credentials.was_called_with((
        invalid_username.clone(),
        valid_password.clone()
    ))));
    assert!(verify(validator.validate_credentials.was_called_with((
        invalid_username.clone(),
        invalid_password.clone()
    ))));
}
