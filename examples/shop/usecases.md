# Use cases

## UC-012 Place order
actor: Customer  prio: must

pre: Customer is logged in.

- s1: Customer opens the cart.
- s2: System shows the total incl. VAT.
- s3: Customer confirms the order.
- s2a: Cart is empty → system shows a hint, use case ends.

post: Order is stored with status "new".
