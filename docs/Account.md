# Account

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**String**> | The Account's identifier | [optional]
**alias** | Option<**String**> | Client-assigned alias for the Account. Only provided if the Account has an alias set | [optional]
**currency** | Option<[**models::AccountCurrency**](AccountCurrency.md)> |  | [optional]
**balance** | Option<**f64**> | The current balance of the Account. | [optional]
**created_by_user_id** | Option<**i32**> | ID of the user that created the Account. | [optional]
**created_time** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**guaranteed_stop_loss_order_mode** | Option<[**models::AccountGuaranteedStopLossOrderMode**](AccountGuaranteedStopLossOrderMode.md)> |  | [optional]
**pl** | Option<**f64**> | The total profit/loss realized over the lifetime of the Account. | [optional]
**resettable_pl** | Option<**f64**> | The total realized profit/loss for the Account since it was last reset by the client. | [optional]
**resettable_pl_time** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**financing** | Option<**f64**> | The total amount of financing paid/collected over the lifetime of the Account. | [optional]
**commission** | Option<**f64**> | The total amount of commission paid over the lifetime of the Account. | [optional]
**guaranteed_execution_fees** | Option<**f64**> | The total amount of fees charged over the lifetime of the Account for the execution of guaranteed Stop Loss Orders. | [optional]
**margin_rate** | Option<**f64**> | Client-provided margin rate override for the Account. The effective margin rate of the Account is the lesser of this value and the OANDA margin rate for the Account's division. This value is only provided if a margin rate override exists for the Account. | [optional]
**margin_call_enter_time** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**margin_call_extension_count** | Option<**i32**> | The number of times that the Account's current margin call was extended. | [optional]
**last_margin_call_extension_time** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**open_trade_count** | Option<**i32**> | The number of Trades currently open in the Account. | [optional]
**open_position_count** | Option<**i32**> | The number of Positions currently open in the Account. | [optional]
**pending_order_count** | Option<**i32**> | The number of Orders currently pending in the Account. | [optional]
**hedging_enabled** | Option<**bool**> | Flag indicating that the Account has hedging enabled. | [optional]
**last_order_fill_timestamp** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**last_transaction_id** | Option<**i32**> | The ID of the last Transaction created for the Account. | [optional]
**unrealized_pl** | Option<**f64**> | The total unrealized profit/loss for all Trades currently open in the Account. | [optional]
**nav** | Option<**f64**> | The net asset value of the Account. Equal to Account balance unrealizedPL. | [optional]
**margin_used** | Option<**f64**> | Margin currently used for the Account. | [optional]
**margin_available** | Option<**f64**> | Margin available for Account currency. | [optional]
**position_value** | Option<**f64**> | The value of the Account's open positions represented in the Account's home currency. | [optional]
**margin_closeout_unrealized_pl** | Option<**f64**> | The Account's margin closeout unrealized PL. | [optional]
**margin_closeout_nav** | Option<**f64**> | The Account's margin closeout NAV. | [optional]
**margin_closeout_margin_used** | Option<**f64**> | The Account's margin closeout margin used. | [optional]
**margin_closeout_percent** | Option<**f64**> | The Account's margin closeout percentage. When this value is 1.0 or above the Account is in a margin closeout situation. | [optional]
**margin_closeout_position_value** | Option<**f64**> | The value of the Account's open positions as used for margin closeout calculations represented in the Account's home currency. | [optional]
**withdrawal_limit** | Option<**f64**> | The current WithdrawalLimit for the account which will be zero or a positive value indicating how much can be withdrawn from the account. | [optional]
**margin_call_margin_used** | Option<**f64**> | The Account's margin call margin used. | [optional]
**margin_call_percent** | Option<**f64**> | The Account's margin call percentage. When this value is 1.0 or above the Account is in a margin call situation. | [optional]
**trades** | Option<[**Vec<models::TradeSummary>**](TradeSummary.md)> | The details of the Trades currently open in the Account. | [optional]
**positions** | Option<[**Vec<models::Position>**](Position.md)> | The details all Account Positions. | [optional]
**orders** | Option<[**Vec<models::Order>**](Order.md)> | The details of the Orders currently pending in the Account. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


