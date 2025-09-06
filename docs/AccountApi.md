# \AccountApi

All URIs are relative to *https://api-fxpractice.oanda.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**configure_account**](AccountApi.md#configure_account) | **PATCH** /accounts/{accountId}/configuration | Configure Account
[**get_account**](AccountApi.md#get_account) | **GET** /accounts/{accountId} | Account Details
[**get_account_changes**](AccountApi.md#get_account_changes) | **GET** /accounts/{accountId}/changes | Poll Account Updates
[**get_account_instruments**](AccountApi.md#get_account_instruments) | **GET** /accounts/{accountId}/instruments | Account Instruments
[**get_account_summary**](AccountApi.md#get_account_summary) | **GET** /accounts/{accountId}/summary | Account Summary
[**get_accounts**](AccountApi.md#get_accounts) | **GET** /accounts | List Accounts



## configure_account

> models::AccountConfigurationResponse configure_account(account_id, accept_datetime_format, account_configuration_request)
Configure Account

Set the client-configurable portions of an Account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |
**account_configuration_request** | Option<[**AccountConfigurationRequest**](AccountConfigurationRequest.md)> | Representation of the Account configuration to set |  |

### Return type

[**models::AccountConfigurationResponse**](AccountConfigurationResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account

> models::AccountResponse get_account(account_id, accept_datetime_format)
Account Details

Get the full details for a single Account that a client has access to. Full pending Order, open Trade and open Position representations are provided.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |

### Return type

[**models::AccountResponse**](AccountResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_changes

> models::AccountChangesResponse get_account_changes(account_id, accept_datetime_format, since_transaction_id)
Poll Account Updates

Endpoint used to poll an Account for its current state and changes since a specified TransactionID.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |
**since_transaction_id** | Option<**i32**> | ID of the Transaction to get Account changes since. |  |

### Return type

[**models::AccountChangesResponse**](AccountChangesResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_instruments

> models::AccountInstrumentsResponse get_account_instruments(account_id, instruments)
Account Instruments

Get the list of tradeable instruments for the given Account. The list of tradeable instruments is dependent on the regulatory division that the Account is located in, thus should be the same for all Accounts owned by a single user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**instruments** | Option<[**Vec<models::InstrumentName>**](models::InstrumentName.md)> | List of instruments to query specifically. |  |

### Return type

[**models::AccountInstrumentsResponse**](AccountInstrumentsResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_summary

> models::AccountSummaryResponse get_account_summary(account_id, accept_datetime_format)
Account Summary

Get a summary for a single Account that a client has access to.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |

### Return type

[**models::AccountSummaryResponse**](AccountSummaryResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_accounts

> models::AccountsResponse get_accounts()
List Accounts

Get a list of all Accounts authorized for the provided token.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::AccountsResponse**](AccountsResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

