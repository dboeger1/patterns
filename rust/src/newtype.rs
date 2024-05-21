#[cfg(test)]
mod tests {
    use std::{
        any::Any,
        fmt::Display,
    };


    #[test]
    fn example_tuple() {
        println!("\n=======================================================\n");

        #[derive(Debug)]
        struct NewType(bool);

        let wrapper = NewType(true);
        println!("tuple wrapper: {:?}", wrapper);

        println!("\n=======================================================\n");
    }


    #[test]
    fn example_struct() {
        println!("\n=======================================================\n");

        #[derive(Debug)]
        struct NewType{_value: bool}

        let wrapper = NewType{_value: true};
        println!("struct wrapper: {:?}", wrapper);

        println!("\n=======================================================\n");
    }


    #[test]
    fn example_differentiation() {
        println!("\n=======================================================\n");

        #[derive(Debug, PartialEq)]
        struct NewType1(bool);

        #[derive(Debug, PartialEq)]
        struct NewType2(bool);

        let wrapper1 = NewType1(true);
        println!("NewType1 ID: {:?}", wrapper1.type_id());
        println!("wrapper1: {:?}", wrapper1);

        let wrapper2 = NewType2(true);
        println!("\nNewType2 ID: {:?}", wrapper2.type_id());
        println!("wrapper2: {:?}", wrapper2);

        assert!(wrapper1.type_id() != wrapper2.type_id());

        println!("\n=======================================================\n");
    }


    #[test]
    fn example_orphan_rule_bypass() {
        println!("\n=======================================================\n");

        let tuple = ();
        // println!("{tuple}");

        // impl Display for () {}

        #[derive(Debug)]
        struct NewType(());
        impl Display for NewType {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "()")
            }
        }

        let wrapper = NewType(tuple);
        println!("wrapper debug: {:?}", wrapper);
        println!("wrapper display: {wrapper}");

        println!("\n=======================================================\n");
    }
}
