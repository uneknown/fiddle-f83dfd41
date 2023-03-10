use fastly::error::anyhow;
use fastly::geo::geo_lookup;
use fastly::{Error, Request, Response};
use serde_json::json;

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    let client_ip = req
        .get_client_ip_addr()
        .ok_or_else(|| anyhow!("could not get client ip"))?;
    let geo = geo_lookup(client_ip).ok_or_else(|| anyhow!("no geographic data available"))?;
    let body = json!({
        "as": {
            "name": geo.as_name(),
        },
        "geo" : {
            "city": geo.city(),
            "client_ip": client_ip,
            "country_name": geo.country_name(),
            "gmt_offset:": geo.utc_offset().unwrap().to_string(),
        },
    });

    Ok(Response::new().with_body_json(&body)?)
}