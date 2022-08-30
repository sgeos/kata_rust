// http://codekata.com/kata/kata01-supermarket-pricing/

use std::fmt;

struct QuantityForCurrency {
  quantity: i32,
  currency: i32,
}

struct CurrencyForUnit<'a> {
  currency: i32,
  unit: &'a str,
}

enum Price<'a> {
  Free,
  Currency(i32), // currency type could be added
  Unit(CurrencyForUnit<'a>), // unit could be added to everything
  Quantity(QuantityForCurrency),
}

impl fmt::Display for Price<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Price::Free => write!(f, "Free"),
      Price::Currency(x) => write!(f, "${}.{:0>2}", x / 100, x % 100),
      Price::Unit(CurrencyForUnit{currency: x, unit: u}) => write!(f, "{} per {}", Price::Currency(*x), u),
      Price::Quantity(QuantityForCurrency{quantity: x, currency: y}) =>
        write!(f, "{} for {}", x, Price::Currency(*y)),
    }
  }
}

struct Promotion<'a> {
  price: Price<'a>,
  buy_quantity: i32,
  free_quantity: i32,
}

impl fmt::Display for Promotion<'_> {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f, "Price: {}, buy {} get {} free",
      self.price, self.buy_quantity, self.free_quantity
    )
  }
}

#[no_mangle]
pub extern fn run() {
  println!("Price: {}", Price::Free);
  println!("Price: {}", Price::Currency(4999));
  println!("Price: {}", Price::Unit(CurrencyForUnit{currency:100,unit:"kg"}));
  println!("Price: {}", Price::Quantity(QuantityForCurrency{quantity:3,currency:1000}));
  println!("PROMOTION {}", Promotion{ price: Price::Free, buy_quantity: 1, free_quantity: 2} );
  println!("PROMOTION {}", Promotion{ price: Price::Unit(CurrencyForUnit{currency:9999,unit:"ton"}), buy_quantity: 1, free_quantity: 1} );
  println!("PROMOTION {}", Promotion{ price: Price::Currency(275), buy_quantity: 3, free_quantity: 2} );
  println!("PROMOTION {}", Promotion{ price: Price::Quantity(QuantityForCurrency{quantity:5,currency:1199}), buy_quantity: 5, free_quantity: 1} );
}

