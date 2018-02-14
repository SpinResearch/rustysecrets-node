extern crate base64;
#[macro_use]
extern crate neon;
extern crate rusty_secrets;

use rusty_secrets::dss::ss1::{self, Reproducibility};

use neon::vm::{Call, JsResult};
use neon::js::{JsArray, JsInteger, JsObject, JsString, Object};

fn split_secret(call: Call) -> JsResult<JsArray> {
    let scope = call.scope;
    let args = call.arguments;
    let threshold_handle = args.require(scope, 0)?;
    let shares_count_handle = args.require(scope, 1)?;
    let secret_handle = args.require(scope, 2)?;

    let threshold = threshold_handle.check::<JsInteger>()?.value() as u8; // FIXME: Unsafe cast
    let shares_count = shares_count_handle.check::<JsInteger>()?.value() as u8; // FIXME: unsafe cast
    let secret = secret_handle.check::<JsString>()?.value();

    let shares = ss1::split_secret(
        threshold,
        shares_count,
        &secret.into_bytes(),
        Reproducibility::reproducible(),
        &None,
    ).unwrap();

    let shares_count = shares.len();

    let mut shares_js = Vec::with_capacity(shares_count);

    for share in shares {
        let obj = JsObject::new(scope);
        obj.set("id", JsInteger::new(scope, share.id as i32))?;
        obj.set("threshold", JsInteger::new(scope, share.threshold as i32))?;
        obj.set(
            "shares_count",
            JsInteger::new(scope, share.shares_count as i32),
        )?;
        obj.set(
            "data",
            JsString::new_or_throw(scope, &base64::encode(&share.data))?,
        )?;

        shares_js.push(obj);
    }

    let result = JsArray::new(scope, shares_count as u32);

    for (i, share_obj) in shares_js.into_iter().enumerate() {
        result.set(i as u32, share_obj)?;
    }

    Ok(result)
}

register_module!(m, { m.export("splitSecret", split_secret) });
