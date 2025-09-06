# MarketOrder

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
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


