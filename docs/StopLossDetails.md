# StopLossDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**price** | Option<**f64**> | The price that the Stop Loss Order will be triggered at. Only one of the price and distance fields may be specified. | [optional]
**distance** | Option<**f64**> | Specifies the distance (in price units) from the Trade's open price to use as the Stop Loss Order price. Only one of the distance and price fields may be specified. | [optional]
**time_in_force** | Option<[**models::TradeOrderTimeInForce**](TradeOrderTimeInForce.md)> |  | [optional]
**gtd_time** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**client_extensions** | Option<[**models::ClientExtensions**](ClientExtensions.md)> |  | [optional]
**guaranteed** | Option<**bool**> | Flag indicating that the price for the Stop Loss Order is guaranteed. The default value depends on the GuaranteedStopLossOrderMode of the account, if it is REQUIRED, the default will be true, for DISABLED or ENABLED the default is false. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


