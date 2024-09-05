-- Add migration script here

CREATE TABLE CartProduct (
  cart_id INTEGER NOT NULL,
  product_id INTEGER NOT NULL,
  quantity INTEGER NOT NULL CHECK(quantity > 0),
  PRIMARY KEY (cart_id, product_id),
  FOREIGN KEY (cart_id) REFERENCES Cart(id),
  FOREIGN KEY (product_id) REFERENCES Product(id)
);
