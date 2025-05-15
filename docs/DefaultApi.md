# \DefaultApi

All URIs are relative to *https://euvdservices.enisa.europa.eu*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_advisory_by_id**](DefaultApi.md#get_advisory_by_id) | **GET** /api/advisory | Show advisory by ID
[**get_by_enisa_id**](DefaultApi.md#get_by_enisa_id) | **GET** /api/enisaid | Show EUVD by ID
[**get_critical_vulnerabilities**](DefaultApi.md#get_critical_vulnerabilities) | **GET** /api/criticalvulnerabilities | Show latest critical vulnerabilities
[**get_exploited_vulnerabilities**](DefaultApi.md#get_exploited_vulnerabilities) | **GET** /api/exploitedvulnerabilities | Show latest exploited vulnerabilities
[**get_last_vulnerabilities**](DefaultApi.md#get_last_vulnerabilities) | **GET** /api/lastvulnerabilities | Show latest vulnerabilities
[**get_vulnerability_by_id**](DefaultApi.md#get_vulnerability_by_id) | **GET** /api/vulnerability | Show vulnerability by ID
[**query_vulnerabilities**](DefaultApi.md#query_vulnerabilities) | **GET** /api/vulnerabilities | Query vulnerabilities with flexible filters



## get_advisory_by_id

> get_advisory_by_id(id)
Show advisory by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_by_enisa_id

> get_by_enisa_id(id)
Show EUVD by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_critical_vulnerabilities

> Vec<models::Vulnerability> get_critical_vulnerabilities()
Show latest critical vulnerabilities

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Vulnerability>**](Vulnerability.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_exploited_vulnerabilities

> Vec<models::Vulnerability> get_exploited_vulnerabilities()
Show latest exploited vulnerabilities

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Vulnerability>**](Vulnerability.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_last_vulnerabilities

> Vec<models::Vulnerability> get_last_vulnerabilities()
Show latest vulnerabilities

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Vulnerability>**](Vulnerability.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_vulnerability_by_id

> get_vulnerability_by_id(id)
Show vulnerability by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## query_vulnerabilities

> models::Vulnerabilities query_vulnerabilities(from_score, to_score, from_epss, to_epss, from_date, to_date, product, vendor, assigner, exploited, page, text, size)
Query vulnerabilities with flexible filters

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**from_score** | Option<**f32**> |  |  |
**to_score** | Option<**f32**> |  |  |
**from_epss** | Option<**f32**> |  |  |
**to_epss** | Option<**f32**> |  |  |
**from_date** | Option<**String**> |  |  |
**to_date** | Option<**String**> |  |  |
**product** | Option<**String**> |  |  |
**vendor** | Option<**String**> |  |  |
**assigner** | Option<**String**> |  |  |
**exploited** | Option<**bool**> |  |  |
**page** | Option<**i32**> |  |  |
**text** | Option<**String**> |  |  |
**size** | Option<**i32**> |  |  |[default to 10]

### Return type

[**models::Vulnerabilities**](Vulnerabilities.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

