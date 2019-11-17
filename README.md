# Test BE lastminute

## Problem: Sales Taxes

Basic sales tax is applicable at a rate of 10% on all goods, except books, food, and medical products that are exempt. Import duty is an additional sales tax applicable on all imported goods at a rate of 5%, with no exemptions.

When I purchase items I receive a receipt which lists the name of all the items and their price (including tax), finishing with the total cost of the items, and the total amounts of sales taxes paid. The rounding rules for sales tax are that for a tax rate of n%, a shelf price of p contains (np/100 rounded up to the nearest 0.05) amount of sales tax.

## Requirements

Download the program from the releases section, or download a stable version of the Rust compiler from [rustup.rs](https://rustup.rs/) and run the program with `cargo run` instead of `program`.

``` shell
$ ./program < input.txt

2 Book 24.98
3 Music CD 49.47
1 Imported Box of Chocolates 10.50
2 Imported Music CDs 34.48
4 Chocolate Bar 3.40
Sales Taxes 9.50
Total 122.83
```

## Implementation notes

The `input.txt` file has been extended to include the tax for each item. This is done in order to simplify parsing. With more information on the grammar of the input though improving the parser to automatically calculate the tax from the kind of good would be trivial.





