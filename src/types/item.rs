use std::hash;

use super::Tax;

/// An Item is a good that can be bought.
#[derive(Debug, Clone)]
pub struct Item {
    id: String,
    price: u32,
    tax: Tax,
}

impl Item {
    /// Create a new Item.
    pub fn new<T>(id: T, price: u32, tax: Tax) -> Self
    where
        T: Into<String>,
    {
        Self {
            id: id.into(),
            price,
            tax,
        }
    }

    /// Return this Item's id.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Return this Item's price.
    pub fn price(&self) -> u32 {
        self.price
    }

    /// Return this Item's applicable tax.
    pub fn tax(&self) -> Tax {
        self.tax
    }
}

impl hash::Hash for Item {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Item {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item() {
        let book = Item::new("Book", 1249, Tax::Tax0);

        assert_eq!("Book", book.id());
        assert_eq!(Tax::Tax0, book.tax());
        assert_eq!(1249, book.price());
    }
}
