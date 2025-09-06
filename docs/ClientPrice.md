# ClientPrice

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | Option<**String**> | The string \"PRICE\". Used to identify the a Price object when found in a stream. | [optional]
**instrument** | Option<[**models::InstrumentName**](InstrumentName.md)> |  | [optional]
**time** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**status** | Option<[**models::PriceStatus**](PriceStatus.md)> |  | [optional]
**tradeable** | Option<**bool**> | Flag indicating if the Price is tradeable or not | [optional]
**bids** | Option<[**Vec<models::PriceBucket>**](PriceBucket.md)> | The list of prices and liquidity available on the Instrument's bid side. It is possible for this list to be empty if there is no bid liquidity currently available for the Instrument in the Account. | [optional]
**asks** | Option<[**Vec<models::PriceBucket>**](PriceBucket.md)> | The list of prices and liquidity available on the Instrument's ask side. It is possible for this list to be empty if there is no ask liquidity currently available for the Instrument in the Account. | [optional]
**closeout_bid** | Option<**f64**> | The closeout bid Price. This Price is used when a bid is required to closeout a Position (margin closeout or manual) yet there is no bid liquidity. The closeout bid is never used to open a new position. | [optional]
**closeout_ask** | Option<**f64**> | The closeout ask Price. This Price is used when a ask is required to closeout a Position (margin closeout or manual) yet there is no ask liquidity. The closeout ask is never used to open a new position. | [optional]
**quote_home_conversion_factors** | Option<[**models::QuoteHomeConversionFactors**](QuoteHomeConversionFactors.md)> |  | [optional]
**units_available** | Option<[**models::UnitsAvailable**](UnitsAvailable.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


