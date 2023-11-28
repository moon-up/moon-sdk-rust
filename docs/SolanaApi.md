# \SolanaApi

All URIs are relative to *https://vault-api.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_solana_account**](SolanaApi.md#create_solana_account) | **POST** /solana | 
[**get_solana_account**](SolanaApi.md#get_solana_account) | **GET** /solana/{accountName} | 
[**list_solana_accounts**](SolanaApi.md#list_solana_accounts) | **GET** /solana | 
[**sign_solana_transaction**](SolanaApi.md#sign_solana_transaction) | **POST** /solana/{accountName}/sign-tx | 



## create_solana_account

> crate::models::AccountControllerResponse create_solana_account(authorization, solana_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**solana_input** | [**SolanaInput**](SolanaInput.md) |  | [required] |

### Return type

[**crate::models::AccountControllerResponse**](AccountControllerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_solana_account

> crate::models::AccountControllerResponse get_solana_account(authorization, account_name)


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


## list_solana_accounts

> crate::models::AccountControllerResponse list_solana_accounts(authorization)


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


## sign_solana_transaction

> crate::models::AccountControllerResponse sign_solana_transaction(authorization, account_name, solana_transaction_input)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |
**solana_transaction_input** | [**SolanaTransactionInput**](SolanaTransactionInput.md) |  | [required] |

### Return type

[**crate::models::AccountControllerResponse**](AccountControllerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

