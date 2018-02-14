use std;
use std::error::Error;

use neon::scope::Scope;
use neon::vm::{JsResult, VmResult};
use neon::js::{JsArray, JsInteger, JsObject, JsString, Object};
use neon::js::error::{JsError, Kind};

use rusty_secrets::errors;

pub(crate) fn to_u8(n: i64, err: &str) -> VmResult<u8> {
    if n < 0 || n > i64::from(std::u8::MAX) {
        JsError::throw(Kind::Error, err)
    } else {
        Ok(n as u8)
    }
}

pub(crate) fn recovery_error_to_js<'a, S: Scope<'a>>(
    scope: &mut S,
    err: &errors::Error,
) -> JsResult<'a, JsObject> {
    use self::errors::ErrorKind::*;

    let obj = JsObject::new(scope);
    obj.set("message", JsString::new_or_throw(scope, err.description())?)?;

    let (share_id, sets) = match *err.kind() {
        InvalidSignature(share_id, _) => (Some(share_id), None),
        MissingSignature(share_id) => (Some(share_id), None),
        ShareParsingErrorEmptyShare(share_id) => (Some(share_id), None),
        ShareParsingInvalidShareId(share_id) => (Some(share_id), None),
        DuplicateShareId(share_id) => (Some(share_id), None),
        DuplicateShareData(share_id) => (Some(share_id), None),
        IncompatibleSets(ref sets) => (None, Some(sets)),
        _ => (None, None),
    };

    if let Some(share_id) = share_id {
        obj.set("share_num", JsInteger::new(scope, i32::from(share_id)))?;
    }

    if let Some(sets) = sets {
        let share_groups: Vec<Vec<u8>> = sets.into_iter()
            .map(|s| s.clone().into_iter().collect())
            .collect();

        let array = JsArray::new(scope, share_groups.len() as u32);

        for (i, shares) in share_groups.into_iter().enumerate() {
            let group = JsArray::new(scope, shares.len() as u32);
            for (j, share) in shares.into_iter().enumerate() {
                group.set(
                    JsInteger::new(scope, j as i32),
                    JsInteger::new(scope, i32::from(share)),
                )?;
            }
            array.set(JsInteger::new(scope, i as i32), group)?;
        }
        obj.set("share_groups", array)?;
    }

    Ok(obj)
}
