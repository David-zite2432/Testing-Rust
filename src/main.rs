use std::io::stdin;

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

    fn show_elements(&self) {
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

    Providers::show_elements(&provider_a);
    let providers = providers_data_based();

    let one_provider = return_one_provider(&providers);
    Providers::show_elements(&one_provider);
}

fn providers_data_based() -> Vec<Providers> {
    let mut provider_vec: Vec<Providers> = Vec::new();
    
    // Recive data until the q has presed
    loop {
        println!("Desea agregar otro proveedor a la base de datos [Q] para salir [Y] para continuar");
        let mut response = String::new();
        stdin().read_line(&mut response).expect("No se pudo leer la línea");

        if  response.trim().to_lowercase().eq("q"){
            break;
        }

        // Companie
        println!("Nombre de la compañía proveedora");
        let mut companie = String::new();
        stdin().read_line(&mut companie).expect("No se pudo leer la línea");

        // Products
        println!("Ingrese los productos separado por espacios");
        let mut products_value = String::new();
        stdin().read_line(&mut products_value).expect("No se pudo leer la línea");
        let products_str: Vec<&str> = products_value.split_whitespace().collect();
        let products = products_str.iter().map(|&s| s.to_string()).collect();

        // Local
        println!("Ingrese el lugar en donde queda la compañía proveedora");
        let mut local = String::new();
        stdin().read_line(&mut local).expect("No se pudo leer la línea");

        // amount_products
        println!("Ingrese la cantidad de productos que le provee diario");
        let mut amount_products_value = String::new();
        stdin().read_line(&mut amount_products_value).expect("No se pudo leer la línea");
        let products_amount: u32 = amount_products_value.trim().parse().expect("No se pudo parsear el valor");

        let provider = Providers {
            companie,
            products,
            local,
            products_amount
        };

        provider_vec.push(provider);
    }

    provider_vec
}

fn return_one_provider(providers: &Vec<Providers>) -> Providers {
    // Show all providers saved in the database
    for (iter, element) in providers.iter().enumerate() {
        println!("[{}]: {}", iter, element.companie);
    }

    // Request option
    println!("Eliga el proveedor que decea mostrar");
    let mut option_value = String::new();
    stdin().read_line(&mut option_value).expect("No se pudo leer la línea");
    let option: usize = option_value.trim().parse().expect("No se pudo parsear el valor");

    providers[option]
}
