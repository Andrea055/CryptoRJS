use neon::prelude::*;
use md5::{Md5, Digest};

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn MD5(mut cx: FunctionContext) -> JsResult<JsString> {
    let mut hasher = Md5::new();
    let word = cx.argument::<JsString>(0).unwrap().value(&mut cx);
    hasher.update(word);
    Ok(cx.string(hex::encode(hasher.finalize())))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("md5", MD5)?;
    Ok(())
}
