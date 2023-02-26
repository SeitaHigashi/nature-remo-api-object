# \DefaultApi

All URIs are relative to *https://api.nature.global*

Method | HTTP request | Description
------------- | ------------- | -------------
[**call_1_appliance_orders_post**](DefaultApi.md#call_1_appliance_orders_post) | **POST** /1/appliance_orders | 
[**call_1_appliances_applianceid_aircon_settings_post**](DefaultApi.md#call_1_appliances_applianceid_aircon_settings_post) | **POST** /1/appliances/{applianceid}/aircon_settings | 
[**call_1_appliances_applianceid_delete_post**](DefaultApi.md#call_1_appliances_applianceid_delete_post) | **POST** /1/appliances/{applianceid}/delete | 
[**call_1_appliances_applianceid_light_post**](DefaultApi.md#call_1_appliances_applianceid_light_post) | **POST** /1/appliances/{applianceid}/light | 
[**call_1_appliances_applianceid_post**](DefaultApi.md#call_1_appliances_applianceid_post) | **POST** /1/appliances/{applianceid} | 
[**call_1_appliances_applianceid_signal_orders_post**](DefaultApi.md#call_1_appliances_applianceid_signal_orders_post) | **POST** /1/appliances/{applianceid}/signal_orders | 
[**call_1_appliances_applianceid_signals_get**](DefaultApi.md#call_1_appliances_applianceid_signals_get) | **GET** /1/appliances/{applianceid}/signals | 
[**call_1_appliances_applianceid_signals_post**](DefaultApi.md#call_1_appliances_applianceid_signals_post) | **POST** /1/appliances/{applianceid}/signals | 
[**call_1_appliances_applianceid_tv_post**](DefaultApi.md#call_1_appliances_applianceid_tv_post) | **POST** /1/appliances/{applianceid}/tv | 
[**call_1_appliances_get**](DefaultApi.md#call_1_appliances_get) | **GET** /1/appliances | 
[**call_1_appliances_post**](DefaultApi.md#call_1_appliances_post) | **POST** /1/appliances | 
[**call_1_detectappliance_post**](DefaultApi.md#call_1_detectappliance_post) | **POST** /1/detectappliance | 
[**call_1_devices_deviceid_delete_post**](DefaultApi.md#call_1_devices_deviceid_delete_post) | **POST** /1/devices/{deviceid}/delete | 
[**call_1_devices_deviceid_humidity_offset_post**](DefaultApi.md#call_1_devices_deviceid_humidity_offset_post) | **POST** /1/devices/{deviceid}/humidity_offset | 
[**call_1_devices_deviceid_post**](DefaultApi.md#call_1_devices_deviceid_post) | **POST** /1/devices/{deviceid} | 
[**call_1_devices_deviceid_send_post**](DefaultApi.md#call_1_devices_deviceid_send_post) | **POST** /1/devices/{deviceid}/send | 
[**call_1_devices_deviceid_temperature_offset_post**](DefaultApi.md#call_1_devices_deviceid_temperature_offset_post) | **POST** /1/devices/{deviceid}/temperature_offset | 
[**call_1_devices_get**](DefaultApi.md#call_1_devices_get) | **GET** /1/devices | 
[**call_1_homes_get**](DefaultApi.md#call_1_homes_get) | **GET** /1/homes | 
[**call_1_homes_homeid_delete_post**](DefaultApi.md#call_1_homes_homeid_delete_post) | **POST** /1/homes/{homeid}/delete | 
[**call_1_homes_homeid_devices_get**](DefaultApi.md#call_1_homes_homeid_devices_get) | **GET** /1/homes/{homeid}/devices | 
[**call_1_homes_homeid_invites_post**](DefaultApi.md#call_1_homes_homeid_invites_post) | **POST** /1/homes/{homeid}/invites | 
[**call_1_homes_homeid_kick_post**](DefaultApi.md#call_1_homes_homeid_kick_post) | **POST** /1/homes/{homeid}/kick | 
[**call_1_homes_homeid_location_delete_post**](DefaultApi.md#call_1_homes_homeid_location_delete_post) | **POST** /1/homes/{homeid}/location/delete | 
[**call_1_homes_homeid_location_post**](DefaultApi.md#call_1_homes_homeid_location_post) | **POST** /1/homes/{homeid}/location | 
[**call_1_homes_homeid_location_state_update_post**](DefaultApi.md#call_1_homes_homeid_location_state_update_post) | **POST** /1/homes/{homeid}/location_state/update | 
[**call_1_homes_homeid_owner_post**](DefaultApi.md#call_1_homes_homeid_owner_post) | **POST** /1/homes/{homeid}/owner | 
[**call_1_homes_homeid_part_post**](DefaultApi.md#call_1_homes_homeid_part_post) | **POST** /1/homes/{homeid}/part | 
[**call_1_homes_homeid_post**](DefaultApi.md#call_1_homes_homeid_post) | **POST** /1/homes/{homeid} | 
[**call_1_homes_homeid_transfer_tohomeid_post**](DefaultApi.md#call_1_homes_homeid_transfer_tohomeid_post) | **POST** /1/homes/{homeid}/transfer/{tohomeid} | 
[**call_1_homes_homeid_users_get**](DefaultApi.md#call_1_homes_homeid_users_get) | **GET** /1/homes/{homeid}/users | 
[**call_1_homes_post**](DefaultApi.md#call_1_homes_post) | **POST** /1/homes | 
[**call_1_invites_invitetoken_get**](DefaultApi.md#call_1_invites_invitetoken_get) | **GET** /1/invites/{invitetoken} | 
[**call_1_invites_invitetoken_post**](DefaultApi.md#call_1_invites_invitetoken_post) | **POST** /1/invites/{invitetoken} | 
[**call_1_signals_signalid_delete_post**](DefaultApi.md#call_1_signals_signalid_delete_post) | **POST** /1/signals/{signalid}/delete | 
[**call_1_signals_signalid_post**](DefaultApi.md#call_1_signals_signalid_post) | **POST** /1/signals/{signalid} | 
[**call_1_signals_signalid_send_post**](DefaultApi.md#call_1_signals_signalid_send_post) | **POST** /1/signals/{signalid}/send | 
[**call_1_users_me_get**](DefaultApi.md#call_1_users_me_get) | **GET** /1/users/me | 
[**call_1_users_me_post**](DefaultApi.md#call_1_users_me_post) | **POST** /1/users/me | 



## call_1_appliance_orders_post

> serde_json::Value call_1_appliance_orders_post(appliances)


Reorder appliances. Requires basic.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**appliances** | Option<**String**> | List of all appliance IDs, comma separated. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_appliances_applianceid_aircon_settings_post

> crate::models::AirconSettings call_1_appliances_applianceid_aircon_settings_post(applianceid, button, dir, dirh, mode, temp, temp_unit, vol)


Update air conditioner settings. Requires sendir OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applianceid** | **String** | Appliance Id | [required] |
**button** | Option<**String**> | Button. Specify 'power-off' always if you want the air conditioner powered off. Empty means powered on. |  |
**dir** | Option<**String**> | AC air direction. Empty means automatic. |  |
**dirh** | Option<**String**> | AC horizontal air direction. |  |
**mode** | Option<**String**> | AC operation mode. The range of operation modes which the air conditioner accepts depends on the air conditioner model. Check the 'AirConRangeMode' information in the response for the range of the particular air conditioner model. |  |
**temp** | Option<**String**> | Temperature. The temperature in string format. The unit is described in Aircon object. The range of Temperatures which the air conditioner accepts depends on the air conditioner model and operation mode. Check the 'AirConRangeMode' information in the response for the range of the particular air conditioner model and operation mode. |  |
**temp_unit** | Option<**String**> | Temperature unit. 'c' or 'f' or '' for unknown. |  |
**vol** | Option<**String**> | AC air volume. Empty means automatic. Numbers express the amount of volume. The range of AirVolumes which the air conditioner accepts depends on the air conditioner model and operation mode. Check the 'AirConRangeMode' information in the response for the range of the particular air conditioner model and operation mode. |  |

### Return type

[**crate::models::AirconSettings**](AirconSettings.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_appliances_applianceid_delete_post

> serde_json::Value call_1_appliances_applianceid_delete_post(applianceid)


Delete an appliance. Requires basic.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applianceid** | **String** | Appliance Id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_appliances_applianceid_light_post

> crate::models::LightState call_1_appliances_applianceid_light_post(applianceid, button)


Send light signal. Requires sendir OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applianceid** | **String** | Appliance Id | [required] |
**button** | Option<**String**> | Button name. |  |

### Return type

[**crate::models::LightState**](LightState.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_appliances_applianceid_post

> crate::models::ApplianceResponse call_1_appliances_applianceid_post(applianceid, image, nickname)


Update an appliance. Requires basic.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applianceid** | **String** | Appliance Id | [required] |
**image** | Option<**String**> | Basename of the image file included in the app. Ex: 'ico_ac_1'. |  |
**nickname** | Option<**String**> | Appliance name. |  |

### Return type

[**crate::models::ApplianceResponse**](ApplianceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_appliances_applianceid_signal_orders_post

> serde_json::Value call_1_appliances_applianceid_signal_orders_post(applianceid, signals)


Reorder signals. Requires basic.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applianceid** | **String** | Appliance Id | [required] |
**signals** | Option<**String**> | List of all signal IDs, comma separated. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_appliances_applianceid_signals_get

> Vec<crate::models::ApplianceResponseSignalsInner> call_1_appliances_applianceid_signals_get(applianceid)


Fetch the list of signals the user has access to. Requires basic.read OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applianceid** | **String** | Appliance Id | [required] |

### Return type

[**Vec<crate::models::ApplianceResponseSignalsInner>**](ApplianceResponse_signals_inner.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_appliances_applianceid_signals_post

> crate::models::Signal call_1_appliances_applianceid_signals_post(applianceid, image, message, name)


Create a new signal. Requires basic.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applianceid** | **String** | Appliance Id | [required] |
**image** | Option<**String**> | Basename of the image file included in the app. Ex: 'ico_io'. |  |
**message** | Option<**String**> | JSON serialized object describing infrared signals. Includes 'data', 'freq' and 'format' keys. |  |
**name** | Option<**String**> | Signal name. |  |

### Return type

[**crate::models::Signal**](Signal.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_appliances_applianceid_tv_post

> crate::models::TvState call_1_appliances_applianceid_tv_post(applianceid, button)


Send TV signal. Requires sendir OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**applianceid** | **String** | Appliance Id | [required] |
**button** | Option<**String**> | Button name. |  |

### Return type

[**crate::models::TvState**](TVState.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_appliances_get

> Vec<crate::models::ApplianceResponsesInner> call_1_appliances_get()


Fetch the list of appliances the user has access to. Requires basic.read OAuth2 scopes.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::ApplianceResponsesInner>**](ApplianceResponses_inner.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_appliances_post

> crate::models::ApplianceResponse call_1_appliances_post(device, image, model_type, nickname)


Create a new appliance. Requires basic.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**device** | Option<**String**> |  |  |
**image** | Option<**String**> | Basename of the image file included in the app. Ex: 'ico_ac_1'. |  |
**model_type** | Option<**String**> | Enum of 'AC', 'TV', 'Light' |  |
**nickname** | Option<**String**> | Appliance name. |  |

### Return type

[**crate::models::ApplianceResponse**](ApplianceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_detectappliance_post

> Vec<crate::models::ApplianceModelAndParamsInner> call_1_detectappliance_post(message)


Find the air conditioner best matching the provided infrared signal. Requires detectappliance OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**message** | Option<**String**> | JSON serialized object describing infrared signals. Includes 'data', 'freq' and 'format' keys. |  |

### Return type

[**Vec<crate::models::ApplianceModelAndParamsInner>**](ApplianceModelAndParams_inner.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_devices_deviceid_delete_post

> serde_json::Value call_1_devices_deviceid_delete_post(deviceid)


Delete a device. Requires basic.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deviceid** | **String** | Device Id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_devices_deviceid_humidity_offset_post

> crate::models::DeviceResponse call_1_devices_deviceid_humidity_offset_post(deviceid, offset)


Update a device's humidity offset. Requires basic.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deviceid** | **String** | Device Id | [required] |
**offset** | Option<**f32**> | Humidity offset value added to the measured humidity. |  |

### Return type

[**crate::models::DeviceResponse**](DeviceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_devices_deviceid_post

> crate::models::DeviceResponse call_1_devices_deviceid_post(deviceid, name)


Update a device. Requires basic.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deviceid** | **String** | Device Id | [required] |
**name** | Option<**String**> |  |  |

### Return type

[**crate::models::DeviceResponse**](DeviceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_devices_deviceid_send_post

> serde_json::Value call_1_devices_deviceid_send_post(deviceid, message)


Send a signal from this device. Requires sendrawir OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deviceid** | **String** | Device Id | [required] |
**message** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_devices_deviceid_temperature_offset_post

> crate::models::DeviceResponse call_1_devices_deviceid_temperature_offset_post(deviceid, offset)


Update a device's temperature offset. Requires basic.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**deviceid** | **String** | Device Id | [required] |
**offset** | Option<**f32**> | Temperature offset value added to the measured temperature. |  |

### Return type

[**crate::models::DeviceResponse**](DeviceResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_devices_get

> Vec<crate::models::DeviceResponsesInner> call_1_devices_get()


Fetch the list of Remo devices the user has access to. Requires basic.read OAuth2 scopes.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::DeviceResponsesInner>**](DeviceResponses_inner.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_homes_get

> Vec<crate::models::HomeResponsesInner> call_1_homes_get()


Fetch the list of homes the user has access to. Requires home.read OAuth2 scopes.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::HomeResponsesInner>**](HomeResponses_inner.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_homes_homeid_delete_post

> serde_json::Value call_1_homes_homeid_delete_post(homeid)


Delete a home. Requires home.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homeid** | **String** | Home Id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_homes_homeid_devices_get

> Vec<crate::models::DeviceResponsesInner> call_1_homes_homeid_devices_get(homeid)


Fetch the list of devices in a home. Requires home.read OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homeid** | **String** | Home Id | [required] |

### Return type

[**Vec<crate::models::DeviceResponsesInner>**](DeviceResponses_inner.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_homes_homeid_invites_post

> crate::models::HomeInvite call_1_homes_homeid_invites_post(homeid)


Create a new home invite. Requires home.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homeid** | **String** | Home Id | [required] |

### Return type

[**crate::models::HomeInvite**](HomeInvite.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_homes_homeid_kick_post

> serde_json::Value call_1_homes_homeid_kick_post(homeid, user)


Kick a user from a home. Requires home.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homeid** | **String** | Home Id | [required] |
**user** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_homes_homeid_location_delete_post

> crate::models::HomeResponse call_1_homes_homeid_location_delete_post(homeid)


Delete a home's location. Requires home.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homeid** | **String** | Home Id | [required] |

### Return type

[**crate::models::HomeResponse**](HomeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_homes_homeid_location_post

> crate::models::HomeResponse call_1_homes_homeid_location_post(homeid)


Update a home's location. Requires home.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homeid** | **String** | Home Id | [required] |

### Return type

[**crate::models::HomeResponse**](HomeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_homes_homeid_location_state_update_post

> serde_json::Value call_1_homes_homeid_location_state_update_post(homeid)


Update the user's location state for a home. Requires home.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homeid** | **String** | Home Id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_homes_homeid_owner_post

> crate::models::HomeResponse call_1_homes_homeid_owner_post(homeid, user)


Change the owner of the home. Requires home.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homeid** | **String** | Home Id | [required] |
**user** | Option<**String**> |  |  |

### Return type

[**crate::models::HomeResponse**](HomeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_homes_homeid_part_post

> serde_json::Value call_1_homes_homeid_part_post(homeid)


Part from a home. Requires home.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homeid** | **String** | Home Id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_homes_homeid_post

> crate::models::HomeResponse call_1_homes_homeid_post(homeid, name)


Update a home. Requires home.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homeid** | **String** | Home Id | [required] |
**name** | Option<**String**> |  |  |

### Return type

[**crate::models::HomeResponse**](HomeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_homes_homeid_transfer_tohomeid_post

> serde_json::Value call_1_homes_homeid_transfer_tohomeid_post(homeid, tohomeid, devices)


Transfer devices to another home. Requires home.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homeid** | **String** | Home Id | [required] |
**tohomeid** | **String** | Transfer to Home Id | [required] |
**devices** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_homes_homeid_users_get

> Vec<crate::models::UserAndRolesInner> call_1_homes_homeid_users_get(homeid)


Fetch the list of users in a home. Requires home.read OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**homeid** | **String** | Home Id | [required] |

### Return type

[**Vec<crate::models::UserAndRolesInner>**](UserAndRoles_inner.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_homes_post

> crate::models::HomeResponse call_1_homes_post(name)


Create a new home. Requires home.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**name** | Option<**String**> |  |  |

### Return type

[**crate::models::HomeResponse**](HomeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_invites_invitetoken_get

> crate::models::HomeInvite call_1_invites_invitetoken_get(invitetoken)


Show a home invite. Requires home.read OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitetoken** | **String** | Invite Token | [required] |

### Return type

[**crate::models::HomeInvite**](HomeInvite.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_invites_invitetoken_post

> crate::models::HomeResponse call_1_invites_invitetoken_post(invitetoken)


Accept a home invite. Requires home.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**invitetoken** | **String** | Invite Token | [required] |

### Return type

[**crate::models::HomeResponse**](HomeResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_signals_signalid_delete_post

> serde_json::Value call_1_signals_signalid_delete_post(signalid)


Delete a signal. Requires basic.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signalid** | **String** | Signal Id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_signals_signalid_post

> crate::models::Signal call_1_signals_signalid_post(signalid, image, name)


Update a signal. Requires basic.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signalid** | **String** | Signal Id | [required] |
**image** | Option<**String**> | Basename of the image file included in the app. Ex: 'ico_io'. |  |
**name** | Option<**String**> | Signal name. |  |

### Return type

[**crate::models::Signal**](Signal.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_signals_signalid_send_post

> serde_json::Value call_1_signals_signalid_send_post(signalid)


Send a signal. Requires sendir OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signalid** | **String** | Signal Id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_users_me_get

> crate::models::UserResponse call_1_users_me_get()


Fetch the authenticated user's information. Requires basic.read OAuth2 scopes.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::UserResponse**](UserResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## call_1_users_me_post

> crate::models::UserResponse call_1_users_me_post(country, distance_unit, nickname, temp_unit)


Update authenticated user's information. Requires basic.write OAuth2 scopes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**country** | Option<**String**> |  |  |
**distance_unit** | Option<**String**> |  |  |
**nickname** | Option<**String**> |  |  |
**temp_unit** | Option<**String**> |  |  |

### Return type

[**crate::models::UserResponse**](UserResponse.md)

### Authorization

[oauth2](../README.md#oauth2)

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

