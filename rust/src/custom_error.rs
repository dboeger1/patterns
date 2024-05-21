use std::{
    error::Error,
    fmt::Display,
};


#[derive(Debug)]
pub struct CustomError {
    pub(crate) message: String,
    pub(crate) source: Option<Box<dyn Error>>,
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for CustomError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self
            .source
            .as_ref()
            .map(|error| error.as_ref())
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    trait GenerateError {
        fn generate_error(&self) -> Result<(), Box<dyn Error>>;
    }


    struct ErrorGenerator();
    impl GenerateError for ErrorGenerator {
        fn generate_error(&self) -> Result<(), Box<dyn Error>> {
            Err(Box::new(CustomError {
                message: "Custom error message".to_string(),
                source: None,
            }))
        }
    }


    #[test]
    fn example() {
        println!("\n=======================================================\n");

        let error_generator = ErrorGenerator();
        let result = error_generator.generate_error();
        println!("result: {:?}", result);
        assert!(result.is_err());
        println!("error: {:?}", result.unwrap_err());

        println!("\n=======================================================\n");
    }
}
