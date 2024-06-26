# \Erc20Api

All URIs are relative to *https://beta.usemoon.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**allowance_erc20**](Erc20Api.md#allowance_erc20) | **POST** /erc20/{name}/allowance | 
[**approve_erc20**](Erc20Api.md#approve_erc20) | **POST** /erc20/{name}/approve | 
[**balance_of_erc20**](Erc20Api.md#balance_of_erc20) | **POST** /erc20/{name}/balance-of | 
[**decimals_erc20**](Erc20Api.md#decimals_erc20) | **POST** /erc20/{name}/decimals | 
[**name_erc20**](Erc20Api.md#name_erc20) | **POST** /erc20/{name}/name | 
[**symbol_erc20**](Erc20Api.md#symbol_erc20) | **POST** /erc20/{name}/symbol | 
[**total_supply_erc20**](Erc20Api.md#total_supply_erc20) | **POST** /erc20/{name}/total-supply | 
[**transfer_erc20**](Erc20Api.md#transfer_erc20) | **POST** /erc20/{name}/transfer | 
[**transfer_from_erc20**](Erc20Api.md#transfer_from_erc20) | **POST** /erc20/{name}/transfer-from | 



## allowance_erc20

> crate::models::TransactionApiResponse allowance_erc20(authorization, name, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## approve_erc20

> crate::models::TransactionApiResponse approve_erc20(authorization, name, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## balance_of_erc20

> crate::models::TransactionApiResponse balance_of_erc20(authorization, name, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## decimals_erc20

> crate::models::TransactionApiResponse decimals_erc20(authorization, name, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## name_erc20

> crate::models::TransactionApiResponse name_erc20(authorization, name, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## symbol_erc20

> crate::models::TransactionApiResponse symbol_erc20(authorization, name, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## total_supply_erc20

> crate::models::TransactionApiResponse total_supply_erc20(authorization, name, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_erc20

> crate::models::TransactionApiResponse transfer_erc20(authorization, name, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## transfer_from_erc20

> crate::models::TransactionApiResponse transfer_from_erc20(authorization, name, input_body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** |  | [required] |
**name** | **String** |  | [required] |
**input_body** | [**InputBody**](InputBody.md) |  | [required] |

### Return type

[**crate::models::TransactionApiResponse**](TransactionAPIResponse.md)

### Authorization

[ApiKeyAuth](../README.md#ApiKeyAuth), [BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

