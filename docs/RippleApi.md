# \RippleApi

All URIs are relative to *https://vault-api.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ripple_account**](RippleApi.md#create_ripple_account) | **POST** /ripple | 
[**get_ripple_account**](RippleApi.md#get_ripple_account) | **GET** /ripple/{accountName} | 
[**list_ripple_accounts**](RippleApi.md#list_ripple_accounts) | **GET** /ripple | 
[**sign_ripple_transaction**](RippleApi.md#sign_ripple_transaction) | **POST** /ripple/{accountName}/sign-tx | 



## create_ripple_account

> crate::models::AccountControllerResponse create_ripple_account(authorization, ripple_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**ripple_input** | [**RippleInput**](RippleInput.md) |  | [required] |

### Return type

[**crate::models::AccountControllerResponse**](AccountControllerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ripple_account

> crate::models::AccountControllerResponse get_ripple_account(authorization, account_name)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |

### Return type

[**crate::models::AccountControllerResponse**](AccountControllerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_ripple_accounts

> crate::models::AccountControllerResponse list_ripple_accounts(authorization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |

### Return type

[**crate::models::AccountControllerResponse**](AccountControllerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sign_ripple_transaction

> crate::models::AccountControllerResponse sign_ripple_transaction(authorization, account_name, ripple_transaction_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |
**ripple_transaction_input** | [**RippleTransactionInput**](RippleTransactionInput.md) |  | [required] |

### Return type

[**crate::models::AccountControllerResponse**](AccountControllerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

