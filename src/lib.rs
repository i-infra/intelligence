use rand::seq::SliceRandom;
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    let mut rng = rand::thread_rng();
    let some_elements_of_str = include_str!("some_elements_of_intelligence_work-dulles.txt");
    let mut sabo2: Vec<&str> = include_str!("sabo.txt").split("\n\n").collect();
    sabo2.shuffle(&mut rng);
    let sabo3: Vec<&str> = sabo2.get(0..73).unwrap().to_vec();
    let mut cleaned: Vec<&str> = some_elements_of_str.split("\n\n").chain(sabo3).collect();
    cleaned.shuffle(&mut rng);

    let router = Router::with_data(cleaned);

    router
        .get("/", |_, ctx| {
            let mut r = Response::ok(ctx.data()[0]).unwrap();
            r.headers_mut()
                .set("Content-type", "text/plain; charset=UTF-8");
            return Ok(r);
        })
        .get("/worker-version", |_, ctx| {
            let version = ctx.var("WORKERS_RS_VERSION")?.to_string();
            Response::ok(version)
        })
        .run(req, env)
        .await
}
