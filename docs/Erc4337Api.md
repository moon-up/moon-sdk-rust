# \Erc4337Api

All URIs are relative to *https://vault-api.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_address**](Erc4337Api.md#get_address) | **POST** /erc4337/{accountName}/address | 
[**sign_broadcast_user_op_tx**](Erc4337Api.md#sign_broadcast_user_op_tx) | **POST** /erc4337/{accountName}/sign-broadcast-userop-tx | 



## get_address

> crate::models::AccountControllerResponse get_address(authorization, account_name, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::AccountControllerResponse**](AccountControllerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sign_broadcast_user_op_tx

> crate::models::AccountControllerResponse sign_broadcast_user_op_tx(authorization, account_name, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**account_name** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::AccountControllerResponse**](AccountControllerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

