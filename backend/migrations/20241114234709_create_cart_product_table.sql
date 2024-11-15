-- Add migration script here
CREATE TABLE CartProduct (
    id INTEGER PRIMARY KEY NOT NULL,
    cart_id INTEGER,
    product_id INTEGER,
    quantity INTEGER NOT NULL, -- Cantidad de producto en el carrito
    FOREIGN KEY (cart_id) REFERENCES Cart(id),
    FOREIGN KEY (product_id) REFERENCES Product(id)
);
