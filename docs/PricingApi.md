# \PricingApi

All URIs are relative to *https://api-fxpractice.oanda.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_pricing**](PricingApi.md#get_pricing) | **GET** /accounts/{accountId}/pricing | Current Account Prices



## get_pricing

> models::PricingResponse get_pricing(account_id, instruments, accept_datetime_format, since, include_units_available, include_home_conversions)
Current Account Prices

Get pricing information for a specified list of Instruments within an Account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**instruments** | [**Vec<models::InstrumentName>**](models::InstrumentName.md) | List of Instruments to get pricing for. | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |
**since** | Option<**String**> | Date/Time filter to apply to the response. Only prices and home conversions (if requested) with a time later than this filter (i.e. the price has changed after the since time) will be provided, and are filtered independently. |  |
**include_units_available** | Option<**bool**> | Flag that enables the inclusion of the unitsAvailable field in the returned Price objects. |  |
**include_home_conversions** | Option<**bool**> | Flag that enables the inclusion of the homeConversions field in the returned response. An entry will be returned for each currency in the set of all base and quote currencies present in the requested instruments list. |  |

### Return type

[**models::PricingResponse**](PricingResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

