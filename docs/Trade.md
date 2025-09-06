# Trade

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The Trade's identifier, unique within the Trade's Account. | [optional]
**instrument** | Option<[**models::InstrumentName**](InstrumentName.md)> |  | [optional]
**price** | Option<**f64**> | The execution price of the Trade. | [optional]
**open_time** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**state** | Option<[**models::TradeState**](TradeState.md)> |  | [optional]
**initial_units** | Option<**f64**> | The initial size of the Trade. Negative values indicate a short Trade, and positive values indicate a long Trade. | [optional]
**initial_margin_required** | Option<**f64**> | The margin required at the time the Trade was created. Note, this is the 'pure' margin required, it is not the 'effective' margin used that factors in the trade risk if a GSLO is attached to the trade. | [optional]
**current_units** | Option<**f64**> | The number of units currently open for the Trade. This value is reduced to 0.0 as the Trade is closed. | [optional]
**realized_pl** | Option<**f64**> | The total profit/loss realized on the closed portion of the Trade. | [optional]
**unrealized_pl** | Option<**f64**> | The unrealized profit/loss on the open portion of the Trade. | [optional]
**margin_used** | Option<**f64**> | Margin currently used by the Trade. | [optional]
**average_close_price** | Option<**f64**> | The average closing price of the Trade. Only present if the Trade has been closed or reduced at least once. | [optional]
**closing_transaction_ids** | Option<**Vec<i32>**> | The IDs of the Transactions that have closed portions of this Trade. | [optional]
**financing** | Option<**f64**> | The financing paid/collected for this Trade. | [optional]
**close_time** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**client_extensions** | Option<[**models::ClientExtensions**](ClientExtensions.md)> |  | [optional]
**take_profit_order** | Option<[**models::TakeProfitOrder**](TakeProfitOrder.md)> |  | [optional]
**stop_loss_order** | Option<[**models::StopLossOrder**](StopLossOrder.md)> |  | [optional]
**trailing_stop_loss_order** | Option<[**models::TrailingStopLossOrder**](TrailingStopLossOrder.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


