# \TransactionApi

All URIs are relative to *https://api-fxpractice.oanda.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_transaction**](TransactionApi.md#get_transaction) | **GET** /accounts/{accountId}/transactions/{transactionId} | Transaction Details
[**get_transaction_id_range**](TransactionApi.md#get_transaction_id_range) | **GET** /accounts/{accountId}/transactions/idrange | Transaction ID Range
[**get_transactions**](TransactionApi.md#get_transactions) | **GET** /accounts/{accountId}/transactions | List Transactions
[**get_transactions_since**](TransactionApi.md#get_transactions_since) | **GET** /accounts/{accountId}/transactions/sinceid | Transactions Since ID



## get_transaction

> models::TransactionResponse get_transaction(account_id, transaction_id, accept_datetime_format)
Transaction Details

Get the details of a single Account Transaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**transaction_id** | **i32** | A Transaction identifier | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |

### Return type

[**models::TransactionResponse**](TransactionResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transaction_id_range

> models::TransactionIdRangeResponse get_transaction_id_range(account_id, from, to, accept_datetime_format, r#type)
Transaction ID Range

Get a range of Transactions for an Account based on the Transaction IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**from** | **i32** | The starting Transacion ID (inclusive) to fetch. | [required] |
**to** | **i32** | The ending Transaction ID (inclusive) to fetch. | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |
**r#type** | Option<[**Vec<models::TransactionType>**](models::TransactionType.md)> | The filter that restricts the types of Transactions to retreive. |  |

### Return type

[**models::TransactionIdRangeResponse**](TransactionIdRangeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions

> models::TransactionsResponse get_transactions(account_id, accept_datetime_format, from, to, page_size, r#type)
List Transactions

Get a list of Transactions pages that satisfy a time-based Transaction query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |
**from** | Option<**String**> | The starting time (inclusive) of the time range for the Transactions being queried. |  |
**to** | Option<**String**> | The ending time (inclusive) of the time range for the Transactions being queried. |  |
**page_size** | Option<**i32**> | The number of Transactions to include in each page of the results. |  |
**r#type** | Option<[**Vec<models::TransactionType>**](models::TransactionType.md)> | A filter for restricting the types of Transactions to retreive. |  |

### Return type

[**models::TransactionsResponse**](TransactionsResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_transactions_since

> models::TransactionSinceIdResponse get_transactions_since(account_id, id, accept_datetime_format)
Transactions Since ID

Get a range of Transactions for an Account starting at (but not including) a provided Transaction ID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**id** | **i32** | The ID of the last Transacion fetched. This query will return all Transactions newer than the TransactionID. | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |

### Return type

[**models::TransactionSinceIdResponse**](TransactionSinceIdResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

