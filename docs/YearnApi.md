# \YearnApi

All URIs are relative to *https://vault-api.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_liquidity**](YearnApi.md#add_liquidity) | **POST** /yearn/{name}/add-liquidity | 
[**add_liquidity_weth**](YearnApi.md#add_liquidity_weth) | **POST** /yearn/{name}/add-liquidity-weth | 
[**remove_liquidity**](YearnApi.md#remove_liquidity) | **POST** /yearn/{name}/remove-liquidity | 
[**remove_liquidity_weth**](YearnApi.md#remove_liquidity_weth) | **POST** /yearn/{name}/remove-liquidity-weth | 



## add_liquidity

> crate::models::AccountControllerResponse add_liquidity(authorization, name, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::AccountControllerResponse**](AccountControllerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_liquidity_weth

> crate::models::AccountControllerResponse add_liquidity_weth(authorization, name, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::AccountControllerResponse**](AccountControllerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_liquidity

> crate::models::AccountControllerResponse remove_liquidity(authorization, name, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::AccountControllerResponse**](AccountControllerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_liquidity_weth

> crate::models::AccountControllerResponse remove_liquidity_weth(authorization, name, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::AccountControllerResponse**](AccountControllerResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

