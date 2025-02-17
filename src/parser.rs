/// Simplistic parser of lines describing an item to be added a cart.
use std::collections::VecDeque;

use super::{Item, Tax};

/// Parse an input line of the form: {item quantity} {item id} {item price} {tax}
pub fn parse(line: &String) -> Result<(u32, Item), &'static str> {
    let mut parts: VecDeque<&str> = line.split_whitespace().collect();
    let quantity = parts
        .pop_front()
        .ok_or_else(|| "Failed to parse quantity.")?
        .parse::<u32>()
        .map_err(|_| "Failed to parse quantity.")?;
    let tax_amount = parts
        .pop_back()
        .ok_or_else(|| "Failed to parse tax.")?
        .parse::<u8>()
        .map_err(|_| "Failed to parse tax.")?;
    let tax = match tax_amount {
        0 => Ok(Tax::Tax0),
        5 => Ok(Tax::Tax5),
        10 => Ok(Tax::Tax10),
        15 => Ok(Tax::Tax15),
        _ => Err("Invalid tax supplied."),
    }?;
    let price = parts
        .pop_back()
        .ok_or_else(|| "Failed to parse price")?
        .split_terminator('.')
        .collect::<String>()
        .parse::<u32>()
        .map_err(|_| "Failed to parse price.")?;
    let _ = parts.pop_back(); // drop the "..at.."
    let id = parts
        .into_iter()
        .collect::<Vec<&str>>()
        .as_slice()
        .join(" ");

    Ok((quantity, Item::new(id, price, tax)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_correct_input() {
        let input = "2 Book at 12.49 0".to_string();
        let (quantity, item) = parse(&input).unwrap();

        assert_eq!(2, quantity);
        assert_eq!("Book", item.id());
        assert_eq!(1249, item.price());
        assert_eq!(Tax::Tax0, item.tax());
    }

    #[test]
    fn test_parse_incorrect_input() {
        assert_eq!(
            "Failed to parse quantity.",
            parse(&"Book".to_string()).unwrap_err()
        );

        assert_eq!(
            "Failed to parse tax.",
            parse(&"1 Book".to_string()).unwrap_err()
        );

        assert_eq!(
            "Failed to parse price.",
            parse(&"1 Book 0".to_string()).unwrap_err()
        );
    }
}
