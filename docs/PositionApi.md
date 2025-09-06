# \PositionApi

All URIs are relative to *https://api-fxpractice.oanda.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**close_position**](PositionApi.md#close_position) | **PUT** /accounts/{accountId}/positions/{instrument}/close | Close Position
[**get_instrument_position**](PositionApi.md#get_instrument_position) | **GET** /accounts/{accountId}/positions/{instrument} | Instrument Position
[**get_open_positions**](PositionApi.md#get_open_positions) | **GET** /accounts/{accountId}/openPositions | Open Positions
[**get_positions**](PositionApi.md#get_positions) | **GET** /accounts/{accountId}/positions | List Positions



## close_position

> models::ClosePositionResponse close_position(account_id, instrument, close_position_request, accept_datetime_format)
Close Position

Closeout the open Position for a specific instrument in an Account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**instrument** | [**InstrumentName**](.md) | Instrument name | [required] |
**close_position_request** | [**ClosePositionRequest**](ClosePositionRequest.md) | Representation of how to close the position | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |

### Return type

[**models::ClosePositionResponse**](ClosePositionResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instrument_position

> models::InstrumentPositionResponse get_instrument_position(account_id, instrument)
Instrument Position

Get the details of a single Instrument's Position in an Account. The Position may by open or not.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |
**instrument** | [**InstrumentName**](.md) | Instrument name | [required] |

### Return type

[**models::InstrumentPositionResponse**](InstrumentPositionResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_open_positions

> models::OpenPositionsResponse get_open_positions(account_id)
Open Positions

List all open Positions for an Account. An open Position is a Position in an Account that currently has a Trade opened for it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |

### Return type

[**models::OpenPositionsResponse**](OpenPositionsResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_positions

> models::PositionsResponse get_positions(account_id)
List Positions

List all Positions for an Account. The Positions returned are for every instrument that has had a position during the lifetime of an the Account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Account identifier | [required] |

### Return type

[**models::PositionsResponse**](PositionsResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

