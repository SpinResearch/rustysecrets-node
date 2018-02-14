extern crate base64;
#[macro_use]
extern crate neon;
extern crate protobuf;
extern crate rusty_secrets;

mod util;
mod sss;
mod wrapped;

register_module!(m, {
    m.export("sss_splitSecret", sss::split_secret)?;
    m.export("sss_recoverSecret", sss::recover_secret)?;

    m.export("wrapped_splitSecret", wrapped::split_secret)?;
    m.export("wrapped_recoverSecret", wrapped::recover_secret)?;

    Ok(())
});
