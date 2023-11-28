# \PaymentApi

All URIs are relative to *https://vault-api.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**moralis_webhook**](PaymentApi.md#moralis_webhook) | **POST** /payment/webhook/{id} | 
[**payment_create_payment_intent**](PaymentApi.md#payment_create_payment_intent) | **POST** /payment | 
[**payment_delete_payment_intent**](PaymentApi.md#payment_delete_payment_intent) | **DELETE** /payment/{id} | 
[**payment_get_all_payment_intents**](PaymentApi.md#payment_get_all_payment_intents) | **GET** /payment | 
[**payment_get_available_chains**](PaymentApi.md#payment_get_available_chains) | **GET** /payment/chains | 
[**payment_get_payment_intent**](PaymentApi.md#payment_get_payment_intent) | **GET** /payment/{id} | 
[**payment_update_payment_intent**](PaymentApi.md#payment_update_payment_intent) | **PUT** /payment/{id} | 
[**tatum_webhook**](PaymentApi.md#tatum_webhook) | **POST** /payment/tatum/webhook/{id} | 



## moralis_webhook

> serde_json::Value moralis_webhook(id, i_webhook)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**i_webhook** | [**IWebhook**](IWebhook.md) |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_create_payment_intent

> crate::models::PaymentIntentResponse payment_create_payment_intent(authorization, create_payment_intent_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**create_payment_intent_input** | [**CreatePaymentIntentInput**](CreatePaymentIntentInput.md) |  | [required] |

### Return type

[**crate::models::PaymentIntentResponse**](PaymentIntentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_delete_payment_intent

> crate::models::PaymentIntentResponse payment_delete_payment_intent(authorization, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**crate::models::PaymentIntentResponse**](PaymentIntentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_get_all_payment_intents

> Vec<crate::models::PaymentIntentResponse> payment_get_all_payment_intents(authorization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |

### Return type

[**Vec<crate::models::PaymentIntentResponse>**](PaymentIntentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_get_available_chains

> Vec<String> payment_get_available_chains()


### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_get_payment_intent

> crate::models::PaymentIntentResponse payment_get_payment_intent(authorization, id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**id** | **String** |  | [required] |

### Return type

[**crate::models::PaymentIntentResponse**](PaymentIntentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## payment_update_payment_intent

> crate::models::PaymentIntentResponse payment_update_payment_intent(authorization, id, create_payment_intent_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**id** | **String** |  | [required] |
**create_payment_intent_input** | [**CreatePaymentIntentInput**](CreatePaymentIntentInput.md) |  | [required] |

### Return type

[**crate::models::PaymentIntentResponse**](PaymentIntentResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tatum_webhook

> serde_json::Value tatum_webhook(id, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |
**body** | Option<**serde_json::Value**> |  | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

