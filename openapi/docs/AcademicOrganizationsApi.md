# \AcademicOrganizationsApi

All URIs are relative to *https://openapi.data.uwaterloo.ca*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v3_academic_organizations_get**](AcademicOrganizationsApi.md#v3_academic_organizations_get) | **GET** /v3/AcademicOrganizations | Gets all Academic Organization data
[**v3_academic_organizations_organization_code_get**](AcademicOrganizationsApi.md#v3_academic_organizations_organization_code_get) | **GET** /v3/AcademicOrganizations/{organizationCode} | Gets Academic Organization data for a specific entry by the Organization code



## v3_academic_organizations_get

> Vec<crate::models::AcademicOrganization> v3_academic_organizations_get()
Gets all Academic Organization data

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::AcademicOrganization>**](AcademicOrganization.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_academic_organizations_organization_code_get

> crate::models::AcademicOrganization v3_academic_organizations_organization_code_get(organization_code)
Gets Academic Organization data for a specific entry by the Organization code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_code** | **String** | Unique Academic Organization code | [required] |

### Return type

[**crate::models::AcademicOrganization**](AcademicOrganization.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
