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

fn main() -> Result<(), serde_json::Error> {
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
