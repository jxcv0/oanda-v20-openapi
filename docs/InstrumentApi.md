# \InstrumentApi

All URIs are relative to *https://api-fxpractice.oanda.com/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_instrument_candles**](InstrumentApi.md#get_instrument_candles) | **GET** /instruments/{instrument}/candles | Get Candlesticks
[**get_instrument_order_book**](InstrumentApi.md#get_instrument_order_book) | **GET** /instruments/{instrument}/orderBook | Get Order Book
[**get_instrument_position_book**](InstrumentApi.md#get_instrument_position_book) | **GET** /instruments/{instrument}/positionBook | Get Position Book



## get_instrument_candles

> models::CandlesResponse get_instrument_candles(instrument, accept_datetime_format, price, granularity, count, from, to, smooth, include_first, daily_alignment, alignment_timezone, weekly_alignment)
Get Candlesticks

Fetch candlestick data for an instrument.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instrument** | [**InstrumentName**](.md) | Instrument name | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |
**price** | Option<**String**> | The Price component(s) to get candlestick data for. Can contain any combination of the characters \"M\" (midpoint candles) \"B\" (bid candles) and \"A\" (ask candles). |  |
**granularity** | Option<[**CandlestickGranularity**](.md)> | The granularity of the candlesticks to fetch |  |
**count** | Option<**i32**> | The number of candlesticks to return in the reponse. Count should not be specified if both the start and end parameters are provided, as the time range combined with the graularity will determine the number of candlesticks to return. |  |
**from** | Option<**String**> | The start of the time range to fetch candlesticks for. |  |
**to** | Option<**String**> | The end of the time range to fetch candlesticks for. |  |
**smooth** | Option<**bool**> | A flag that controls whether the candlestick is \"smoothed\" or not.  A smoothed candlestick uses the previous candle's close price as its open price, while an unsmoothed candlestick uses the first price from its time range as its open price. |  |
**include_first** | Option<**bool**> | A flag that controls whether the candlestick that is covered by the from time should be included in the results. This flag enables clients to use the timestamp of the last completed candlestick received to poll for future candlesticks but avoid receiving the previous candlestick repeatedly. |  |
**daily_alignment** | Option<**i32**> | The hour of the day (in the specified timezone) to use for granularities that have daily alignments. |  |
**alignment_timezone** | Option<**String**> | The timezone to use for the dailyAlignment parameter. Candlesticks with daily alignment will be aligned to the dailyAlignment hour within the alignmentTimezone.  Note that the returned times will still be represented in UTC. |  |
**weekly_alignment** | Option<**String**> | The day of the week used for granularities that have weekly alignment. |  |

### Return type

[**models::CandlesResponse**](CandlesResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instrument_order_book

> models::InstrumentOrderBookResponse get_instrument_order_book(instrument, accept_datetime_format, time)
Get Order Book

Fetch an order book for an instrument.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instrument** | [**InstrumentName**](.md) | Instrument name | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |
**time** | Option<**String**> | The time of the snapshot to fetch. If not specified, then the most recent snapshot is fetched. |  |

### Return type

[**models::InstrumentOrderBookResponse**](InstrumentOrderBookResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_instrument_position_book

> models::InstrumentPositionBookResponse get_instrument_position_book(instrument, accept_datetime_format, time)
Get Position Book

Fetch a position book for an instrument.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instrument** | [**InstrumentName**](.md) | Instrument name | [required] |
**accept_datetime_format** | Option<[**DateTimeFormat**](.md)> | Format of DateTime fields in the request and response. |  |
**time** | Option<**String**> | The time of the snapshot to fetch. If not specified, then the most recent snapshot is fetched. |  |

### Return type

[**models::InstrumentPositionBookResponse**](InstrumentPositionBookResponse.md)

### Authorization

[PersonalAccessToken](../README.md#PersonalAccessToken)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

