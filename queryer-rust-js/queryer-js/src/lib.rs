use neon::prelude::*;

fn example_sql(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(queryer::example_sql()))
}

fn query (mut cx: FunctionContext) -> JsResult<JsString>{
    let rt = tokio::runtime::Runtime::new().unwrap();
    let data = rt.block_on(async { queryer::query(queryer::example_sql()).await.unwrap() });
    Ok(cx.string(data.to_csv().unwrap()))
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("example_sql", example_sql)?;
    cx.export_function("query", query)?;
    Ok(())
}
