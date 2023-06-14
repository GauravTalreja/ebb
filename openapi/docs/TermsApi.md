# \TermsApi

All URIs are relative to *https://openapi.data.uwaterloo.ca*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v3_terms_code_get**](TermsApi.md#v3_terms_code_get) | **GET** /v3/Terms/{code} | Gets Term data for a specific Term filtered by code
[**v3_terms_current_get**](TermsApi.md#v3_terms_current_get) | **GET** /v3/Terms/current | Gets the current Term data
[**v3_terms_foracademicyear_year_get**](TermsApi.md#v3_terms_foracademicyear_year_get) | **GET** /v3/Terms/foracademicyear/{year} | Gets Term data for terms that are part of a specific academic year
[**v3_terms_get**](TermsApi.md#v3_terms_get) | **GET** /v3/Terms | Gets all Term data that is effective at the current time



## v3_terms_code_get

> crate::models::Term v3_terms_code_get(code)
Gets Term data for a specific Term filtered by code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Term 4 digit Code | [required] |

### Return type

[**crate::models::Term**](Term.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_terms_current_get

> crate::models::Term v3_terms_current_get()
Gets the current Term data

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::Term**](Term.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_terms_foracademicyear_year_get

> Vec<crate::models::Term> v3_terms_foracademicyear_year_get(year)
Gets Term data for terms that are part of a specific academic year

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** | Academic year associated to Terms | [required] |

### Return type

[**Vec<crate::models::Term>**](Term.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_terms_get

> Vec<crate::models::Term> v3_terms_get()
Gets all Term data that is effective at the current time

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Term>**](Term.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
