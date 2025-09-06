# \TradeApi

All URIs are relative to *https://api-fxpractice.oanda.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**close_trade**](TradeApi.md#close_trade) | **PUT** /accounts/{accountId}/trades/{tradeSpecifier}/close | Close Trade
[**get_open_trades**](TradeApi.md#get_open_trades) | **GET** /accounts/{accountId}/openTrades | List Open Trades
[**get_trade**](TradeApi.md#get_trade) | **GET** /accounts/{accountId}/trades/{tradeSpecifier} | Trade Details
[**get_trades**](TradeApi.md#get_trades) | **GET** /accounts/{accountId}/trades | List Trades
[**set_trade_extensions**](TradeApi.md#set_trade_extensions) | **PUT** /accounts/{accountId}/trades/{tradeSpecifier}/clientExtensions | Set Trade Client Extensions
[**set_trade_orders**](TradeApi.md#set_trade_orders) | **PUT** /accounts/{accountId}/trades/{tradeSpecifier}/orders | Set Dependent Orders



## close_trade

> models::CloseTradeResponse close_trade(account_id, trade_specifier, close_trade_request, accept_datetime_format)
Close Trade

Close (partially or fully) a specific open Trade in an Account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**trade_specifier** | **String** | Specifier for the Trade | [required] |
**close_trade_request** | [**CloseTradeRequest**](CloseTradeRequest.md) | Details of how much of the open Trade to close. | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |

### Return type

[**models::CloseTradeResponse**](CloseTradeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_open_trades

> models::OpenTradeResponse get_open_trades(account_id, accept_datetime_format)
List Open Trades

Get the list of open Trades for an Account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |

### Return type

[**models::OpenTradeResponse**](OpenTradeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trade

> models::TradeResponse get_trade(account_id, trade_specifier, accept_datetime_format)
Trade Details

Get the details of a specific Trade in an Account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**trade_specifier** | **String** | Specifier for the Trade | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |

### Return type

[**models::TradeResponse**](TradeResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_trades

> models::TradesResponse get_trades(account_id, accept_datetime_format, ids, state, instrument, count, before_id)
List Trades

Get a list of Trades for an Account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |
**ids** | Option<[**Vec<i32>**](i32.md)> | List of Trade IDs to retrieve. |  |
**state** | Option<**String**> | The state to filter the requested Trades by. |  |
**instrument** | Option<[**InstrumentName**](.md)> | The instrument to filter the requested Trades by. |  |
**count** | Option<**i32**> | The maximum number of Trades to return. |  |
**before_id** | Option<**i32**> | The maximum Trade ID to return. If not provided the most recent Trades in the Account are returned. |  |

### Return type

[**models::TradesResponse**](TradesResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_trade_extensions

> models::TradeExtensionsResponse set_trade_extensions(account_id, trade_specifier, trade_extensions_request, accept_datetime_format)
Set Trade Client Extensions

Update the Client Extensions for a Trade. Do not add, update, or delete the Client Extensions if your account is associated with MT4.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**trade_specifier** | **String** | Specifier for the Trade | [required] |
**trade_extensions_request** | [**TradeExtensionsRequest**](TradeExtensionsRequest.md) | Details of how to modify the Trade's Client Extensions. | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |

### Return type

[**models::TradeExtensionsResponse**](TradeExtensionsResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_trade_orders

> models::DependentTradeOrdersResponse set_trade_orders(account_id, trade_specifier, dependent_trade_orders_request, accept_datetime_format)
Set Dependent Orders

Create, replace and cancel a Trade's dependent Orders (Take Profit, Stop Loss and Trailing Stop Loss) through the Trade itself

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**trade_specifier** | **String** | Specifier for the Trade | [required] |
**dependent_trade_orders_request** | [**DependentTradeOrdersRequest**](DependentTradeOrdersRequest.md) | Details of how to modify the Trade's dependent Orders. | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |

### Return type

[**models::DependentTradeOrdersResponse**](DependentTradeOrdersResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

