# \AccountApi

All URIs are relative to *https://openapi.data.uwaterloo.ca/v3*

Method | HTTP request | Description
------------- | ------------- | -------------
[**v3_account_confirm_post**](AccountApi.md#v3_account_confirm_post) | **POST** /v3/Account/Confirm | Use this method to confirm your email and activate your account
[**v3_account_email_api_key_reset_get**](AccountApi.md#v3_account_email_api_key_reset_get) | **GET** /v3/Account/{email}/{apiKey}/reset | User this method to put your account in pending confirmation status and generate a new API key. Your old key will no longer grant access. The account will need to be confirmed again before the new key grants access.
[**v3_account_email_get**](AccountApi.md#v3_account_email_get) | **GET** /v3/Account/{email} | Use this method to see if an email has already been registered for an API key
[**v3_account_email_notify_get**](AccountApi.md#v3_account_email_notify_get) | **GET** /v3/Account/{email}/notify | Use this method to have us re-send the confirmation email to an account pending confirmation, if it exists. The activation code will be reset in the process.
[**v3_account_register_post**](AccountApi.md#v3_account_register_post) | **POST** /v3/Account/Register | Use this method to request an API key and begin your registration process



## v3_account_confirm_post

> v3_account_confirm_post(email, code)
Use this method to confirm your email and activate your account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | Option<**String**> | Email address you used to register |  |
**code** | Option<**uuid::Uuid**> | Activation code we sent you in the confirmation email |  |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_account_email_api_key_reset_get

> v3_account_email_api_key_reset_get(email, api_key)
User this method to put your account in pending confirmation status and generate a new API key. Your old key will no longer grant access. The account will need to be confirmed again before the new key grants access.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email address of associated account to reset | [required] |
**api_key** | **String** | Current API account key | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_account_email_get

> v3_account_email_get(email)
Use this method to see if an email has already been registered for an API key

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email address to check | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_account_email_notify_get

> v3_account_email_notify_get(email)
Use this method to have us re-send the confirmation email to an account pending confirmation, if it exists. The activation code will be reset in the process.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | **String** | Email address to re-send confirmation email to | [required] |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## v3_account_register_post

> v3_account_register_post(email, project, uri)
Use this method to request an API key and begin your registration process

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**email** | Option<**String**> | A unique email that we can use to identify you and contact you. We will ask to confirm it. |  |
**project** | Option<**String**> | A name and description of your project |  |
**uri** | Option<**String**> | The web address of your project: it's live location, app store entry, or similar |  |

### Return type

 (empty response body)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
