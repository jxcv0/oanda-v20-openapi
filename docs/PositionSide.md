# PositionSide

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**units** | Option<**f64**> | Number of units in the position (negative value indicates short position, positive indicates long position). | [optional]
**average_price** | Option<**f64**> | Volume-weighted average of the underlying Trade open prices for the Position. | [optional]
**trade_ids** | Option<**Vec<i32>**> | List of the open Trade IDs which contribute to the open Position. | [optional]
**pl** | Option<**f64**> | Profit/loss realized by the PositionSide over the lifetime of the Account. | [optional]
**unrealized_pl** | Option<**f64**> | The unrealized profit/loss of all open Trades that contribute to this PositionSide. | [optional]
**resettable_pl** | Option<**f64**> | Profit/loss realized by the PositionSide since the Account's resettablePL was last reset by the client. | [optional]
**financing** | Option<**f64**> | The total amount of financing paid/collected for this PositionSide over the lifetime of the Account. | [optional]
**guaranteed_execution_fees** | Option<**f64**> | The total amount of fees charged over the lifetime of the Account for the execution of guaranteed Stop Loss Orders attached to Trades for this PositionSide. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


