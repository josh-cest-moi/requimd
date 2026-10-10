# Intentionally broken

## UC-001 Checkout
actor: Customer

- s1: Customer opens the cart.
- s2: Customer pays.
- s2: Customer gets a receipt.

## FR-001 Fast checkout
impl: UC-001.s1  prio: urgent  colour: red

The checkout should be fast.

## FR-001 Payment
impl: UC-001.s9

When the customer pays, the system shall charge the card.

## FR-002 Receipt
impl: FR-001

When the payment succeeded, the system shall show a receipt. Refunds are
handled as in FR-999.

## FR-? Basket summary

The system shall show a summary of the basket.

## NFR-001 Performance
prio: must

The system shall respond quickly.

```markdown
## FR-001 Inside a code block, must be ignored
impl: UC-404.s1
```

Inline code like `FR-888` must be ignored as well.

## FR-003 Refund
impl: FR-002

When a refund is requested, the system shall return the full amount.

## FR-004 Order page link
impl: UC-001.s1

The Fast Track page shall load the order form. <!-- reqmd: allow fast -->
The order form shall be easy to use. <!-- reqmd: allow quick -->
