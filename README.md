# tiger-hall
Adobe CAI Bakery challenge
==========================

To run the application:
1. Download the github repo.
2. From the tiger-hall/cai-bakery directory. 
	a. Run > cargo build.
	b. Run > cargo test.

All tests must pass successfully. 
If needed I can also devise this as a normal application with entry point from main.rs.

Designing the cart application of a bakery store
================================================

1. The items' description are stored in input_treats.json and offer are stored in input_offer.json. Both of these are loaded at first when the application is run into structs Treat and Offer respectively.

2. For the first version of this application, we are assuming every treat is going to have only one offer at a time - we can extend this to multiple offers and optimize to utilize the best offer in the later versions.

3. order::execute_order is the entry point to the application. If provided with an empty order_date, we assume there is no offer to be applied. If not, we check for existing offers. It returns the total price, with or without offers for the current order.

4. The check to verify whether an order can be applied or not is based out of a cron expression - we are assuming the right cron expression for the corresponding offer is entered in the json file. More on cron expressions here: https://crontab.guru/

Things to improve on in version 2.0:
====================================

1. Better error handling and logging.

2. Setup and tearDown methods for the tests. Issue tracker here - https://github.com/rust-lang/rfcs/issues/1664
    
3. Having 3 different types of structs for the different types of Offer, and parse them appropriately. Refer to PR: (Couldn't incorporate due to the shortcomings of serde in v1.). Ensures better modularisation, and extensibility.

4. Getter/setter methods for the structs.