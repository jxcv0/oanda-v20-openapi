# TrailingStopLossOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The Order's identifier, unique within the Order's Account. | [optional]
**create_time** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**state** | Option<[**models::OrderState**](OrderState.md)> |  | [optional]
**client_extensions** | Option<[**models::ClientExtensions**](ClientExtensions.md)> |  | [optional]
**r#type** | Option<**String**> | The type of the Order. | [optional]
**trade_id** | Option<**i32**> | The ID of the Trade to close when the price threshold is breached. | [optional]
**client_trade_id** | Option<**String**> | The client ID of the Trade to be closed when the price threshold is breached. | [optional]
**time_in_force** | Option<[**models::TradeOrderTimeInForce**](TradeOrderTimeInForce.md)> |  | [optional]
**gtd_time** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**trigger_condition** | Option<[**models::TradeOrderTriggerCondition**](TradeOrderTriggerCondition.md)> |  | [optional]
**filling_transaction_id** | Option<**i32**> | ID of the Transaction that filled this Order (only provided when the Order's state is FILLED) | [optional]
**filled_time** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**trade_opened_id** | Option<**i32**> | Trade ID of Trade opened when the Order was filled (only provided when the Order's state is FILLED and a Trade was opened as a result of the fill) | [optional]
**trade_reduced_id** | Option<**i32**> | Trade ID of Trade reduced when the Order was filled (only provided when the Order's state is FILLED and a Trade was reduced as a result of the fill) | [optional]
**trade_closed_ids** | Option<**Vec<i32>**> | Trade IDs of Trades closed when the Order was filled (only provided when the Order's state is FILLED and one or more Trades were closed as a result of the fill) | [optional]
**cancelling_transaction_id** | Option<**i32**> | ID of the Transaction that cancelled the Order (only provided when the Order's state is CANCELLED) | [optional]
**cancelled_time** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**replaces_order_id** | Option<**i32**> | The ID of the Order that was replaced by this Order (only provided if this Order was created as part of a cancel/replace). | [optional]
**replaced_by_order_id** | Option<**i32**> | The ID of the Order that replaced this Order (only provided if this Order was cancelled as part of a cancel/replace). | [optional]
**distance** | Option<**f64**> | The price distance (in price units) specified for the TrailingStopLoss Order. | [optional]
**trailing_stop_value** | Option<**f64**> | The trigger price for the Trailing Stop Loss Order. The trailing stop value will trail (follow) the market price by the TSL order's configured \"distance\" as the market price moves in the winning direction. If the market price moves to a level that is equal to or worse than the trailing stop value, the order will be filled and the Trade will be closed. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


