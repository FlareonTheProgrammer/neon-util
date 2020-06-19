use neon::prelude::*;
use rand::Rng;
use std::format;

fn do_act(mut cx: FunctionContext) -> JsResult<JsString> {
    let acts = ["write about", "draw"];
    let subjects = ["landscape", "camping trip", "sunset"];

    let rand_subj = format!(
        "You should {} a {}",
        acts[rand::thread_rng().gen_range(0, 2)],
        subjects[rand::thread_rng().gen_range(0, 3)]
    );

    Ok(cx.string(rand_subj))
}

fn rn_gen(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let lower = cx.argument::<JsNumber>(0)?.value().round() as i64;
    let upper = cx.argument::<JsNumber>(1)?.value().round() as i64;
    Ok(cx.number(rand::thread_rng().gen_range(lower, upper) as f64))
}

register_module!(mut cx, {
    cx.export_function("doAct", do_act)?;
    cx.export_function("rnGen", rn_gen)?;
    Ok(())
});
