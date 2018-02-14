use std::error::Error;

use neon::scope::Scope;
use neon::vm::{Call, JsResult, Lock};
use neon::js::binary::JsBuffer;
use neon::js::error::{JsError, Kind};
use neon::js::{JsArray, JsBoolean, JsFunction, JsInteger, JsNull, JsObject, JsString, JsUndefined,
               Object, ToJsString, Value};

use rusty_secrets::wrapped_secrets;
use rusty_secrets::proto::wrapped::SecretProto;

use util::{recovery_error_to_js, to_u8};

pub fn split_secret(call: Call) -> JsResult<JsUndefined> {
    let scope = call.scope;
    let args = call.arguments;
    let threshold_handle = args.require(scope, 0)?;
    let shares_count_handle = args.require(scope, 1)?;
    let secret_handle = args.require(scope, 2)?;
    let mime_type_handle = args.require(scope, 3)?;
    let sign_handle = args.require(scope, 4)?;
    let cb_handle = args.require(scope, 5)?;

    let threshold = to_u8(
        threshold_handle.check::<JsInteger>()?.value(),
        "Invalid threshold",
    )?;

    let shares_count = to_u8(
        shares_count_handle.check::<JsInteger>()?.value(),
        "Invalid shares count",
    )?;

    let cb = cb_handle.check::<JsFunction>()?;
    let mime_type = mime_type_handle.downcast::<JsString>().map(|s| s.value());
    let sign = sign_handle.check::<JsBoolean>()?.value();
    let secret = secret_handle
        .check::<JsBuffer>()?
        .grab(|contents| contents.as_slice().to_vec());

    let (result, err) =
        match wrapped_secrets::split_secret(threshold, shares_count, &secret, mime_type, sign) {
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

    let (secret, err) = match wrapped_secrets::recover_secret(&shares, verify) {
        Ok(proto) => (
            proto_to_js_obj(scope, &proto)?.as_value(scope),
            JsNull::new().as_value(scope),
        ),
        Err(err) => (
            JsNull::new().as_value(scope),
            recovery_error_to_js(scope, &err)?.as_value(scope),
        ),
    };

    cb.call(scope, JsNull::new(), vec![err, secret])?;

    Ok(JsUndefined::new())
}

fn proto_to_js_obj<'a, S: Scope<'a>>(scope: &mut S, proto: &SecretProto) -> JsResult<'a, JsObject> {
    use protobuf::core::ProtobufEnum;

    let obj = JsObject::new(scope);

    let proto_version = proto.get_version();
    let versions = proto_version.enum_descriptor();
    let version_str = versions.value_by_number(proto_version.value()).name();
    let version = JsString::new_or_throw(scope, version_str)?;

    let secret_vec = proto.get_secret();
    let mut buffer = JsBuffer::new(scope, secret_vec.len() as u32)?;
    buffer.grab(|mut contents| {
        contents.as_mut_slice().copy_from_slice(secret_vec);
    });

    let mime_type_str = proto.get_mime_type();
    let mime_type = if mime_type_str.is_empty() {
        JsNull::new().as_value(scope)
    } else {
        JsString::new_or_throw(scope, mime_type_str)?.as_value(scope)
    };

    obj.set("version", version)?;
    obj.set("secret", buffer)?;
    obj.set("mimeType", mime_type)?;

    Ok(obj)
}
