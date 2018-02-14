use std::error::Error;

use neon::task::Task;
use neon::scope::Scope;
use neon::vm::{Call, JsResult, Lock};
use neon::js::binary::JsBuffer;
use neon::js::error::{throw, JsError, Kind};
use neon::js::{JsArray, JsBoolean, JsFunction, JsInteger, JsNull, JsObject, JsString, JsUndefined,
               Object, ToJsString, Value};

use rusty_secrets;
use rusty_secrets::wrapped_secrets;
use rusty_secrets::proto::wrapped::SecretProto;

use util::{recovery_error_to_js, to_u8};

struct SplitSecretTask {
    threshold: u8,
    shares_count: u8,
    secret: Vec<u8>,
    mime_type: Option<String>,
    sign_shares: bool,
}

impl Task for SplitSecretTask {
    type Output = Vec<String>;
    type Error = rusty_secrets::errors::Error;
    type JsEvent = JsArray;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        wrapped_secrets::split_secret(
            self.threshold,
            self.shares_count,
            self.secret.as_slice(),
            self.mime_type.clone(),
            self.sign_shares,
        )
    }

    fn complete<'a, T: Scope<'a>>(
        self,
        scope: &'a mut T,
        result: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
        match result {
            Ok(shares) => {
                let output = JsArray::new(scope, shares.len() as u32);

                for (i, share) in shares.into_iter().enumerate() {
                    output.set(i as u32, share.as_str().to_js_string(scope))?;
                }

                Ok(output)
            }
            Err(err) => JsError::throw(Kind::Error, err.description()),
        }
    }
}

pub fn split_secret(call: Call) -> JsResult<JsUndefined> {
    let scope = call.scope;
    let args = call.arguments;
    let threshold_handle = args.require(scope, 0)?;
    let shares_count_handle = args.require(scope, 1)?;
    let secret_handle = args.require(scope, 2)?;
    let mime_type_handle = args.require(scope, 3)?;
    let sign_shares_handle = args.require(scope, 4)?;
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
    let sign_shares = sign_shares_handle.check::<JsBoolean>()?.value();
    let secret = secret_handle
        .check::<JsBuffer>()?
        .grab(|contents| contents.as_slice().to_vec());

    let task = SplitSecretTask {
        threshold,
        shares_count,
        secret,
        mime_type,
        sign_shares,
    };

    task.schedule(cb);

    Ok(JsUndefined::new())
}

struct RecoverSecretTask {
    shares: Vec<String>,
    verify_signatures: bool,
}

impl Task for RecoverSecretTask {
    type Output = SecretProto;
    type Error = rusty_secrets::errors::Error;
    type JsEvent = JsObject;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        wrapped_secrets::recover_secret(&self.shares, self.verify_signatures)
    }

    fn complete<'a, T: Scope<'a>>(
        self,
        scope: &'a mut T,
        result: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
        match result {
            Ok(proto) => proto_to_js_obj(scope, &proto),
            Err(err) => throw(recovery_error_to_js(scope, &err)?),
        }
    }
}

pub fn recover_secret(call: Call) -> JsResult<JsUndefined> {
    let scope = call.scope;
    let args = call.arguments;

    let js_shares = args.require(scope, 0)?.check::<JsArray>()?.to_vec(scope)?;
    let verify_signatures = args.require(scope, 1)?.check::<JsBoolean>()?.value();
    let cb = args.require(scope, 2)?.check::<JsFunction>()?;

    let mut shares = Vec::with_capacity(js_shares.len());
    for js_share in js_shares {
        let share = js_share.check::<JsString>()?;
        shares.push(share.value());
    }

    let task = RecoverSecretTask {
        shares,
        verify_signatures,
    };

    task.schedule(cb);

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
