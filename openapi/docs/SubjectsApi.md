# \SubjectsApi

All URIs are relative to *https://openapi.data.uwaterloo.ca*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v3_subjects_associatedto_organization_code_get**](SubjectsApi.md#v3_subjects_associatedto_organization_code_get) | **GET** /v3/Subjects/associatedto/{organizationCode} | Gets Subject data for Subjects associated to an Academic Organization by Organization code
[**v3_subjects_code_get**](SubjectsApi.md#v3_subjects_code_get) | **GET** /v3/Subjects/{code} | Gets Subject data filtered by Subject code
[**v3_subjects_get**](SubjectsApi.md#v3_subjects_get) | **GET** /v3/Subjects | Gets all Subject data



## v3_subjects_associatedto_organization_code_get

> Vec<crate::models::Subject> v3_subjects_associatedto_organization_code_get(organization_code)
Gets Subject data for Subjects associated to an Academic Organization by Organization code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_code** | **String** | Academic Organization Code that associates to the Subjects | [required] |

### Return type

[**Vec<crate::models::Subject>**](Subject.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_subjects_code_get

> crate::models::Subject v3_subjects_code_get(code)
Gets Subject data filtered by Subject code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Specific Subject code | [required] |

### Return type

[**crate::models::Subject**](Subject.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_subjects_get

> Vec<crate::models::Subject> v3_subjects_get()
Gets all Subject data

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Subject>**](Subject.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
