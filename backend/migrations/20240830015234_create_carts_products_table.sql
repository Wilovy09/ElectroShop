-- Add migration script here

CREATE TABLE CartProduct (
  cartId INTEGER NOT NULL,
  productId INTEGER NOT NULL,
  quantity INTEGER NOT NULL CHECK(quantity > 0),
  PRIMARY KEY (cartId, productId),
  FOREIGN KEY (cartId) REFERENCES Cart(id),
  FOREIGN KEY (productId) REFERENCES Product(id)
);
