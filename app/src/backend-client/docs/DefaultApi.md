# DefaultApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**addStation**](#addstation) | **POST** /stations | |
|[**checkinStation**](#checkinstation) | **PUT** /stations/{stationId}/checkin | |
|[**deleteStation**](#deletestation) | **DELETE** /stations/{stationId} | |
|[**getProfiles**](#getprofiles) | **GET** /profiles | |
|[**getStation**](#getstation) | **GET** /stations/{stationId} | |
|[**getStationLog**](#getstationlog) | **GET** /stations/{stationId}/log | |
|[**listStations**](#liststations) | **GET** /stations | |
|[**setProfile**](#setprofile) | **POST** /profiles/profile/{stationId} | |
|[**updateStation**](#updatestation) | **PUT** /stations/{stationId} | |
|[**uploadAvatar**](#uploadavatar) | **POST** /stations/{stationId}/upload | |
|[**viewAvatar**](#viewavatar) | **GET** /stations/{stationId}/avatar | |
|[**wateredAtStation**](#wateredatstation) | **POST** /stations/{stationId}/watered | |

# **addStation**
> string addStation(stationInsert)


### Example

```typescript
import {
    DefaultApi,
    Configuration,
    StationInsert
} from './api';

const configuration = new Configuration();
const apiInstance = new DefaultApi(configuration);

let stationInsert: StationInsert; //

const { status, data } = await apiInstance.addStation(
    stationInsert
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **stationInsert** | **StationInsert**|  | |


### Return type

**string**

### Authorization

[httpAuth](../README.md#httpAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, text/plain


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Invalid value for: body |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **checkinStation**
> Watering checkinStation()


### Example

```typescript
import {
    DefaultApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new DefaultApi(configuration);

let stationId: string; // (default to undefined)
let stationMeasurement: Array<StationMeasurement>; // (optional)

const { status, data } = await apiInstance.checkinStation(
    stationId,
    stationMeasurement
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **stationMeasurement** | **Array<StationMeasurement>**|  | |
| **stationId** | [**string**] |  | defaults to undefined|


### Return type

**Watering**

### Authorization

[httpAuth](../README.md#httpAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json, text/plain


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Invalid value for: path parameter stationId, Invalid value for: body |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **deleteStation**
> deleteStation()


### Example

```typescript
import {
    DefaultApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new DefaultApi(configuration);

let stationId: string; // (default to undefined)

const { status, data } = await apiInstance.deleteStation(
    stationId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **stationId** | [**string**] |  | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[httpAuth](../README.md#httpAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/plain


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Invalid value for: path parameter stationId |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getProfiles**
> Array<StationPlantProfile> getProfiles()


### Example

```typescript
import {
    DefaultApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new DefaultApi(configuration);

const { status, data } = await apiInstance.getProfiles();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Array<StationPlantProfile>**

### Authorization

[httpAuth](../README.md#httpAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getStation**
> StationDetails getStation()


### Example

```typescript
import {
    DefaultApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new DefaultApi(configuration);

let stationId: string; // (default to undefined)
let period: string; // (optional) (default to undefined)

const { status, data } = await apiInstance.getStation(
    stationId,
    period
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **stationId** | [**string**] |  | defaults to undefined|
| **period** | [**string**] |  | (optional) defaults to undefined|


### Return type

**StationDetails**

### Authorization

[httpAuth](../README.md#httpAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/plain


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Invalid value for: path parameter stationId |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getStationLog**
> Array<StationLog> getStationLog()


### Example

```typescript
import {
    DefaultApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new DefaultApi(configuration);

let stationId: string; // (default to undefined)
let page: number; // (optional) (default to undefined)

const { status, data } = await apiInstance.getStationLog(
    stationId,
    page
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **stationId** | [**string**] |  | defaults to undefined|
| **page** | [**number**] |  | (optional) defaults to undefined|


### Return type

**Array<StationLog>**

### Authorization

[httpAuth](../README.md#httpAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/plain


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Invalid value for: path parameter stationId, Invalid value for: query parameter page |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listStations**
> Array<Station> listStations()


### Example

```typescript
import {
    DefaultApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new DefaultApi(configuration);

const { status, data } = await apiInstance.listStations();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**Array<Station>**

### Authorization

[httpAuth](../README.md#httpAuth)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **setProfile**
> setProfile(plantProfile)


### Example

```typescript
import {
    DefaultApi,
    Configuration,
    PlantProfile
} from './api';

const configuration = new Configuration();
const apiInstance = new DefaultApi(configuration);

let stationId: string; // (default to undefined)
let plantProfile: PlantProfile; //

const { status, data } = await apiInstance.setProfile(
    stationId,
    plantProfile
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **plantProfile** | **PlantProfile**|  | |
| **stationId** | [**string**] |  | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[httpAuth](../README.md#httpAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: text/plain


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Invalid value for: path parameter stationId, Invalid value for: body |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **updateStation**
> updateStation(stationUpdate)


### Example

```typescript
import {
    DefaultApi,
    Configuration,
    StationUpdate
} from './api';

const configuration = new Configuration();
const apiInstance = new DefaultApi(configuration);

let stationId: string; // (default to undefined)
let stationUpdate: StationUpdate; //

const { status, data } = await apiInstance.updateStation(
    stationId,
    stationUpdate
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **stationUpdate** | **StationUpdate**|  | |
| **stationId** | [**string**] |  | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[httpAuth](../README.md#httpAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: text/plain


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Invalid value for: path parameter stationId, Invalid value for: body |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **uploadAvatar**
> Array<PlantProfile> uploadAvatar(body)


### Example

```typescript
import {
    DefaultApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new DefaultApi(configuration);

let stationId: string; // (default to undefined)
let body: number; //

const { status, data } = await apiInstance.uploadAvatar(
    stationId,
    body
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **body** | **number**|  | |
| **stationId** | [**string**] |  | defaults to undefined|


### Return type

**Array<PlantProfile>**

### Authorization

[httpAuth](../README.md#httpAuth)

### HTTP request headers

 - **Content-Type**: application/octet-stream
 - **Accept**: application/json, text/plain


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Invalid value for: path parameter stationId, Invalid value for: body |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **viewAvatar**
> number viewAvatar()


### Example

```typescript
import {
    DefaultApi,
    Configuration
} from './api';

const configuration = new Configuration();
const apiInstance = new DefaultApi(configuration);

let stationId: string; // (default to undefined)

const { status, data } = await apiInstance.viewAvatar(
    stationId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **stationId** | [**string**] |  | defaults to undefined|


### Return type

**number**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/octet-stream, text/plain


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Invalid value for: path parameter stationId |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **wateredAtStation**
> wateredAtStation(watering)


### Example

```typescript
import {
    DefaultApi,
    Configuration,
    Watering
} from './api';

const configuration = new Configuration();
const apiInstance = new DefaultApi(configuration);

let stationId: string; // (default to undefined)
let watering: Watering; //

const { status, data } = await apiInstance.wateredAtStation(
    stationId,
    watering
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **watering** | **Watering**|  | |
| **stationId** | [**string**] |  | defaults to undefined|


### Return type

void (empty response body)

### Authorization

[httpAuth](../README.md#httpAuth)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: text/plain


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** |  |  -  |
|**400** | Invalid value for: path parameter stationId, Invalid value for: body |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

