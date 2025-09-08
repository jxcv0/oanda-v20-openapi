use oanda_v20_openapi::models::{AccountInstrumentsResponse, InstrumentName, InstrumentType};
use serde_json::json;

#[test]
fn parsing_account_instruments_json() {
    let json = json!({
        "instruments": [
            {
                "name": "EUR_SGD",
                "type": "CURRENCY",
                "displayName": "EUR/SGD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "ALLOWED",
                "minimumGuaranteedStopLossDistance": "0.0025",
                "guaranteedStopLossOrderExecutionPremium": "0.00029999999998",
                "guaranteedStopLossOrderLevelRestriction": {
                    "volume": "1000000",
                    "priceRange": "0.01"
                },
                "tags": [
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    },
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    }
                ],
                "financing": {
                    "longRate": "-0.0081",
                    "shortRate": "-0.0163",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "XAG_GBP",
                "type": "METAL",
                "displayName": "Silver/GBP",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "500000",
                "marginRate": "0.10",
                "guaranteedStopLossOrderMode": "DISABLED",
                "tags": [
                    {
                        "type": "ASSET_CLASS",
                        "name": "COMMODITY"
                    },
                    {
                        "type": "KID_ASSET_CLASS",
                        "name": "METAL"
                    },
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "METAL"
                    }
                ],
                "financing": {
                    "longRate": "-0.032",
                    "shortRate": "-0.0136",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "EUR_AUD",
                "type": "CURRENCY",
                "displayName": "EUR/AUD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.05",
                "guaranteedStopLossOrderMode": "ALLOWED",
                "minimumGuaranteedStopLossDistance": "0.0010",
                "guaranteedStopLossOrderExecutionPremium": "0.001",
                "guaranteedStopLossOrderLevelRestriction": {
                    "volume": "1000000",
                    "priceRange": "0.0025"
                },
                "tags": [
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    },
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    }
                ],
                "financing": {
                    "longRate": "-0.0266",
                    "shortRate": "0.0059",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            },
            {
                "name": "USD_CAD",
                "type": "CURRENCY",
                "displayName": "USD/CAD",
                "pipLocation": -4,
                "displayPrecision": 5,
                "tradeUnitsPrecision": 0,
                "minimumTradeSize": "1",
                "maximumTrailingStopDistance": "1.00000",
                "minimumTrailingStopDistance": "0.00050",
                "maximumPositionSize": "0",
                "maximumOrderUnits": "100000000",
                "marginRate": "0.03333333333333",
                "guaranteedStopLossOrderMode": "ALLOWED",
                "minimumGuaranteedStopLossDistance": "0.0010",
                "guaranteedStopLossOrderExecutionPremium": "0.00029999999998",
                "guaranteedStopLossOrderLevelRestriction": {
                    "volume": "1000000",
                    "priceRange": "0.0025"
                },
                "tags": [
                    {
                        "type": "ASSET_CLASS",
                        "name": "CURRENCY"
                    },
                    {
                        "type": "BRAIN_ASSET_CLASS",
                        "name": "FX"
                    }
                ],
                "financing": {
                    "longRate": "0.0069",
                    "shortRate": "-0.0275",
                    "financingDaysOfWeek": [
                        {
                            "dayOfWeek": "MONDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "TUESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "WEDNESDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "THURSDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "FRIDAY",
                            "daysCharged": 1
                        },
                        {
                            "dayOfWeek": "SATURDAY",
                            "daysCharged": 0
                        },
                        {
                            "dayOfWeek": "SUNDAY",
                            "daysCharged": 0
                        }
                    ]
                }
            }
        ],
        "lastTransactionID": "7"
    });

    let res: AccountInstrumentsResponse = serde_json::from_value(json).unwrap();
    let Some(instruments) = res.instruments else {
        panic!();
    };

    let Some(ltid) = res.last_transaction_id else {
        panic!();
    };

    // Top-level sanity checks on the parsed model
    assert_eq!(instruments.len(), 4);
    assert_eq!(ltid, "7");

    // JSON-level asserts (avoids guessing exact Rust field types of the generated model)
    let eur_sgd = &instruments[0];

    // Instrument identity & formatting
    assert_eq!(eur_sgd.name, Some(InstrumentName::EurSgd));
    assert_eq!(eur_sgd.display_name, Some("EUR/SGD".to_string()));
    assert_eq!(eur_sgd.r#type, Some(InstrumentType::Currency));

    assert_eq!(eur_sgd.pip_location, Some(-4));
    assert_eq!(eur_sgd.display_precision, Some(5));
    assert_eq!(eur_sgd.trade_units_precision, Some(0));
    assert_eq!(eur_sgd.minimum_trade_size, Some("1".to_string()));

    assert_eq!(eur_sgd.maximum_trailing_stop_distance, Some("1.00000".to_string()));

    // Financing details
    // let fin = &eur_sgd.margin_rate;
    // assert_eq!(fin["shortRate"], "-0.0163");
    //
    // // Financing days-of-week
    // let fdow = fin["financingDaysOfWeek"].as_array().expect("fdow array");
    // assert_eq!(fdow.len(), 7);
    // assert_eq!(fdow[0]["dayOfWeek"], "MONDAY");
    // assert_eq!(fdow[0]["daysCharged"], 1);
    // assert_eq!(fdow[6]["dayOfWeek"], "SUNDAY");
    // assert_eq!(fdow[6]["daysCharged"], 0);
}
