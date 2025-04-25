#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, store: &Store, product_name: String) {
        if let Some(product) = store.products.iter().find(|(name, _)| *name == product_name) {
            self.items.push((product.0.clone(), product.1));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let free_items = prices.len() / 3;
        let original_total: f32 = prices.iter().sum();
        let discount_total: f32 = prices.iter().take(free_items).sum();
        let discounted_total = original_total - discount_total;

        let sum_before_adjustment: f32 = prices.iter().sum();
        let adjustment_factor = discounted_total / sum_before_adjustment;

        let mut adjusted_prices: Vec<f32> = prices
            .iter()
            .map(|&price| (price * adjustment_factor * 100.0).round() / 100.0)
            .collect();

        adjusted_prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        self.receipt = adjusted_prices.clone();
        adjusted_prices
    }
}