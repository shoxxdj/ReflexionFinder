use clap::{Arg, ArgAction, Command};
use rand::{distributions::Alphanumeric, Rng};
use reqwest::blocking::get;
use std::io::{self, BufRead};
use url::{form_urlencoded, Url};

fn generate_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn main() {
    let matches = Command::new("ReflexionFinder")
        .version("1.0.0")
        .author("shoxxdj")
        .about("Extrait les paramètres d'une URL et peut rechercher leur présence dans le contenu")
        .arg(Arg::new("url").help("L'URL à analyser").index(1))
        .arg(
            Arg::new("proxy")
                .short('p')
                .long("proxy")
                .value_name("URL")
                .help("Utiliser un proxy pour les requêtes HTTP ex: http://127.0.0.1:8080")
                .num_args(1),
        )
        .arg(
            Arg::new("insecure")
                .short('k')
                .long("insecure")
                .help("Ignorer les erreurs de certificats TLS (⚠️ non sécurisé)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("search")
                .short('s')
                .long("search")
                .help("Active la recherche des clés et valeurs dans la page")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("fuzz")
                .short('f')
                .long("fuzz")
                .help("Active le fuzzing des valeurs (remplacement par des chaînes aléatoires)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("user_agent")
                .short('u')
                .long("user-agent")
                .value_name("UA")
                .help("Définit le header User-Agent à utiliser dans les requêtes")
                .num_args(1),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Mode verbeux")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let user_agent = matches
        .get_one::<String>("user_agent")
        .map(|s| s.to_owned())
        .unwrap_or_else(|| "ReflexionFinder/1.0".to_string());

    let do_search = *matches.get_one::<bool>("search").unwrap_or(&false);
    let do_fuzz = *matches.get_one::<bool>("fuzz").unwrap_or(&false);
    let do_verbose = *matches.get_one::<bool>("verbose").unwrap_or(&false);
    let proxy_option = matches.get_one::<String>("proxy");
    let ignore_cert = *matches.get_one::<bool>("insecure").unwrap_or(&false);

    let mut client_builder = reqwest::blocking::Client::builder().user_agent(user_agent);

    if ignore_cert {
        client_builder = client_builder.danger_accept_invalid_certs(true)
    }

    let client = match proxy_option {
        Some(proxy_url) => match reqwest::Proxy::all(proxy_url) {
            Ok(proxy) => match client_builder.proxy(proxy).build() {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("❌ Erreur création client HTTP avec proxy : {}", e);
                    return;
                }
            },
            Err(e) => {
                eprintln!("❌ Erreur proxy invalide : {}", e);
                return;
            }
        },
        None => match client_builder.build() {
            Ok(c) => c,
            Err(e) => {
                eprintln!("❌ Erreur création client HTTP : {}", e);
                return;
            }
        },
    };

    let urls: Vec<String> = if let Some(input_url) = matches.get_one::<String>("url") {
        vec![input_url.to_string()]
    } else {
        println!("📥 Lecture des URLs depuis l'entrée standard...");
        io::stdin()
            .lock()
            .lines()
            .filter_map(Result::ok)
            .filter(|line| !line.trim().is_empty())
            .collect()
    };

    for original_input_url in urls {
        let input_url = original_input_url.clone();

        println!("\n=====================");
        println!("🌐 Analyse de : {}", input_url);

        let parsed_url = match Url::parse(&input_url) {
            Ok(url) => url,
            Err(e) => {
                eprintln!("❌ Erreur de parsing : {} ({})", input_url, e);
                continue;
            }
        };

        let query_params: Vec<(String, String)> = parsed_url
            .query_pairs()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();

        println!("🔍 Paramètres trouvés :");
        for (k, v) in &query_params {
            println!("  {} = {}", k, v);
        }

        if do_search || do_fuzz {
            if do_search {
                println!("\n🌐 Requête vers la page...");

                let response = match client.get(&input_url).send() {
                    Ok(resp) => match resp.text() {
                        Ok(text) => text,
                        Err(e) => {
                            eprintln!("Erreur en lisant la réponse : {}", e);
                            continue;
                        }
                    },
                    Err(e) => {
                        eprintln!("Erreur lors de la requête HTTP : {}", e);
                        continue;
                    }
                };

                let mut found_keys = vec![];
                let mut found_combos = vec![];

                for (k, v) in &query_params {
                    if response.contains(k) {
                        found_keys.push(k.clone());
                    }

                    if !v.is_empty() {
                        let combo = format!("{}={}", k, v);
                        if response.contains(&combo) {
                            found_combos.push(combo);
                        }
                    }
                }

                println!("\n📄 Résultats de la recherche dans le contenu :");

                println!("\n✔️ Clés trouvées :");
                if found_keys.is_empty() {
                    println!("  Aucune");
                } else {
                    for k in &found_keys {
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
                    for combo in &found_combos {
                        println!("  {}", combo);
                    }
                }
            }

            if do_fuzz {
                println!("\n🧑‍🍳 Fuzzing des paramètres...");

                for (k, v) in &query_params {
                    if v.is_empty() {
                        println!("⚠️ Skipping clé '{}' avec valeur vide", k);
                        continue;
                    }

                    let fuzz_val = generate_random_string(12);
                    let mut new_params = query_params.clone();
                    for &mut (ref key, ref mut val) in &mut new_params {
                        if key == k {
                            *val = fuzz_val.clone();
                        }
                    }

                    let mut new_url = parsed_url.clone();
                    let query_string: String = form_urlencoded::Serializer::new(String::new())
                        .extend_pairs(&new_params)
                        .finish();
                    new_url.set_query(Some(&query_string));

                    println!("  🔄 Requête avec {} = {}", k, fuzz_val);
                    match client.get(new_url.as_str()).send() {
                        Ok(resp) => match resp.text() {
                            Ok(body) => {
                                if body.contains(&fuzz_val) {
                                    println!("✅ Valeur fuzz reflétée dans la réponse");
                                    println!("✅  {}", new_url);
                                } else if do_verbose {
                                    println!("❌ Valeur fuzz NON trouvée");
                                }
                            }
                            Err(e) => println!("⚠️ Erreur de lecture réponse : {}", e),
                        },
                        Err(e) => println!("⚠️ Erreur requête : {}", e),
                    }
                }

                println!("\n🧑‍🍳 Fuzzing terminé");
            }
        }
    }
}
