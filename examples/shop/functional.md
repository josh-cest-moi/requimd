# Functional requirements

## FR-101 Show cart
impl: UC-012.s1  prio: must

When the customer opens the cart, the system shall list all selected items
with quantity and price.

## FR-102 Order confirmation
impl: UC-012.s3  prio: must

When the customer confirms the order, the system shall store the order with
status "new" and send a confirmation e-mail.

## FR-103 Empty cart hint
impl: UC-012.s2a  prio: should

If the cart contains no items, then the system shall show a hint and a link
to the product catalogue.

## FR-104 VAT per line item
impl: UC-012.s2  prio: must  status: review

When the cart is opened, the system shall calculate VAT per line item based
on the delivery country (see BR-003).

## FR-105 Missing delivery country
impl: UC-012.s2  prio: should

If no delivery country is known, then the system shall use the billing
country for VAT.
