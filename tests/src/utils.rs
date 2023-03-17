pub mod structurs {
    #[derive(Debug)]
    #[derive(PartialEq)]
    pub struct Providers {
        pub companie: String,
        pub products: Vec<String>,
        pub local: String,
        pub products_amount: u32
    }

    impl Providers {
        pub fn struct_fill(companie: &str, products: Vec<String>, local: &str, products_amount: u32) -> Providers {
            Providers { 
                companie: companie.to_string(), 
                products,
                local: local.to_string(), 
                products_amount 
            }
        }

        pub fn show_elements(&self) -> String {
            format!(
                "[{} provider]\n> Productos que provee: {:?}\n> Local: {}\n> Cantidad de productos al día: {}\n", 
                self.companie, self.products, self.local, self.products_amount
            )
        }
    }
}