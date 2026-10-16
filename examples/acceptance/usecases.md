# Use cases

## UC-901 Order goods
actor: Customer

pre: The customer is logged in.

- s1: Customer opens the cart.
- s2: System shows the total.
- s2a: The cart is empty → system shows a hint, use case ends.

post: The customer knows the total.
