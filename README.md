# Trading engine. 

Technical evaluation.

## Backend Rust

### Introduction

A trading engine is responsible for processing the orders that users create in an exchange. The trading-engine has a main component called orderbook. An orderbook is responsible for storing the orders until they match or are canceled by the user

An orderbook stores orders in two large groups, sell orders and buy orders. Sell orders are always accessed from cheapest to most expensive and buy orders from most expensive to cheapest. Orders can be buy or sell. Each time a new order enters it is evaluated against the opposite side of the orderbook if it has a chance to match

### Clarification

A buy order matches all sell orders with the same or lower price. A sell order matches all buy orders with the same or higher price. If there is a match, the order that matched the new order from the orderbook is removed. If there is no match, the order must be stored in its corresponding side

Each match is called a trade and normally the information of the orders involved is stored in a structure called trades.

### Requirements

Implement a trading engine in Rust
The algorithm has to process the orders from the file orders.json, which is attached in the mail
As a result of the execution, the matching algorithm has to generate 2 files, one called orderbook.json and another trades.json
There must be at least one test that evaluates a simple trade
Any extra that you can offer and that shows your knowledge and technical skills is valued.
The solution has to be shared via Github.


### See also
https://www.investopedia.com/terms/o/order-book.asp
