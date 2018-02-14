use std::error::Error;

use neon::task::Task;
use neon::scope::Scope;
use neon::vm::{Call, JsResult, Lock};
use neon::js::binary::JsBuffer;
use neon::js::error::{throw, JsError, Kind};
use neon::js::{JsArray, JsBoolean, JsFunction, JsInteger, JsString, JsUndefined, Object,
               ToJsString};

use rusty_secrets;
use rusty_secrets::sss;

use util::{recovery_error_to_js, to_u8};

struct SplitSecretTask {
    threshold: u8,
    shares_count: u8,
    secret: Vec<u8>,
    sign_shares: bool,
}

impl Task for SplitSecretTask {
    type Output = Vec<String>;
    type Error = rusty_secrets::errors::Error;
    type JsEvent = JsArray;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        sss::split_secret(
            self.threshold,
            self.shares_count,
            self.secret.as_slice(),
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
    let sign_shares_handle = args.require(scope, 3)?;
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
    let sign_shares = sign_shares_handle.check::<JsBoolean>()?.value();
    let secret = secret_handle
        .check::<JsBuffer>()?
        .grab(|contents| contents.as_slice().to_vec());

    let task = SplitSecretTask {
        threshold,
        shares_count,
        secret,
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
    type Output = Vec<u8>;
    type Error = rusty_secrets::errors::Error;
    type JsEvent = JsBuffer;

    fn perform(&self) -> Result<Self::Output, Self::Error> {
        sss::recover_secret(&self.shares, self.verify_signatures)
    }

    fn complete<'a, T: Scope<'a>>(
        self,
        scope: &'a mut T,
        result: Result<Self::Output, Self::Error>,
    ) -> JsResult<Self::JsEvent> {
        match result {
            Ok(bytes) => {
                let mut buffer = JsBuffer::new(scope, bytes.len() as u32)?;
                buffer.grab(|mut contents| {
                    contents.as_mut_slice().copy_from_slice(&bytes);
                });
                Ok(buffer)
            }
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
