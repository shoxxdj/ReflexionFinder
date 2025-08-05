# 🔎 ReflexionFinder

**ReflexionFinder** est un outil en ligne de commande écrit en Rust qui :

- Extrait les paramètres d'une URL.
- Recherche leur présence dans le contenu de la page.
- Fait du fuzzing sur les valeurs.
- Supporte proxy, configuration TLS, User-Agent, etc.

---

## 🚀 Installation

### 1. Cloner le dépôt

```bash
git clone <url_du_repo>
cd ReflexionFinder
```

### 2. Compiler

```bash
cargo build --release
```

Le binaire sera dans target/release/ReflexionFinder.

## 🧪 Utilisation

```bash
ReflexionFinder [OPTIONS] [URL]
```

Si l’URL n’est pas fournie en argument, elle peut être saisie via l'entrée standard (stdin).

### 📌 Options

```bash
Extrait les paramètres d'une URL et peut rechercher leur présence dans le contenu

Usage: ReflexionFinder [OPTIONS] [url]

Arguments:
  [url]  L'URL à analyser

Options:
  -p, --proxy <URL>      Utiliser un proxy pour les requêtes HTTP ex: http://127.0.0.1:8080
  -k, --insecure         Ignorer les erreurs de certificats TLS (⚠️ non sécurisé)
  -s, --search           Active la recherche des clés et valeurs dans la page
  -f, --fuzz             Active le fuzzing des valeurs (remplacement par des chaînes aléatoires)
  -u, --user-agent <UA>  Définit le header User-Agent à utiliser dans les requêtes
  -v, --verbose          Mode verbeux
  -h, --help             Print help
  -V, --version          Print version

```

### Exemples

Extraire les paramètres d’une URL

```bash
ReflexionFinder "https://example.com/?q=rust&lang=fr"
```

Rechercher les paramètres dans le contenu de la page

```bash
ReflexionFinder "https://example.com/?q=rust&lang=fr" --search
```

Fuzzer les valeurs et tester leur réflexion

```bash
ReflexionFinder "https://example.com/?q=test" --fuzz
```

Utiliser un proxy avec bypass TLS et User-Agent personnalisé

```bash
ReflexionFinder --proxy http://127.0.0.1:8080 -k -U "Mozilla/5.0 CustomBot" --search
```

Lire l’URL depuis l’entrée standard

```bash
echo "https://example.com/?a=1&b=2" | ReflexionFinder --search
```
