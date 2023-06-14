# \ExamSchedulesApi

All URIs are relative to *https://openapi.data.uwaterloo.ca/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v3_exam_schedules_code_get**](ExamSchedulesApi.md#v3_exam_schedules_code_get) | **GET** /v3/ExamSchedules/{code} | Returns Exam Schedule data for the requested Term
[**v3_exam_schedules_get**](ExamSchedulesApi.md#v3_exam_schedules_get) | **GET** /v3/ExamSchedules | Returns Exam Schedule data for the current Term



## v3_exam_schedules_code_get

> Vec<crate::models::Exam> v3_exam_schedules_code_get(code)
Returns Exam Schedule data for the requested Term

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code** | **String** | Waterloo Term code | [required] |

### Return type

[**Vec<crate::models::Exam>**](Exam.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_exam_schedules_get

> Vec<crate::models::Exam> v3_exam_schedules_get()
Returns Exam Schedule data for the current Term

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Exam>**](Exam.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
