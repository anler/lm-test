use std::collections::HashMap;

use super::Item;

/// A shopping cart that keeps track of items and their taxes.
pub struct Cart {
    items: Vec<Item>,
    quantities: HashMap<Item, u32>,
}

impl Cart {
    /// Create a cart with enough capacity to avoid unnecessary resizing.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
            quantities: HashMap::with_capacity(capacity),
        }
    }

    /// Create a new cart.
    pub fn new() -> Self {
        Self::with_capacity(2)
    }

    /// Add Items to the cart.
    pub fn add(&mut self, amount: u32, item: Item) {
        let quantity = self.quantities.entry(item.clone()).or_default();

        if *quantity == 0 {
            self.items.push(item);
        }

        *quantity = quantity.saturating_add(amount)
    }

    /// Return the cart receipt.
    ///
    /// None means that some price overflowed.
    pub fn receipt(&self) -> Option<Vec<(String, f64)>> {
        let (total, total_taxes) = self.totals()?;
        let mut receipt: Vec<(String, f64)> = Vec::new();

        for item in self.items.iter() {
            let quantity = self
                .quantities
                .get(item)
                .expect("invariant: item has no quantity");
            let (total, _) = self.item_price(item, *quantity)?;

            receipt.push((format!("{} {}", quantity, item.id()), as_real(total)));
        }

        receipt.push(("Sales Taxes".to_string(), as_real(total_taxes)));
        receipt.push(("Total".to_string(), as_real(total)));

        Some(receipt)
    }

    /// Return the total amount for the items in the cart.
    ///
    /// None means that some price overflowed.
    pub fn total(&self) -> Option<f64> {
        // let (total, _) = self.totals()?;
        self.totals().map(|(total, _)| as_real(total))
        // let total = price.checked_add(taxes)?;

        // Some(total as f64 / 100f64)
    }

    /// Return the part of total that are taxes.
    ///
    /// None means that some price overflowed.
    pub fn total_taxes(&self) -> Option<f64> {
        self.totals().map(|(_, taxes)| as_real(taxes))
    }

    fn item_price(&self, item: &Item, quantity: u32) -> Option<(u32, u32)> {
        let price = item.price().checked_mul(quantity)?;
        let tax = item.tax().apply(price);
        let total = price.checked_add(tax)?;

        Some((total, tax))
    }

    fn totals(&self) -> Option<(u32, u32)> {
        let mut total_price = 0u32;
        let mut total_taxes = 0u32;

        for (item, quantity) in self.quantities.iter() {
            let (total, tax) = self.item_price(item, *quantity)?;

            total_price = total_price.checked_add(total)?;
            total_taxes = total_taxes.checked_add(tax)?;
        }

        Some((total_price, total_taxes))
    }
}

impl From<Vec<(u32, Item)>> for Cart {
    fn from(items: Vec<(u32, Item)>) -> Self {
        let mut cart = Cart::with_capacity(items.len());

        for (quantity, item) in items {
            cart.add(quantity, item);
        }

        cart
    }
}

/// Utility function to turn a given amount of cents into a real
/// quantity.
#[inline]
fn as_real(cents: u32) -> f64 {
    cents as f64 / 100f64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::*;

    #[test]
    fn test_cart() {
        let mut cart = Cart::new();

        cart.add(1, Item::new("Book", 1249, Tax::Tax0));
        cart.add(2, Item::new("Book", 1249, Tax::Tax0));

        assert_eq!(1, cart.items.len());
        assert_eq!(1, cart.quantities.len());
    }

    #[test]
    fn test_example_input_1() {
        let cart = Cart::from(vec![
            (1, Item::new("Book", 1249, Tax::Tax0)),
            (1, Item::new("Music CD", 1499, Tax::Tax10)),
            (1, Item::new("Chocolate Bar", 85, Tax::Tax0)),
        ]);

        let receipt = vec![
            ("1 Book".to_string(), 12.49),
            ("1 Music CD".to_string(), 16.49),
            ("1 Chocolate Bar".to_string(), 0.85),
            ("Sales Taxes".to_string(), 1.50),
            ("Total".to_string(), 29.83),
        ];

        assert_eq!(receipt, cart.receipt().unwrap());
        assert_eq!(1.50, cart.total_taxes().unwrap());
        assert_eq!(29.83, cart.total().unwrap());
    }

    #[test]
    fn test_example_input_2() {
        let cart = Cart::from(vec![
            (1, Item::new("Imported Box of Chocolates", 1000, Tax::Tax5)),
            (1, Item::new("Imported Bottle of Perfume", 4750, Tax::Tax15)),
        ]);

        let receipt = vec![
            ("1 Imported Box of Chocolates".to_string(), 10.50),
            ("1 Imported Bottle of Perfume".to_string(), 54.65),
            ("Sales Taxes".to_string(), 7.65),
            ("Total".to_string(), 65.15),
        ];

        assert_eq!(receipt, cart.receipt().unwrap());
        assert_eq!(7.65, cart.total_taxes().unwrap());
        assert_eq!(65.15, cart.total().unwrap());
    }

    #[test]
    fn test_example_input_3() {
        let cart = Cart::from(vec![
            (1, Item::new("Imported Bottle of Perfume", 2799, Tax::Tax15)),
            (1, Item::new("Bottle of Perfume", 1899, Tax::Tax10)),
            (1, Item::new("Packet of Headache Pills", 975, Tax::Tax0)),
            (1, Item::new("Box of Imported Chocolates", 1125, Tax::Tax5)),
        ]);

        let receipt = vec![
            ("1 Imported Bottle of Perfume".to_string(), 32.19),
            ("1 Bottle of Perfume".to_string(), 20.89),
            ("1 Packet of Headache Pills".to_string(), 9.75),
            ("1 Box of Imported Chocolates".to_string(), 11.85),
            ("Sales Taxes".to_string(), 6.70),
            ("Total".to_string(), 74.68),
        ];

        assert_eq!(receipt, cart.receipt().unwrap());
        assert_eq!(6.70, cart.total_taxes().unwrap());
        assert_eq!(74.68, cart.total().unwrap());
    }

    #[test]
    fn test_example_input_4() {
        let cart = Cart::from(vec![
            (1, Item::new("Book", 1249, Tax::Tax0)),
            (1, Item::new("Book", 1249, Tax::Tax0)),
            (3, Item::new("Music CD", 1499, Tax::Tax10)),
            (4, Item::new("Chocolate Bar", 85, Tax::Tax0)),
        ]);

        let receipt = vec![
            ("2 Book".to_string(), 24.98),
            ("3 Music CD".to_string(), 49.47),
            ("4 Chocolate Bar".to_string(), 3.40),
            ("Sales Taxes".to_string(), 4.50),
            ("Total".to_string(), 77.85),
        ];

        assert_eq!(receipt, cart.receipt().unwrap());
        assert_eq!(4.50, cart.total_taxes().unwrap());
        assert_eq!(77.85, cart.total().unwrap());
    }
}
