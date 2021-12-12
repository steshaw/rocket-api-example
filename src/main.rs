//#![feature(decl_macro)]
//#![feature(proc_macro_hygiene)]

use rocket::get;
use rocket::serde::json::Json;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
struct Episode {
    id: String,
    short_title: String,
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
struct Clip {
    id: String,
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
struct Program {
    id: String,
}

#[derive(Debug, JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Extra {
    Episode(Episode),
    Clip(Clip),
    Program(Program),
}

#[openapi]
#[get("/")]
fn index() -> &'static str {
    "Hello!\n"
}

#[openapi]
#[get("/extras")]
fn get_extras() -> Json<Vec<Extra>> {
    Json(extras().into())
}

fn extras() -> [Extra; 3] {
    [
        Extra::Episode(Episode {
            id: "ep-1".to_string(),
            short_title: "The Blade Itself".to_string(),
        }),
        Extra::Clip(Clip {
            id: "cl-1".to_string(),
        }),
        Extra::Program(Program {
            id: "pg-1".to_string(),
        }),
    ]
}

fn foo() -> Result<(), serde_json::Error> {
    let ep = Episode {
        id: "0a234-234-23".to_string(),
        short_title: "The Blade Itself".to_string(),
    };
    let json = serde_json::to_string_pretty(&ep)?;
    println!("ep = {}", json);

    let extra = Extra::Episode(ep);
    println!("extra = {}", serde_json::to_string_pretty(&extra)?);

    let extras = extras();
    println!("extras = {}", serde_json::to_string_pretty(&extras)?);

    let schema = schemars::schema_for!(Episode);
    println!("Episode schema = {}", serde_json::to_string_pretty(&schema)?);

    let schema = schemars::schema_for!(Clip);
    println!("Clip schema = {}", serde_json::to_string_pretty(&schema)?);

    let schema = schemars::schema_for!(Program);
    println!("Program schema = {}", serde_json::to_string_pretty(&schema)?);

    let schema = schemars::schema_for!(Extra);
    println!("Extra schema = {}", serde_json::to_string_pretty(&schema)?);

    Ok(())
}

fn routes() -> Vec<rocket::Route> {
    openapi_get_routes![index, get_extras]
}

#[rocket::launch]
fn rocket() -> _ {
    let _ = foo();
    rocket::build()
        .mount("/", routes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn foo() {
        let actual = 1 + 1;
        assert_eq!(2, actual);
    }

    #[test]
    fn argh() {
        if false {
            panic!("boo!");
        }
    }

    #[test]
    fn test_index() {
        let rkt = rocket::build().mount("/", routes![index]);
        let client = Client::tracked(rkt).expect("valid rocket");
        let resp = client.get("/").dispatch();
        assert_eq!(Status::Ok, resp.status());
        let body = resp.into_string();
        assert_eq!(Some("Hello!\n".into()), body);
    }
}
