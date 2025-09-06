# OrderCancelRejectTransaction

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
**order_id** | Option<**i32**> | The ID of the Order cancelled | [optional]
**client_order_id** | Option<**String**> | The client ID of the Order cancelled (only provided if the Order has a client Order ID). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


