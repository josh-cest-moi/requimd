# Other items

## NFR-901 Time to show the total
ref: FR-902  prio: must
scale: ms from opening the cart to the displayed total, 95th percentile
meter: measurement with 100 carts in the test environment
must: 500

## BR-901 VAT rate
src: Tax law

The VAT rate is 19 %.

## US-901 Reorder
prio: could

As a customer, I want to reorder a past order so that I save time.

ac:
- Given a past order, when I choose reorder, then its items are in the cart.

## TC-901 Total includes VAT
ver: FR-902

Open a cart with one item for 100 EUR; the total shows 119 EUR.
