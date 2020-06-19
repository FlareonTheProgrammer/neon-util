use neon::prelude::*;
use rand::Rng;
use std::format;

fn coin_flip(mut cx: FunctionContext) -> JsResult<JsString> {
    let mut _reply = String::from("");
    let prob = rand::thread_rng().gen_range(1,100002);
    match prob {
        1..=50000 => _reply = String::from("Heads!"),
        50001..=100000 => _reply = String::from("Tails!"),
        100001 => _reply = String::from("It... landed on its side?"),
        _ => panic!(),
    }

    Ok(cx.string(_reply))
}

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
    cx.export_function("coinFlip", coin_flip)?;
    cx.export_function("doAct", do_act)?;
    cx.export_function("rng", rn_gen)?;
    Ok(())
});
