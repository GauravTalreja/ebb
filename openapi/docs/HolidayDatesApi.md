# \HolidayDatesApi

All URIs are relative to *https://openapi.data.uwaterloo.ca*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v3_holiday_dates_paidholidays_get**](HolidayDatesApi.md#v3_holiday_dates_paidholidays_get) | **GET** /v3/HolidayDates/paidholidays | Retrieves data for all paid holidays as published by Human Resources
[**v3_holiday_dates_paidholidays_ics_get**](HolidayDatesApi.md#v3_holiday_dates_paidholidays_ics_get) | **GET** /v3/HolidayDates/paidholidays/ics | Retrieves data for all paid holidays as published by Human Resources, as an ICS format feed. Allows anonymous access.
[**v3_holiday_dates_paidholidays_year_get**](HolidayDatesApi.md#v3_holiday_dates_paidholidays_year_get) | **GET** /v3/HolidayDates/paidholidays/{year} | Retrieves data for paid holidays associated to the given year as published by Human Resources



## v3_holiday_dates_paidholidays_get

> Vec<crate::models::PaidHoliday> v3_holiday_dates_paidholidays_get()
Retrieves data for all paid holidays as published by Human Resources

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::PaidHoliday>**](PaidHoliday.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_holiday_dates_paidholidays_ics_get

> v3_holiday_dates_paidholidays_ics_get()
Retrieves data for all paid holidays as published by Human Resources, as an ICS format feed. Allows anonymous access.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_holiday_dates_paidholidays_year_get

> Vec<crate::models::PaidHoliday> v3_holiday_dates_paidholidays_year_get(year)
Retrieves data for paid holidays associated to the given year as published by Human Resources

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**year** | **i32** |  | [required] |

### Return type

[**Vec<crate::models::PaidHoliday>**](PaidHoliday.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
