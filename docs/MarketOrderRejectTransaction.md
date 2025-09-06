# MarketOrderRejectTransaction

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
**reject_reason** | Option<**String**> | The reason that the Reject Transaction was created | [optional]
**instrument** | Option<[**models::InstrumentName**](InstrumentName.md)> |  | [optional]
**units** | Option<**f64**> | The quantity requested to be filled by the Market Order. A posititive number of units results in a long Order, and a negative number of units results in a short Order. | [optional]
**time_in_force** | Option<[**models::MarketOrderTradeClose**](MarketOrderTradeClose.md)> |  | [optional]
**price_bound** | Option<**f64**> | The worst price that the client is willing to have the Market Order filled at. | [optional]
**position_fill** | Option<[**models::MarketOrderPositionFill**](MarketOrderPositionFill.md)> |  | [optional]
**trade_close** | Option<[**models::MarketOrderTradeClose**](MarketOrder_tradeClose.md)> |  | [optional]
**long_position_closeout** | Option<[**models::MarketOrderPositionCloseout**](MarketOrderPositionCloseout.md)> |  | [optional]
**short_position_closeout** | Option<[**models::MarketOrderPositionCloseout**](MarketOrderPositionCloseout.md)> |  | [optional]
**margin_closeout** | Option<[**models::MarketOrderMarginCloseout**](MarketOrderMarginCloseout.md)> |  | [optional]
**delayed_trade_close** | Option<[**models::MarketOrderDelayedTradeClose**](MarketOrderDelayedTradeClose.md)> |  | [optional]
**reason** | Option<[**models::MarketOrderReason**](MarketOrderReason.md)> |  | [optional]
**client_extensions** | Option<[**models::ClientExtensions**](ClientExtensions.md)> |  | [optional]
**take_profit_on_fill** | Option<[**models::TakeProfitDetails**](TakeProfitDetails.md)> |  | [optional]
**stop_loss_on_fill** | Option<[**models::StopLossDetails**](StopLossDetails.md)> |  | [optional]
**trailing_stop_loss_on_fill** | Option<[**models::TrailingStopLossDetails**](TrailingStopLossDetails.md)> |  | [optional]
**trade_client_extensions** | Option<[**models::ClientExtensions**](ClientExtensions.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


