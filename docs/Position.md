# Position

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**instrument** | Option<[**models::InstrumentName**](InstrumentName.md)> |  | [optional]
**pl** | Option<**f64**> | Profit/loss realized by the Position over the lifetime of the Account. | [optional]
**unrealized_pl** | Option<**f64**> | The unrealized profit/loss of all open Trades that contribute to this Position. | [optional]
**margin_used** | Option<**f64**> | Margin currently used by the Position. | [optional]
**resettable_pl** | Option<**f64**> | Profit/loss realized by the Position since the Account's resettablePL was last reset by the client. | [optional]
**financing** | Option<**f64**> | The total amount of financing paid/collected for this instrument over the lifetime of the Account. | [optional]
**commission** | Option<**f64**> | The total amount of commission paid for this instrument over the lifetime of the Account. | [optional]
**guaranteed_execution_fees** | Option<**f64**> | The total amount of fees charged over the lifetime of the Account for the execution of guaranteed Stop Loss Orders for this instrument. | [optional]
**long** | Option<[**models::PositionSide**](PositionSide.md)> |  | [optional]
**short** | Option<[**models::PositionSide**](PositionSide.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


