# \OrderApi

All URIs are relative to *https://api-fxpractice.oanda.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accounts_account_id_orders_order_specifier_put**](OrderApi.md#accounts_account_id_orders_order_specifier_put) | **PUT** /accounts/{accountId}/orders/{orderSpecifier} | Replace Order
[**cancel_order**](OrderApi.md#cancel_order) | **PUT** /accounts/{accountId}/orders/{orderSpecifier}/cancel | Cancel Order
[**create_order**](OrderApi.md#create_order) | **POST** /accounts/{accountId}/orders | Create Order
[**get_order**](OrderApi.md#get_order) | **GET** /accounts/{accountId}/orders/{orderSpecifier} | Get Order
[**get_orders**](OrderApi.md#get_orders) | **GET** /accounts/{accountId}/orders | List Orders
[**get_pending_orders**](OrderApi.md#get_pending_orders) | **GET** /accounts/{accountId}/pendingOrders | Pending Orders
[**set_order_extensions**](OrderApi.md#set_order_extensions) | **PUT** /accounts/{accountId}/orders/{orderSpecifier}/clientExtensions | Set Order Extensions



## accounts_account_id_orders_order_specifier_put

> models::ReplaceOrderResponse accounts_account_id_orders_order_specifier_put(account_id, order_specifier, replace_order_request, accept_datetime_format, client_request_id)
Replace Order

Replace an Order in an Account by simultaneously cancelling it and creating a replacement Order

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**order_specifier** | **String** | The Order Specifier | [required] |
**replace_order_request** | [**ReplaceOrderRequest**](ReplaceOrderRequest.md) | Specification of the replacing Order. The replacing order must have the same type as the replaced Order. | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |
**client_request_id** | Option<**String**> | Client specified RequestID to be sent with request. |  |

### Return type

[**models::ReplaceOrderResponse**](ReplaceOrderResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cancel_order

> models::CancelOrderResponse cancel_order(account_id, order_specifier, accept_datetime_format, client_request_id)
Cancel Order

Cancel a pending Order in an Account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**order_specifier** | **String** | The Order Specifier | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |
**client_request_id** | Option<**String**> | Client specified RequestID to be sent with request. |  |

### Return type

[**models::CancelOrderResponse**](CancelOrderResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_order

> models::CreateOrderResponse create_order(account_id, create_order_request, accept_datetime_format)
Create Order

Create an Order for an Account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**create_order_request** | [**CreateOrderRequest**](CreateOrderRequest.md) |  | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |

### Return type

[**models::CreateOrderResponse**](CreateOrderResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_order

> models::OrderResponse get_order(account_id, order_specifier, accept_datetime_format)
Get Order

Get details for a single Order in an Account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**order_specifier** | **String** | The Order Specifier | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |

### Return type

[**models::OrderResponse**](OrderResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_orders

> models::OrdersResponse get_orders(account_id, accept_datetime_format, ids, state, instrument, count, before_id)
List Orders

Get a list of Orders for an Account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |
**ids** | Option<[**Vec<i32>**](i32.md)> | List of Order IDs to retrieve |  |
**state** | Option<**String**> | The state to filter the requested Orders by |  |
**instrument** | Option<[**InstrumentName**](.md)> | The instrument to filter the requested orders by |  |
**count** | Option<**i32**> | The maximum number of Orders to return |  |
**before_id** | Option<**i32**> | The maximum Order ID to return. If not provided the most recent Orders in the Account are returned |  |

### Return type

[**models::OrdersResponse**](OrdersResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_pending_orders

> models::PendingOrdersResponse get_pending_orders(account_id, accept_datetime_format)
Pending Orders

List all pending Orders in an Account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |

### Return type

[**models::PendingOrdersResponse**](PendingOrdersResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_order_extensions

> models::OrderExtensionsResponse set_order_extensions(account_id, order_specifier, order_extensions_request, accept_datetime_format)
Set Order Extensions

Update the Client Extensions for an Order in an Account. Do notset, modify, or delete clientExtensions if your account is associated with MT4.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**order_specifier** | **String** | The Order Specifier | [required] |
**order_extensions_request** | [**OrderExtensionsRequest**](OrderExtensionsRequest.md) | Representation of the replacing Order | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |

### Return type

[**models::OrderExtensionsResponse**](OrderExtensionsResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

