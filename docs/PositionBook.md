# PositionBook

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**instrument** | Option<[**models::InstrumentName**](InstrumentName.md)> |  | [optional]
**time** | Option<**String**> | A date and time value using either RFC3339 or UNIX time representation. The RFC 3339 representation is a string conforming to https://tools.ietf.org/rfc/rfc3339.txt. The Unix representation is a string representing the number of seconds since the Unix Epoch (January 1st, 1970 at UTC). The value is a fractional number, where the fractional part represents a fraction of a second (up to nine decimal places). | [optional]
**price** | Option<**f64**> | The price (midpoint) for the position book's instrument at the time of the position book snapshot | [optional]
**bucket_width** | Option<**f64**> | The price width for each bucket. Each bucket covers the price range from the bucket's price to the bucket's price + bucketWidth. | [optional]
**buckets** | Option<[**Vec<models::PositionBookBucket>**](PositionBookBucket.md)> | The partitioned position book, divided into buckets using a default bucket width. These buckets are only provided for price ranges which actually contain order or position data. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


