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
        let mut result = Vec::new();
        let prices: Vec<f32> = self.items.iter().map(|(_, p)| *p).collect();
        let mut i = 0;

        while i + 2 < prices.len() {
            let group = &prices[i..i + 3];
            let total: f32 = group.iter().sum();
            let min_price = group.iter().cloned().fold(f32::INFINITY, f32::min);
            let discount_ratio = min_price / total;

            let mut discounted_group: Vec<f32> = group
                .iter()
                .map(|p| ((p - p * discount_ratio) * 100.0).round() / 100.0)
                .collect();

            result.append(&mut discounted_group);
            i += 3;
        }

        // Append remaining items (less than 3) with no discount
        for &p in &prices[i..] {
            result.push((p * 100.0).round() / 100.0);
        }

        result.sort_by(|a, b| a.partial_cmp(b).unwrap());
        self.receipt = result.clone();
        result
    }
}
