use std::error::Error;


pub trait ValueConstruct: Sized {
    type ValueType;

    fn new(value: Self::ValueType) -> Result<Self, Box<dyn Error>>;
}


#[cfg(test)]
mod tests {
    use crate::custom_error::CustomError;
    use super::*;
    

    #[test]
    fn example_basic() {
        println!("\n=======================================================\n");

        #[derive(Debug)]
        struct Wrapper<T>(T);
        impl<T> ValueConstruct for Wrapper<T> {
            type ValueType = T;

            fn new(value: Self::ValueType) -> Result<Self, Box<dyn Error>> {
                Ok(Self(value))
            }
        }

        let result = Wrapper::new(true);
        println!("result: {:?}", result);
        assert!(result.is_ok());
        println!("wrapper: {:?}", result.unwrap());

        println!("\n=======================================================\n");
    }


    #[test]
    fn example_constraint() {
        println!("\n=======================================================\n");

        #[derive(Debug)]
        struct NonEmpty(String);
        impl ValueConstruct for NonEmpty {
            type ValueType = String;

            fn new(value: Self::ValueType) -> Result<Self, Box<dyn Error>> {
                if value.is_empty() {
                    return Err(Box::new(CustomError {
                        message: "Empty string".to_string(),
                        source: None,
                    }));
                }

                Ok(Self(value))
            }
        }


        let result = NonEmpty::new("".to_string());
        println!("result: {:?}", result);
        assert!(result.is_err());
        println!("error: {:?}", result.unwrap_err());

        println!("\n=======================================================\n");
    }
}
