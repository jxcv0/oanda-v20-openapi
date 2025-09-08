# CloseTradeResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**last_transaction_id** | Option<**String**> | The ID of the most recent Transaction created for the Account | [optional]
**related_transaction_ids** | Option<**Vec<i32>**> | The IDs of all Transactions that were created while satisfying the request. | [optional]
**order_create_transaction** | Option<[**models::MarketOrderTransaction**](MarketOrderTransaction.md)> |  | [optional]
**order_fill_transaction** | Option<[**models::OrderFillTransaction**](OrderFillTransaction.md)> |  | [optional]
**order_cancel_transaction** | Option<[**models::OrderCancelTransaction**](OrderCancelTransaction.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


