extern crate chrono;
extern crate cron_parser;

pub mod offer;
pub mod order;
pub mod treat;

#[cfg(test)]
mod tests {
    use super::*;
    const COOKIE: &str = "Cookie";
    const BROWNIE: &str = "Brownie";
    const DONUT: &str = "Mini Gingerbread Donut";
    const CHEESECAKE: &str = "Key Lime Cheesecake";

    // Since there's no way of having setup and tearDown for tests - I have included all the tests in one.
    // (To avoid parsing json data everytime for each test)
    // Open Issue - https://github.com/rust-lang/rfcs/issues/1664
    #[test]
    fn cai_bakery_tests() {
        //Load up treats
        let treats = treat::read_treats("input_treats.json").unwrap();
        let mut treat_map = std::collections::HashMap::new();
        for treat in treats.iter() {
            treat_map.insert(&treat.name, treat);
        }

        //Load up offers
        let offers = offer::read_offers("input_offer.json").unwrap();
        let mut offer_map = std::collections::HashMap::new();
        for offer in offers.iter() {
            offer_map.insert(&offer.item, offer);
        }

        //**********************************************************
        // Test cases
        // Test case 1 : Without any offer, Brownies X 9
        let mut expected_result = 16.0;
        let mut items = vec![order::Tuple {
            name: BROWNIE.to_string(),
            unit: 9,
        }];
        let mut actual_result =
            order::execute_order(&String::from(""), &items, &treat_map, &offer_map);
        assert_eq!(actual_result, expected_result);

        //Test case 2 : Without any offer, Cookie X 1, Brownies X 4, Cheesecake X 1
        expected_result = 16.25;
        items = vec![
            order::Tuple {
                name: COOKIE.to_string(),
                unit: 1,
            },
            order::Tuple {
                name: BROWNIE.to_string(),
                unit: 4,
            },
            order::Tuple {
                name: CHEESECAKE.to_string(),
                unit: 1,
            },
        ];
        actual_result = order::execute_order(&String::from(""), &items, &treat_map, &offer_map);
        assert_eq!(actual_result, expected_result);

        // Test case 3: Without any offer, Cookies X 8
        expected_result = 8.50;
        items = vec![order::Tuple {
            name: COOKIE.to_string(),
            unit: 8,
        }];
        actual_result = order::execute_order(&String::from(""), &items, &treat_map, &offer_map);
        assert_eq!(actual_result, expected_result);

        // Test case 4: Without any offer, Cookie X 1, Brownie X 1, Cheesecake X 1 and Donuts X 2
        expected_result = 12.25;
        items = vec![
            order::Tuple {
                name: COOKIE.to_string(),
                unit: 1,
            },
            order::Tuple {
                name: BROWNIE.to_string(),
                unit: 1,
            },
            order::Tuple {
                name: CHEESECAKE.to_string(),
                unit: 1,
            },
            order::Tuple {
                name: DONUT.to_string(),
                unit: 2,
            },
        ];
        actual_result = order::execute_order(&String::from(""), &items, &treat_map, &offer_map);
        assert_eq!(actual_result, expected_result);

        // Test case 5: On October 1st 2021, order: Cookies X 8 and Cheesecakes X 4
        expected_result = 30.00;
        items = vec![
            order::Tuple {
                name: COOKIE.to_string(),
                unit: 8,
            },
            order::Tuple {
                name: CHEESECAKE.to_string(),
                unit: 4,
            },
        ];
        actual_result = order::execute_order(
            &String::from("2021/10/1 00:00:00"),
            &items,
            &treat_map,
            &offer_map,
        );
        assert_eq!(actual_result, expected_result);

        // Additional test cases
        // Test case 6: Empty order and Empty date
        expected_result = 0.0;
        items = Vec::new();
        actual_result = order::execute_order(&String::from(""), &items, &treat_map, &offer_map);
        assert_eq!(actual_result, expected_result);

        // Test case 7: Empty order on January 1, 2021
        expected_result = 0.0;
        actual_result = order::execute_order(
            &String::from("2021/01/01 00:00:00"),
            &items,
            &treat_map,
            &offer_map,
        );
        assert_eq!(actual_result, expected_result);

        // Test case 8: On October 6th 2020, order: Donuts X 16
        expected_result = 8.0;
        items = vec![order::Tuple {
            name: DONUT.to_string(),
            unit: 32,
        }];
        actual_result = order::execute_order(
            &String::from("2020/10/6 00:00:00"),
            &items,
            &treat_map,
            &offer_map,
        );
        assert_eq!(actual_result, expected_result);

        // Test case 9: order: Donuts X 16
        expected_result = 16.0;
        items = vec![order::Tuple {
            name: DONUT.to_string(),
            unit: 32,
        }];
        actual_result = order::execute_order(&String::from(""), &items, &treat_map, &offer_map);
        assert_eq!(actual_result, expected_result);
    }
}
