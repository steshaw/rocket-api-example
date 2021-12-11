#![feature(decl_macro)]

use rocket::routes;
use rocket_contrib::json::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Episode {
    id: String,
    short_title: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Clip {
    id: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Program {
    id: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum Extra {
    Episode(Episode),
    Clip(Clip),
    Program(Program),
}

#[rocket::get("/")]
fn index() -> &'static str {
    "Hello!\n"
}

#[rocket::get("/extras")]
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

    Ok(())
}

fn routes() -> Vec<rocket::Route> {
    routes![index, get_extras]
}

fn main() {
    let _ = foo();
    rocket::ignite().mount("/", routes()).launch();
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::*;

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
        let rkt = rocket::ignite().mount("/", routes![index]);
        let client = Client::new(rkt).expect("valid rocket");
        let mut resp = client.get("/").dispatch();
        let body = resp.body_string();
        assert_eq!(Status::Ok, resp.status());
        assert_eq!(Some("Hello!\n".into()), body);
    }
}
