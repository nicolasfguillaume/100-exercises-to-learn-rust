// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u32
}

// fn takes an immutable reference to a `String` as an argument, bound to the name `product_name`
// &String is a pointer to the memory location where the String's metadata is stored.
pub fn validate_product_name(product_name: &String) {
    if product_name.is_empty() {
        panic!("The product name can't be empty");
    }
    if product_name.len() > 300 {
        panic!("The product name can't be longer than 300 bytes");
    }
}

pub fn validate_quantity(quantity: &u32) {
    // use * to dereference the borrow
    if *quantity <= 0 {
        panic!("The quantity must be strictly greater than zero");
    }
}

pub fn validate_unit_price(unit_price: &u32) {
    // use * to dereference the borrow
    if *unit_price <= 0 {
        panic!("The unit price must be strictly greater than zero");
    }
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u32) -> Order {
        validate_product_name(&product_name);
        validate_quantity(&quantity);
        validate_unit_price(&unit_price);

        Order {
            product_name: product_name,
            quantity: quantity,
            unit_price: unit_price
        }
    }

    // Order must include a method named `total` that returns the total price of the order.
    pub fn total(&self) -> u32 {
        &self.quantity * &self.unit_price
    }

    // Borrow `self` to read the value, rather than taking ownership of it.
    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u32 {
        &self.unit_price
    }

    // Borrow `self` to modify the value, rather than taking ownership of it.
    pub fn set_product_name(&mut self, product_name: String) {
        validate_product_name(&product_name); // product_name is borrowed using & (bc the ownership is not needed here)
        self.product_name = product_name;
    }

    pub fn set_quantity(&mut self, quantity: u32) {
        validate_quantity(&quantity); // quantity is borrowed using &
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: u32) {
        validate_unit_price(&unit_price); // unit_price is borrowed using &
        self.unit_price = unit_price;
    }
}
