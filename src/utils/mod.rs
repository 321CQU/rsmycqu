//! [rsmycqu] 的工具库

use reqwest::{header::AsHeaderName, Response};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::{Map, Value};

use crate::{
    errors,
    errors::{ApiError, ApiResult},
};

pub(crate) trait ApiModel: Serialize + DeserializeOwned {
    fn parse_json_array<E: errors::RsMyCQUError>(
        array: &mut Vec<Value>,
    ) -> ApiResult<Vec<Self>, E> {
        array
            .iter_mut()
            .map(Value::take)
            .map(serde_json::from_value)
            .collect::<Result<Vec<Self>, _>>()
            .map_err(|e| errors::ApiError::ModelParse {
                msg: format!("Model parse error: {e:?}"),
            })
    }

    fn extract_array<E: errors::RsMyCQUError>(
        json: &mut Map<String, Value>,
        key: &str,
    ) -> ApiResult<Vec<Self>, E> {
        json.get_mut(key)
            .and_then(Value::as_array_mut)
            .ok_or(errors::ApiError::ModelParse {
                msg: format!("Expected field \"{}\" is missing or format incorrect", key),
            })
            .and_then(Self::parse_json_array)
    }

    fn extract_object<E: errors::RsMyCQUError>(
        json: &mut Map<String, Value>,
        key: &str,
    ) -> ApiResult<Self, E> {
        json.get_mut(key)
            .map(Value::take)
            .ok_or(errors::ApiError::ModelParse {
                msg: format!("Expected field \"{}\" is missing or format incorrect", key),
            })
            .map(serde_json::from_value)?
            .map_err(|e| ApiError::ModelParse {
                msg: format!("Model parse error: {e:?}"),
            })
    }
}

pub mod models;

pub(crate) mod consts;
#[macro_use]
pub(crate) mod macros;

#[cfg(feature = "mycqu")]
pub(crate) mod datetimes;
#[cfg(feature = "sso")]
pub(crate) mod encrypt;
#[cfg(feature = "sso")]
pub(crate) mod page_parser;

#[cfg(test)]
pub(crate) mod test_fixture;

#[inline]
pub(crate) fn get_response_header(res: &Response, target: impl AsHeaderName) -> Option<&str> {
    res.headers()
        .get(target)
        .and_then(|value| value.to_str().ok())
}
