mod utils;
use utils::{ structurs::* };

pub fn return_provider(companie: &str, products: Vec<String>, local: &str, products_amount: u32) -> Providers {
    Providers::struct_fill(companie, products, local, products_amount)
}

pub fn show_value(provider: &Providers) -> String {
    let result = Providers::show_elements(provider);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn comprobate_fill_providers() {
        let provider_to_comparate = return_provider(
            "B companie", 
            vec![String::from("Café"), String::from("Leche"), String::from("Harina")], 
            "Sucre", 
            100
        );

        let provider = Providers {
           companie: "B companie".to_string(), 
           products: vec![String::from("Café"), String::from("Leche"), String::from("Harina")], 
           local: "Sucre".to_string(), 
           products_amount: 100
        };

        assert_eq!(provider_to_comparate, provider, "[La Estructura debería de ser de tipo Providers]");
    }

    #[test]
    fn comprobate_outpud() {
        let provider = Providers {
            companie: "B companie".to_string(), 
            products: vec![String::from("Café"), String::from("Leche"), String::from("Harina")], 
            local: "Sucre".to_string(), 
            products_amount: 100
        };

        let result = show_value(&provider);
        let espected_output = format!(
            "[{} provider]\n> Productos que provee: {:?}\n> Local: {}\n> Cantidad de productos al día: {}\n", 
            provider.companie, provider.products, provider.local, provider.products_amount
        );

        assert_eq!(result, espected_output);
    }
}
