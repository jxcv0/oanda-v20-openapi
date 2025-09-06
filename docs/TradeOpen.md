# TradeOpen

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**trade_id** | Option<**i32**> | The ID of the Trade that was opened | [optional]
**units** | Option<**f64**> | The number of units opened by the Trade | [optional]
**price** | Option<**f64**> | The average price that the units were opened at. | [optional]
**guaranteed_execution_fee** | Option<**f64**> | This is the fee charged for opening the trade if it has a guaranteed Stop Loss Order attached to it. | [optional]
**client_extensions** | Option<[**models::ClientExtensions**](ClientExtensions.md)> |  | [optional]
**half_spread_cost** | Option<**f64**> | The half spread cost for the trade open. This can be a positive or negative value and is represented in the home currency of the Account. | [optional]
**initial_margin_required** | Option<**f64**> | The margin required at the time the Trade was created. Note, this is the 'pure' margin required, it is not the 'effective' margin used that factors in the trade risk if a GSLO is attached to the trade. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


