use neon::prelude::*;
use yubikey::{
    // certificate::{Certificate, PublicKeyInfo},
    // piv::{self, AlgorithmId, Key, RetiredSlotId, SlotId},
    // Error, MgmKey, PinPolicy, TouchPolicy, 
    YubiKey,
};

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn yubikey_serial(mut cx: FunctionContext) -> JsResult<JsString> {
    let yubikey = YubiKey::open().unwrap();
    Ok(cx.string(  yubikey.serial().to_string() ))
    // Ok(cs.string( yubikey.serial() ))
}

fn yubikey_version(mut cx: FunctionContext) -> JsResult<JsString> {
    let yubikey = YubiKey::open().unwrap();
    Ok(cx.string(  yubikey.version().to_string() ))
    // Ok(cs.string( yubikey.version() ))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("yubikeySerial", yubikey_serial)?;
    cx.export_function("yubikeyVersion", yubikey_version)?;
    Ok(())
}
