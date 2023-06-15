# \CoursesApi

All URIs are relative to *https://openapi.data.uwaterloo.ca*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v3_courses_term_code_course_id_get**](CoursesApi.md#v3_courses_term_code_course_id_get) | **GET** /v3/Courses/{termCode}/{courseId} | Get Course catalog data filtered by Term and Course ID, multiple Courses are usually cross listed
[**v3_courses_term_code_course_id_offer_number_get**](CoursesApi.md#v3_courses_term_code_course_id_offer_number_get) | **GET** /v3/Courses/{termCode}/{courseId}/{offerNumber} | Get Course catalog data filtered by Term, Course ID, and offer number
[**v3_courses_term_code_get**](CoursesApi.md#v3_courses_term_code_get) | **GET** /v3/Courses/{termCode} | Get all Course data for a Term
[**v3_courses_term_code_subject_catalog_number_get**](CoursesApi.md#v3_courses_term_code_subject_catalog_number_get) | **GET** /v3/Courses/{termCode}/{subject}/{catalogNumber} | Get Course catalog data filtered by Term, Subject, and catalog number
[**v3_courses_term_code_subject_get**](CoursesApi.md#v3_courses_term_code_subject_get) | **GET** /v3/Courses/{termCode}/{subject} | Get Course catalog data filtered by Term and Subject code



## v3_courses_term_code_course_id_get

> Vec<crate::models::Course> v3_courses_term_code_course_id_get(term_code, course_id)
Get Course catalog data filtered by Term and Course ID, multiple Courses are usually cross listed

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term_code** | **String** | Term code to filter on | [required] |
**course_id** | **String** | Course ID to filter on | [required] |

### Return type

[**Vec<crate::models::Course>**](Course.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_courses_term_code_course_id_offer_number_get

> crate::models::Course v3_courses_term_code_course_id_offer_number_get(term_code, course_id, offer_number)
Get Course catalog data filtered by Term, Course ID, and offer number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term_code** | **String** | Term code to filter on | [required] |
**course_id** | **String** | Course ID to filter on | [required] |
**offer_number** | **i32** | Offer number to filter on | [required] |

### Return type

[**crate::models::Course**](Course.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_courses_term_code_get

> Vec<crate::models::Course> v3_courses_term_code_get(term_code)
Get all Course data for a Term

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term_code** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Course>**](Course.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_courses_term_code_subject_catalog_number_get

> crate::models::Course v3_courses_term_code_subject_catalog_number_get(term_code, subject, catalog_number)
Get Course catalog data filtered by Term, Subject, and catalog number

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term_code** | **String** | Term code to filter on | [required] |
**subject** | **String** | Academic Subject code to filter on | [required] |
**catalog_number** | **String** | Course catalog number to filter on, ie: 101, 111W, 239 | [required] |

### Return type

[**crate::models::Course**](Course.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_courses_term_code_subject_get

> Vec<crate::models::Course> v3_courses_term_code_subject_get(term_code, subject)
Get Course catalog data filtered by Term and Subject code

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**term_code** | **String** | Term code to filter on | [required] |
**subject** | **String** | Academic Subject code to filter on | [required] |

### Return type

[**Vec<crate::models::Course>**](Course.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
