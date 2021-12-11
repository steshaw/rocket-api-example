#![feature(decl_macro)]

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
    "Hello!"
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

    let extras = [
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
    ];
    println!("extras = {}", serde_json::to_string_pretty(&extras)?);

    Ok(())
}
fn main() {
    let _ = foo();
    rocket::ignite().mount("/", rocket::routes![index]).launch();
}
