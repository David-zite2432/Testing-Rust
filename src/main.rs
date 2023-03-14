struct Providers {
    companie: String,
    products: Vec<String>,
    local: String,
    products_amount: u32
}

impl Providers {
    fn struct_fill(companie: &str, products: Vec<String>, local: &str, products_amount: u32) -> Providers {
        Providers { 
            companie: companie.to_string(), 
            products,
            local: local.to_string(), 
            products_amount 
        }
    }

    fn show_elements(self) {
        print!(
            "[{} provider]\n> Productos que provee: {:?}\n> Local: {}\n> Cantidad de productos al día: {}\n", 
            self.companie, self.products, self.local, self.products_amount
        );
    }
}


fn main() {
    println!("Wellcome to [A company project!]");

    let provider_a = Providers::struct_fill(
        "B companie", 
        vec![String::from("Café"), String::from("Leche"), String::from("Harina")], 
        "Sucre", 
        100
    );

    Providers::show_elements(provider_a);
}
