# \WcmsApi

All URIs are relative to *https://openapi.data.uwaterloo.ca*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v3_wcms_get**](WcmsApi.md#v3_wcms_get) | **GET** /v3/Wcms | Retrieves information about all active and published WCMS sites
[**v3_wcms_id_events_get**](WcmsApi.md#v3_wcms_id_events_get) | **GET** /v3/Wcms/{id}/events | Retrieves all event items for a specific WCMS site by Site Id
[**v3_wcms_id_get**](WcmsApi.md#v3_wcms_id_get) | **GET** /v3/Wcms/{id} | Retrieves information about a specific WCMS site by Site Id
[**v3_wcms_id_news_get**](WcmsApi.md#v3_wcms_id_news_get) | **GET** /v3/Wcms/{id}/news | Retrieves all news items for a specific WCMS site by Site Id
[**v3_wcms_id_opportunities_get**](WcmsApi.md#v3_wcms_id_opportunities_get) | **GET** /v3/Wcms/{id}/opportunities | Retrieves all opportunity items for a specific WCMS site by Site Id
[**v3_wcms_id_posts_get**](WcmsApi.md#v3_wcms_id_posts_get) | **GET** /v3/Wcms/{id}/posts | Retrieves all blog post items for a specific WCMS site by Site Id
[**v3_wcms_latestevents_max_items_get**](WcmsApi.md#v3_wcms_latestevents_max_items_get) | **GET** /v3/Wcms/latestevents/{maxItems} | Retrieves the latest events items across all WCMS sites, ordered by event start date
[**v3_wcms_latestnews_max_items_get**](WcmsApi.md#v3_wcms_latestnews_max_items_get) | **GET** /v3/Wcms/latestnews/{maxItems} | Retrieves the latest news items across all WCMS sites, ordered by posted date
[**v3_wcms_latestopportunities_max_items_get**](WcmsApi.md#v3_wcms_latestopportunities_max_items_get) | **GET** /v3/Wcms/latestopportunities/{maxItems} | Retrieves the latest opportunity items across all WCMS sites, ordered by posted/open date
[**v3_wcms_latestposts_max_items_get**](WcmsApi.md#v3_wcms_latestposts_max_items_get) | **GET** /v3/Wcms/latestposts/{maxItems} | Retrieves the latest blog post items across all WCMS sites, ordered by posted date



## v3_wcms_get

> Vec<crate::models::Site> v3_wcms_get()
Retrieves information about all active and published WCMS sites

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Site>**](Site.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_wcms_id_events_get

> Vec<crate::models::SiteEvent> v3_wcms_id_events_get(id, max_items, newest_first)
Retrieves all event items for a specific WCMS site by Site Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Unique Id for the Site | [required] |
**max_items** | Option<**i32**> | (Optional) Maximum amount of items to retrieve |  |
**newest_first** | Option<**bool**> | (Optional) Requires maxItems parameter, sorts items by created date, newest first |  |

### Return type

[**Vec<crate::models::SiteEvent>**](SiteEvent.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_wcms_id_get

> crate::models::Site v3_wcms_id_get(id)
Retrieves information about a specific WCMS site by Site Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Unique site Id | [required] |

### Return type

[**crate::models::Site**](Site.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_wcms_id_news_get

> Vec<crate::models::SiteNews> v3_wcms_id_news_get(id, max_items, newest_first)
Retrieves all news items for a specific WCMS site by Site Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Unique Id for the Site | [required] |
**max_items** | Option<**i32**> | (Optional) Maximum amount of items to retrieve |  |
**newest_first** | Option<**bool**> | (Optional) Requires maxItems parameter, sorts items by created date, newest first |  |

### Return type

[**Vec<crate::models::SiteNews>**](SiteNews.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_wcms_id_opportunities_get

> Vec<crate::models::SiteOpportunity> v3_wcms_id_opportunities_get(id, max_items, newest_first)
Retrieves all opportunity items for a specific WCMS site by Site Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Unique Id for the Site | [required] |
**max_items** | Option<**i32**> | (Optional) Maximum amount of items to retrieve |  |
**newest_first** | Option<**bool**> | (Optional) Requires maxItems parameter, sorts items by created date, newest first |  |

### Return type

[**Vec<crate::models::SiteOpportunity>**](SiteOpportunity.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_wcms_id_posts_get

> Vec<crate::models::SiteBlogPost> v3_wcms_id_posts_get(id, max_items, newest_first)
Retrieves all blog post items for a specific WCMS site by Site Id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Unique Id for the Site | [required] |
**max_items** | Option<**i32**> | (Optional) Maximum amount of items to retrieve |  |
**newest_first** | Option<**bool**> | (Optional) Requires maxItems parameter, sorts items by created date, newest first |  |

### Return type

[**Vec<crate::models::SiteBlogPost>**](SiteBlogPost.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_wcms_latestevents_max_items_get

> Vec<crate::models::SiteEvent> v3_wcms_latestevents_max_items_get(max_items)
Retrieves the latest events items across all WCMS sites, ordered by event start date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_items** | **i32** | Number of items to retrieve, default 15, maximum 25 | [required] |[default to 15]

### Return type

[**Vec<crate::models::SiteEvent>**](SiteEvent.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_wcms_latestnews_max_items_get

> Vec<crate::models::SiteNews> v3_wcms_latestnews_max_items_get(max_items)
Retrieves the latest news items across all WCMS sites, ordered by posted date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_items** | **i32** | Number of items to retrieve, default 15, maximum 25 | [required] |[default to 15]

### Return type

[**Vec<crate::models::SiteNews>**](SiteNews.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_wcms_latestopportunities_max_items_get

> Vec<crate::models::SiteOpportunity> v3_wcms_latestopportunities_max_items_get(max_items)
Retrieves the latest opportunity items across all WCMS sites, ordered by posted/open date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_items** | **i32** | Number of items to retrieve, default 15, maximum 25 | [required] |[default to 15]

### Return type

[**Vec<crate::models::SiteOpportunity>**](SiteOpportunity.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_wcms_latestposts_max_items_get

> Vec<crate::models::SiteBlogPost> v3_wcms_latestposts_max_items_get(max_items)
Retrieves the latest blog post items across all WCMS sites, ordered by posted date

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**max_items** | **i32** | Number of items to retrieve, default 15, maximum 25 | [required] |[default to 15]

### Return type

[**Vec<crate::models::SiteBlogPost>**](SiteBlogPost.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
