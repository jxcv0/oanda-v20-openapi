# TransactionsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**to** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**page_size** | Option<**i32**> | The pageSize provided in the request | [optional]
**r#type** | Option<[**Vec<models::TransactionType>**](TransactionType.md)> | The Transaction-type filter provided in the request | [optional]
**count** | Option<**i32**> | The number of Transactions that are contained in the pages returned | [optional]
**pages** | Option<**Vec<String>**> | The list of URLs that represent idrange queries providing the data for each page in the query results | [optional]
**last_transaction_id** | Option<**i32**> | The ID of the most recent Transaction created for the Account | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


