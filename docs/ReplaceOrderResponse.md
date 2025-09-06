# ReplaceOrderResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**error_code** | Option<**String**> | The code of the error that has occurred. This field may not be returned for some errors. | [optional]
**error_message** | Option<**String**> | The human-readable description of the error that has occurred. | [optional]
**last_transaction_id** | Option<**i32**> | The ID of the most recent Transaction created for the Account | [optional]
**related_transaction_ids** | Option<**Vec<i32>**> | The IDs of all Transactions that were created while satisfying the request. | [optional]
**order_cancel_transaction** | Option<[**models::OrderCancelTransaction**](OrderCancelTransaction.md)> |  | [optional]
**order_create_transaction** | Option<[**models::Transaction**](Transaction.md)> |  | [optional]
**order_fill_transaction** | Option<[**models::OrderFillTransaction**](OrderFillTransaction.md)> |  | [optional]
**order_reissue_transaction** | Option<[**models::Transaction**](Transaction.md)> |  | [optional]
**order_reissue_reject_transaction** | Option<[**models::Transaction**](Transaction.md)> |  | [optional]
**replacing_order_cancel_transaction** | Option<[**models::OrderCancelTransaction**](OrderCancelTransaction.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


