# PricingResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**prices** | Option<[**Vec<models::ClientPrice>**](ClientPrice.md)> | The list of Price objects requested. | [optional]
**home_conversions** | Option<[**Vec<models::HomeConversions>**](HomeConversions.md)> | The list of home currency conversion factors requested. This field will only be present if includeHomeConversions was set to true in the request. | [optional]
**time** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


