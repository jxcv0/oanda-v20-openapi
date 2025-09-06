# StopLossOrderTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The Transaction's Identifier. | [optional]
**time** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**user_id** | Option<**i32**> | The ID of the user that initiated the creation of the Transaction. | [optional]
**account_id** | Option<**String**> | The Account's identifier | [optional]
**batch_id** | Option<**i32**> | The ID of the \"batch\" that the Transaction belongs to. Transactions in the same batch are applied to the Account simultaneously. | [optional]
**request_id** | Option<**String**> | The Request ID of the request which generated the transaction. | [optional]
**r#type** | Option<[**models::TransactionType**](TransactionType.md)> |  | [optional]
**trade_id** | Option<**i32**> | The ID of the Trade to close when the price threshold is breached. | [optional]
**client_trade_id** | Option<**String**> | The client ID of the Trade to be closed when the price threshold is breached. | [optional]
**price** | Option<**f64**> | The price threshold specified for the Stop Loss Order. If the guaranteed flag is false, the associated Trade will be closed by a market price that is equal to or worse than this threshold. If the flag is true the associated Trade will be closed at this price. | [optional]
**distance** | Option<**f64**> | Specifies the distance (in price units) from the Account's current price to use as the Stop Loss Order price. If the Trade is short the Instrument's bid price is used, and for long Trades the ask is used. | [optional]
**time_in_force** | Option<[**models::TradeOrderTimeInForce**](TradeOrderTimeInForce.md)> |  | [optional]
**gtd_time** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**trigger_condition** | Option<[**models::TradeOrderTriggerCondition**](TradeOrderTriggerCondition.md)> |  | [optional]
**guaranteed** | Option<**bool**> | Flag indicating that the Stop Loss Order is guaranteed. The default value depends on the GuaranteedStopLossOrderMode of the account, if it is REQUIRED, the default will be true, for DISABLED or ENABLED the default is false. | [optional]
**guaranteed_execution_premium** | Option<**f64**> | The fee that will be charged if the Stop Loss Order is guaranteed and the Order is filled at the guaranteed price. The value is determined at Order creation time. It is in price units and is charged for each unit of the Trade. | [optional]
**reason** | Option<[**models::TradeOrderTransactionReason**](TradeOrderTransactionReason.md)> |  | [optional]
**client_extensions** | Option<[**models::ClientExtensions**](ClientExtensions.md)> |  | [optional]
**order_fill_transaction_id** | Option<**i32**> | The ID of the OrderFill Transaction that caused this Order to be created (only provided if this Order was created automatically when another Order was filled). | [optional]
**replaces_order_id** | Option<**i32**> | The ID of the Order that this Order replaces (only provided if this Order replaces an existing Order). | [optional]
**cancelling_transaction_id** | Option<**i32**> | The ID of the Transaction that cancels the replaced Order (only provided if this Order replaces an existing Order). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


