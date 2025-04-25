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

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        if let Some((_, price)) = s.products.iter().find(|(name, _)| *name == ele) {
            self.items.push((ele, *price));
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut prices: Vec<f32> = self.items.iter().map(|(_, price)| *price).collect();
        let mut final_receipt = Vec::new();

        prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut i = 0;
        while i + 2 < prices.len() {
            let group = &mut prices[i..i + 3].to_vec();
            let min_price = *group.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
            let total: f32 = group.iter().sum();
            let discount_ratio = min_price / total;
            let discounted: Vec<f32> = group
                .iter()
                .map(|p| ((p - (p * discount_ratio)) * 100.0).round() / 100.0)
                .collect();
            final_receipt.extend(discounted);
            i += 3;
        }

        // Remaining items that don't form a group of 3
        for j in i..prices.len() {
            final_receipt.push(((prices[j]) * 100.0).round() / 100.0);
        }

        final_receipt.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt = final_receipt.clone();
        final_receipt
    }
}
