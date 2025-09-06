# StopLossOrder

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
**price** | Option<**f64**> | The price threshold specified for the Stop Loss Order. If the guaranteed flag is false, the associated Trade will be closed by a market price that is equal to or worse than this threshold. If the flag is true the associated Trade will be closed at this price. | [optional]
**guaranteed_execution_premium** | Option<**f64**> | The premium that will be charged if the Stop Loss Order is guaranteed and the Order is filled at the guaranteed price. It is in price units and is charged for each unit of the Trade. | [optional]
**distance** | Option<**f64**> | Specifies the distance (in price units) from the Account's current price to use as the Stop Loss Order price. If the Trade is short the Instrument's bid price is used, and for long Trades the ask is used. | [optional]
**guaranteed** | Option<**bool**> | Flag indicating that the Stop Loss Order is guaranteed. The default value depends on the GuaranteedStopLossOrderMode of the account, if it is REQUIRED, the default will be true, for DISABLED or ENABLED the default is false. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


