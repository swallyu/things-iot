pub mod server;

#[cfg(test)]
mod tests {
    use super::*;

    struct Sample{
        name:String
    }

    impl Sample{
        fn New(name:String){
            Sample{name:name}
        }

        fn Say(&self){
            println!(self.name);
        }
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        println!("SS");
    }
}
