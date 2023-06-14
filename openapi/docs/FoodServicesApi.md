# \FoodServicesApi

All URIs are relative to *https://openapi.data.uwaterloo.ca*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v3_food_services_franchises_get**](FoodServicesApi.md#v3_food_services_franchises_get) | **GET** /v3/FoodServices/franchises | Get all food service Franchise data
[**v3_food_services_franchises_id_get**](FoodServicesApi.md#v3_food_services_franchises_id_get) | **GET** /v3/FoodServices/franchises/{id} | Get specific food sercices Franchise data by Id
[**v3_food_services_franchises_name_get**](FoodServicesApi.md#v3_food_services_franchises_name_get) | **GET** /v3/FoodServices/franchises/{name} | Get specific food services Franchise data by Franchise name
[**v3_food_services_outlets_get**](FoodServicesApi.md#v3_food_services_outlets_get) | **GET** /v3/FoodServices/outlets | Get all food service Outlet data
[**v3_food_services_outlets_id_get**](FoodServicesApi.md#v3_food_services_outlets_id_get) | **GET** /v3/FoodServices/outlets/{id} | Get specific food services Outlet data by Id
[**v3_food_services_outlets_name_get**](FoodServicesApi.md#v3_food_services_outlets_name_get) | **GET** /v3/FoodServices/outlets/{name} | Get specific food services Outlet data by Outlet name



## v3_food_services_franchises_get

> crate::models::FoodServicesFranchises v3_food_services_franchises_get()
Get all food service Franchise data

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::FoodServicesFranchises**](FoodServicesFranchises.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_food_services_franchises_id_get

> crate::models::FoodServicesFranchise v3_food_services_franchises_id_get(id)
Get specific food sercices Franchise data by Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | WCMS object ID representing the Franchise | [required] |

### Return type

[**crate::models::FoodServicesFranchise**](FoodServicesFranchise.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_food_services_franchises_name_get

> crate::models::FoodServicesFranchise v3_food_services_franchises_name_get(name)
Get specific food services Franchise data by Franchise name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name to filter Franchise by | [required] |

### Return type

[**crate::models::FoodServicesFranchise**](FoodServicesFranchise.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_food_services_outlets_get

> crate::models::FoodServicesOutlets v3_food_services_outlets_get()
Get all food service Outlet data

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::FoodServicesOutlets**](FoodServicesOutlets.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_food_services_outlets_id_get

> crate::models::FoodServicesOutlet v3_food_services_outlets_id_get(id)
Get specific food services Outlet data by Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | WCMS object ID representing the Outlet | [required] |

### Return type

[**crate::models::FoodServicesOutlet**](FoodServicesOutlet.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_food_services_outlets_name_get

> crate::models::FoodServicesOutlet v3_food_services_outlets_name_get(name)
Get specific food services Outlet data by Outlet name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | **String** | Name to filter Outlet by | [required] |

### Return type

[**crate::models::FoodServicesOutlet**](FoodServicesOutlet.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
