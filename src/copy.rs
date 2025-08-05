use clap::ArgAction;
use clap::{Arg, Command};
use reqwest::blocking::get;
use url::Url;

fn main() {
    let matches = Command::new("ParamExtractor")
        .version("1.0")
        .author("Ton Nom")
        .about("Extrait les paramètres d'une URL et peut rechercher leur présence dans le contenu")
        .arg(
            Arg::new("url")
                .help("L'URL à analyser")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("search")
                .short('s')
                .long("search")
                .help("Active la recherche des clés et valeurs dans la page")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let input_url = matches.get_one::<String>("url").unwrap();
    let do_search = matches.contains_id("search");

    let parsed_url = match Url::parse(input_url) {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Erreur lors du parsing de l'URL: {}", e);
            return;
        }
    };

    let query_params: Vec<(String, String)> = parsed_url
        .query_pairs()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    println!("🔍 Paramètres trouvés dans l'URL :");
    for (k, v) in &query_params {
        println!("  {} = {}", k, v);
    }

    if do_search {
        println!("\n🌐 Requête vers la page...");

        let response = match get(input_url) {
            Ok(resp) => match resp.text() {
                Ok(text) => text,
                Err(e) => {
                    eprintln!("Erreur en lisant la réponse : {}", e);
                    return;
                }
            },
            Err(e) => {
                eprintln!("Erreur lors de la requête HTTP : {}", e);
                return;
            }
        };

        let mut found_keys = vec![];
        let mut found_values = vec![];
        let mut found_combos = vec![];

        for (k, v) in &query_params {
            let key_found = response.contains(k);
            let value_found = response.contains(v);
            let combo = format!("{}={}", k, v);
            let combo_found = response.contains(&combo);

            if key_found {
                found_keys.push(k.clone());
            }
            if value_found {
                found_values.push(v.clone());
            }
            if combo_found {
                found_combos.push(combo);
            }
        }

        println!("\n📄 Résultats de la recherche dans le contenu :");

        println!("\n✔️ Clés trouvées :");
        if found_keys.is_empty() {
            println!("  Aucune");
        } else {
            for k in found_keys {
                println!("  {}", k);
            }
        }

        println!("\n✔️ Valeurs trouvées :");
        let mut any_value_found = false;
        for (k, v) in &query_params {
            if !v.is_empty() && response.contains(v) {
                println!("  {} ({})", v, k);
                any_value_found = true;
            }
        }
        if !any_value_found {
            println!("  Aucune");
        }

        println!("\n✔️ Combos clé=valeur trouvés :");
        if found_combos.is_empty() {
            println!("  Aucun");
        } else {
            for combo in found_combos {
                println!("  {}", combo);
            }
        }
    }
}
