use std::error::Error;

use neon::vm::{Call, JsResult, Lock};
use neon::js::binary::JsBuffer;
use neon::js::error::{JsError, Kind};
use neon::js::{JsArray, JsBoolean, JsFunction, JsInteger, JsNull, JsString, JsUndefined, Object,
               ToJsString, Value};

use rusty_secrets::sss;

use util::{recovery_error_to_js, to_u8};

pub fn split_secret(call: Call) -> JsResult<JsUndefined> {
    let scope = call.scope;
    let args = call.arguments;
    let threshold_handle = args.require(scope, 0)?;
    let shares_count_handle = args.require(scope, 1)?;
    let secret_handle = args.require(scope, 2)?;
    let sign_handle = args.require(scope, 3)?;
    let cb_handle = args.require(scope, 4)?;

    let threshold = to_u8(
        threshold_handle.check::<JsInteger>()?.value(),
        "Invalid threshold",
    )?;

    let shares_count = to_u8(
        shares_count_handle.check::<JsInteger>()?.value(),
        "Invalid shares count",
    )?;

    let cb = cb_handle.check::<JsFunction>()?;
    let sign = sign_handle.check::<JsBoolean>()?.value();
    let secret = secret_handle
        .check::<JsBuffer>()?
        .grab(|contents| contents.as_slice().to_vec());

    let (result, err) = match sss::split_secret(threshold, shares_count, &secret, sign) {
        Ok(shares) => {
            let result = JsArray::new(scope, shares.len() as u32);

            for (i, share) in shares.into_iter().enumerate() {
                result.set(i as u32, share.as_str().to_js_string(scope))?;
            }

            (result.as_value(scope), JsNull::new().as_value(scope))
        }
        Err(err) => (
            JsNull::new().as_value(scope),
            JsError::new(scope, Kind::Error, err.description())?.as_value(scope),
        ),
    };

    cb.call(scope, JsNull::new(), vec![err, result])?;

    Ok(JsUndefined::new())
}

pub fn recover_secret(call: Call) -> JsResult<JsUndefined> {
    let scope = call.scope;
    let args = call.arguments;
    let shares_handle = args.require(scope, 0)?;
    let verify_handle = args.require(scope, 1)?;
    let cb_handle = args.require(scope, 2)?;

    let js_shares = shares_handle.check::<JsArray>()?.to_vec(scope)?;

    let mut shares = Vec::with_capacity(js_shares.len());
    for js_share in js_shares {
        let share = js_share.check::<JsString>()?;
        shares.push(share.value());
    }

    let verify = verify_handle.check::<JsBoolean>()?.value();
    let cb = cb_handle.check::<JsFunction>()?;

    let secret = match sss::recover_secret(&shares, verify) {
        Ok(bytes) => {
            let mut buffer = JsBuffer::new(scope, bytes.len() as u32)?;
            buffer.grab(|mut contents| {
                contents.as_mut_slice().copy_from_slice(&bytes);
            });
            Ok(buffer)
        }
        Err(err) => Err(err),
    };

    let (result, err) = match secret {
        Ok(secret) => (secret.as_value(scope), JsNull::new().as_value(scope)),
        Err(err) => (
            JsNull::new().as_value(scope),
            recovery_error_to_js(scope, &err)?.as_value(scope),
        ),
    };

    cb.call(scope, JsNull::new(), vec![err, result])?;

    Ok(JsUndefined::new())
}
