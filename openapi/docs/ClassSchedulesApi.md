# \ClassSchedulesApi

All URIs are relative to *https://openapi.data.uwaterloo.ca/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v3_class_schedules_term_code_course_id_get**](ClassSchedulesApi.md#v3_class_schedules_term_code_course_id_get) | **GET** /v3/ClassSchedules/{termCode}/{courseId} | Get Class data for a scheduled Course by Course ID, in a specific Term
[**v3_class_schedules_term_code_get**](ClassSchedulesApi.md#v3_class_schedules_term_code_get) | **GET** /v3/ClassSchedules/{termCode} | Get the Course IDs that have one or more active and associated schedules in the given Term
[**v3_class_schedules_term_code_subject_catalog_number_get**](ClassSchedulesApi.md#v3_class_schedules_term_code_subject_catalog_number_get) | **GET** /v3/ClassSchedules/{termCode}/{subject}/{catalogNumber} | Get Class data for a scheduled Course by Subject and catalog number, in a specific Term



## v3_class_schedules_term_code_course_id_get

> Vec<crate::models::Class> v3_class_schedules_term_code_course_id_get(term_code, course_id)
Get Class data for a scheduled Course by Course ID, in a specific Term

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term_code** | **String** | Waterloo Term code to filter on | [required] |
**course_id** | **String** | Course ID to filter on | [required] |

### Return type

[**Vec<crate::models::Class>**](Class.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_class_schedules_term_code_get

> Vec<String> v3_class_schedules_term_code_get(term_code)
Get the Course IDs that have one or more active and associated schedules in the given Term

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term_code** | **String** | Waterloo Term code to filter on | [required] |

### Return type

**Vec<String>**

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_class_schedules_term_code_subject_catalog_number_get

> Vec<crate::models::Class> v3_class_schedules_term_code_subject_catalog_number_get(term_code, subject, catalog_number)
Get Class data for a scheduled Course by Subject and catalog number, in a specific Term

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term_code** | **String** | Waterloo Term code to filter on | [required] |
**subject** | **String** | Academic Subject code to filter ron | [required] |
**catalog_number** | **String** | Catalog number to filter on | [required] |

### Return type

[**Vec<crate::models::Class>**](Class.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
