# \LocationsApi

All URIs are relative to *https://openapi.data.uwaterloo.ca/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v3_locations_geojson_get**](LocationsApi.md#v3_locations_geojson_get) | **GET** /v3/Locations/geojson | Get all building location data as GEO JSON
[**v3_locations_get**](LocationsApi.md#v3_locations_get) | **GET** /v3/Locations | Get all building location data
[**v3_locations_location_code_geojson_get**](LocationsApi.md#v3_locations_location_code_geojson_get) | **GET** /v3/Locations/{locationCode}/geojson | Gets building by matched building code, case insensitive, as GEO JSON
[**v3_locations_location_code_get**](LocationsApi.md#v3_locations_location_code_get) | **GET** /v3/Locations/{locationCode} | Gets building by matched building code, case insensitive
[**v3_locations_search_location_name_geojson_get**](LocationsApi.md#v3_locations_search_location_name_geojson_get) | **GET** /v3/Locations/search/{locationName}/geojson | Gets buildings by matched building name, contains, case insensitive, as GEO JSON
[**v3_locations_search_location_name_get**](LocationsApi.md#v3_locations_search_location_name_get) | **GET** /v3/Locations/search/{locationName} | Gets buildings by matched building name, contains, case insensitive



## v3_locations_geojson_get

> crate::models::LocationGeo v3_locations_geojson_get()
Get all building location data as GEO JSON

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::LocationGeo**](LocationGeo.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_locations_get

> Vec<crate::models::Location> v3_locations_get()
Get all building location data

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Location>**](Location.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_locations_location_code_geojson_get

> crate::models::LocationGeo v3_locations_location_code_geojson_get(location_code)
Gets building by matched building code, case insensitive, as GEO JSON

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_code** | **String** | Building code to match | [required] |

### Return type

[**crate::models::LocationGeo**](LocationGeo.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_locations_location_code_get

> crate::models::Location v3_locations_location_code_get(location_code)
Gets building by matched building code, case insensitive

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_code** | **String** | Building code to match | [required] |

### Return type

[**crate::models::Location**](Location.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_locations_search_location_name_geojson_get

> crate::models::LocationGeo v3_locations_search_location_name_geojson_get(location_name)
Gets buildings by matched building name, contains, case insensitive, as GEO JSON

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_name** | **String** | Text to match in building name | [required] |

### Return type

[**crate::models::LocationGeo**](LocationGeo.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_locations_search_location_name_get

> Vec<crate::models::Location> v3_locations_search_location_name_get(location_name)
Gets buildings by matched building name, contains, case insensitive

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**location_name** | **String** | Text to match in building name | [required] |

### Return type

[**Vec<crate::models::Location>**](Location.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
