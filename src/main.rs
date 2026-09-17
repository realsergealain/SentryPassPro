#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::process;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const HTML_PAGE: &str = include_str!("index.html");

/// Dictionnaire cryptographique Diceware de mots propres, mémorables et sans ambiguïté
const WORDLIST: &[&str] = &[
    "abysse", "acier", "aimant", "albatros", "alliance", "ambre", "amiral", "ancre", "anneau",
    "antenne", "arcade", "archipel", "argent", "armure", "arsenal", "artiste", "astral", "atome",
    "aurore", "avatar", "azur", "balise", "bambou", "banniere", "barrage", "baryum", "bastion",
    "boussole", "bouclier", "braise", "brillant", "bronze", "brume", "cadence", "calibre",
    "calypso", "camelia", "canon", "capitaine", "capsule", "caravelle", "carbone", "cascade",
    "casque", "catalyseur", "cavalerie", "celeste", "cercle", "cerveau", "chaman", "chimere",
    "chrome", "circuit", "citadelle", "clairon", "cobalt", "colibri", "colosse", "comete",
    "compas", "concorde", "constellation", "corail", "corsaire", "cratere", "cristal", "cuirasse",
    "cyclone", "dague", "delta", "diamant", "diapason", "dinosaure", "disque", "drakkar", "drone",
    "dynamite", "ebene", "eclair", "eclipse", "effigie", "elixir", "emeraude", "enigme", "epave",
    "epilogue", "epopee", "erable", "escrime", "espion", "etincelle", "etoile", "falaise", "faucon",
    "ferveur", "fibre", "filtre", "flambeau", "flamme", "fleche", "flotte", "fluide", "flux",
    "foret", "forge", "foudre", "fournaise", "fregate", "frisson", "fusion", "galaxie", "galion",
    "gardien", "gemme", "geyser", "glacier", "glaive", "glyphe", "granite", "gravite", "griffon",
    "guerrier", "guide", "harmonie", "harpon", "havre", "helium", "heraut", "heron", "horizon",
    "horloge", "hublot", "hydre", "hyperbole", "iceberg", "icone", "iguane", "illusion", "impact",
    "impulse", "indigo", "infini", "insigne", "instinct", "iris", "isotope", "jaguar", "javelot",
    "jonquille", "jubile", "jupiter", "kayak", "kilo", "kraken", "krypton", "labyrinthe", "lagune",
    "lampe", "lance", "laser", "lave", "legende", "lentille", "leopard", "levier", "libellule",
    "lichen", "ligne", "limier", "lingot", "lion", "liquide", "losange", "luciole", "luminaire",
    "lumiere", "lunaire", "lynx", "magma", "magnat", "magnolia", "malachite", "mammouth", "manoir",
    "manteau", "marbre", "maree", "marteau", "masque", "matrice", "maxime", "mecanique", "medaille",
    "melodie", "mercure", "meridien", "mesure", "meteor", "metaphore", "mica", "mirage", "miroir",
    "missile", "module", "molecule", "monolithe", "montagne", "mosaique", "moteur", "mouette",
    "moulin", "mousquet", "muraille", "mutant", "mystere", "nacre", "nacelle", "navire", "nebuleuse",
    "nectar", "nemesis", "neon", "neptune", "nerveux", "neutre", "neutron", "nexus", "nickel",
    "nimbus", "nomade", "nouvelle", "noyau", "nuage", "oasis", "obelisque", "objet", "obsidienne",
    "ocean", "octane", "odyssee", "ogive", "olive", "olympe", "ombre", "omega", "onde", "onyx",
    "opale", "optique", "orbite", "orque", "orion", "ouragan", "outillage", "oxygene", "ozone",
    "pacte", "paladin", "palmier", "panache", "panthere", "parade", "parcours", "parchemin",
    "parfum", "passerelle", "pastel", "patrouille", "pendule", "pelican", "perle", "petrole",
    "phalange", "pharaon", "phare", "phoenix", "photon", "pilier", "pilote", "pinnacle", "pionnier",
    "pirogue", "piston", "planete", "plasma", "platine", "plongeur", "plume", "polarite", "polygone",
    "pont", "portail", "posture", "poudre", "prisme", "progres", "pulsar", "pyramide", "quai",
    "quantum", "quartz", "quasar", "quete", "racine", "radar", "radeau", "radium", "rafale",
    "raie", "rampe", "rapace", "rayon", "recif", "relais", "rempart", "renard", "repere", "reseau",
    "resine", "ressort", "ricochet", "rivage", "riviere", "robot", "roche", "roquette", "roseau",
    "rouage", "rubis", "ruche", "rythme", "sable", "sablier", "sabre", "safari", "saphir",
    "satellite", "saturne", "scarabee", "sculpture", "seisme", "semence", "sentinelle", "serpent",
    "silice", "sillage", "siphon", "sirene", "solaire", "sommet", "sonde", "sonate", "souris",
    "spectre", "sphere", "spire", "spirale", "stature", "steppe", "strate", "stride", "sublime",
    "summum", "superbe", "surtension", "synapse", "synergie", "tableau", "talisman", "tambour",
    "tangente", "taureau", "tectonique", "temple", "tempo", "tenor", "tension", "terminal",
    "terrasse", "thermique", "tigre", "timon", "titane", "tonnerre", "torpille", "torrent",
    "totem", "tourbillon", "trace", "tracteur", "traineau", "trajet", "transept", "trapeze",
    "treuil", "tribord", "trident", "triptyque", "trompette", "trophee", "troupe", "tsunami",
    "tube", "tungstene", "turbine", "turbo", "tuyau", "typhon", "ultime", "ultrason", "uniforme",
    "unite", "uranium", "usine", "utopie", "vague", "vaisseau", "valence", "vallee", "vapeur",
    "vecteur", "vedette", "velours", "vent", "ventouse", "verrou", "vestige", "vibration",
    "vigie", "vinaigre", "vitesse", "volcan", "volt", "voltage", "volume", "vortex", "voyage",
    "vulcan", "wagon", "zenith", "zephyr", "zero", "zinc", "zircon", "zodiaque", "zone",
    // Compléments internationaux
    "alpha", "apex", "bravo", "cipher", "delta", "echo", "falcon", "gamma", "hazard", "iron",
    "kilo", "lunar", "matrix", "nexus", "omega", "pulse", "quantum", "radar", "shadow", "titan",
    "ultra", "vector", "vortex", "zenith", "zero"
];

/// Mots de passe faibles courants pour le moteur d'audit
const COMMON_WEAK_PASSWORDS: &[&str] = &[
    "password", "123456", "12345678", "123456789", "12345", "1234", "qwerty", "azerty",
    "admin", "welcome", "letmein", "motdepasse", "bienvenue", "root", "secret", "master",
    "monkey", "dragon", "football", "iloveyou", "superman", "trustno1", "soleil", "chouchou",
    "cheval", "orange", "marseille", "paris", "internet", "bonjour", "test", "passer"
];

/// Générateur pseudo-aléatoire Xoshiro256++ en pur Rust
struct Rng {
    s: [u64; 4],
}

impl Rng {
    fn new() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(123456789);

        let pid = process::id() as u128;
        let heap_dummy = Box::new(42);
        let aslr_addr = (&*heap_dummy as *const i32 as usize) as u128;

        let mut seed = ((nanos ^ (pid << 32)) ^ (aslr_addr << 16)) as u64;

        let mut next_splitmix = || {
            seed = seed.wrapping_add(0x9E3779B97F4A7C15);
            let mut z = seed;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
            z ^ (z >> 31)
        };

        Self {
            s: [
                next_splitmix(),
                next_splitmix(),
                next_splitmix(),
                next_splitmix(),
            ],
        }
    }

    fn next_u64(&mut self) -> u64 {
        let result = (self.s[0].wrapping_add(self.s[3]))
            .rotate_left(23)
            .wrapping_add(self.s[0]);

        let t = self.s[1] << 17;

        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];

        self.s[2] ^= t;
        self.s[3] = self.s[3].rotate_left(45);

        result
    }

    fn gen_range(&mut self, max: usize) -> usize {
        if max == 0 {
            return 0;
        }
        (self.next_u64() % (max as u64)) as usize
    }

    fn shuffle<T>(&mut self, slice: &mut [T]) {
        let len = slice.len();
        if len <= 1 {
            return;
        }
        for i in (1..len).rev() {
            let j = self.gen_range(i + 1);
            slice.swap(i, j);
        }
    }
}

#[derive(Debug, Clone)]
pub struct PasswordConfig {
    pub length: usize,
    pub use_uppercase: bool,
    pub use_lowercase: bool,
    pub use_numbers: bool,
    pub use_symbols: bool,
    pub avoid_ambiguous: bool,
}

pub struct PasswordGenerator;

impl PasswordGenerator {
    const UPPERCASE: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    const LOWERCASE: &'static str = "abcdefghijklmnopqrstuvwxyz";
    const NUMBERS: &'static str = "0123456789";
    const SYMBOLS: &'static str = "!@#$%^&*()-_=+[]{}|;:,.<>?";
    const AMBIGUOUS: &'static str = "0O1Il|`'";

    pub fn generate(config: &PasswordConfig) -> Result<String, &'static str> {
        if config.length == 0 {
            return Err("La longueur doit être supérieure à 0.");
        }

        let mut guaranteed_chars = Vec::new();
        let mut full_pool = String::new();
        let mut rng = Rng::new();

        let mut add_category = |chars: &str, enabled: bool| {
            if !enabled {
                return;
            }
            let filtered: String = chars
                .chars()
                .filter(|c| !config.avoid_ambiguous || !Self::AMBIGUOUS.contains(*c))
                .collect();

            if !filtered.is_empty() {
                let chars_vec: Vec<char> = filtered.chars().collect();
                let idx = rng.gen_range(chars_vec.len());
                guaranteed_chars.push(chars_vec[idx]);
                full_pool.push_str(&filtered);
            }
        };

        add_category(Self::UPPERCASE, config.use_uppercase);
        add_category(Self::LOWERCASE, config.use_lowercase);
        add_category(Self::NUMBERS, config.use_numbers);
        add_category(Self::SYMBOLS, config.use_symbols);

        if full_pool.is_empty() {
            return Err("Au moins une catégorie de caractères doit être activée !");
        }

        if config.length < guaranteed_chars.len() {
            return Err("La longueur est trop petite pour inclure toutes les catégories demandées.");
        }

        let mut password_chars = guaranteed_chars;
        let pool_vec: Vec<char> = full_pool.chars().collect();

        while password_chars.len() < config.length {
            let idx = rng.gen_range(pool_vec.len());
            password_chars.push(pool_vec[idx]);
        }

        rng.shuffle(&mut password_chars);
        Ok(password_chars.into_iter().collect())
    }

    pub fn generate_passphrase(
        words_count: usize,
        separator: &str,
        capitalize: bool,
        include_number: bool,
    ) -> (String, f64) {
        let count = words_count.clamp(3, 10);
        let mut rng = Rng::new();
        let mut selected_words = Vec::with_capacity(count);

        for _ in 0..count {
            let idx = rng.gen_range(WORDLIST.len());
            let word = WORDLIST[idx];
            if capitalize {
                let mut c = word.chars();
                let capitalized = match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                };
                selected_words.push(capitalized);
            } else {
                selected_words.push(word.to_string());
            }
        }

        if include_number {
            let num = rng.gen_range(100);
            let insert_pos = rng.gen_range(selected_words.len());
            selected_words[insert_pos].push_str(&format!("{}", num));
        }

        let phrase = selected_words.join(separator);
        // Calcul d'entropie Diceware : count * log2(pool_size) + bonus si nombre
        let pool_entropy = (count as f64) * (WORDLIST.len() as f64).log2();
        let entropy = if include_number { pool_entropy + 6.64 } else { pool_entropy };

        (phrase, entropy)
    }

    pub fn generate_pin(length: usize) -> (String, f64) {
        let len = length.clamp(4, 16);
        let mut rng = Rng::new();
        let digits: Vec<char> = Self::NUMBERS.chars().collect();

        let mut pin = String::with_capacity(len);
        let mut prev_digit: Option<char> = None;
        let mut repeat_count = 0;

        while pin.len() < len {
            let d = digits[rng.gen_range(digits.len())];
            if let Some(prev) = prev_digit {
                if prev == d {
                    repeat_count += 1;
                    if repeat_count >= 2 {
                        continue; // Évite 3 chiffres identiques de suite
                    }
                } else {
                    repeat_count = 0;
                }
            }
            prev_digit = Some(d);
            pin.push(d);
        }

        let entropy = (len as f64) * (10.0_f64).log2();
        (pin, entropy)
    }

    pub fn generate_token(token_type: &str, bytes_len: usize) -> (String, f64) {
        let mut rng = Rng::new();
        let size = bytes_len.clamp(16, 64);
        let mut raw = vec![0u8; size];
        for b in raw.iter_mut() {
            *b = (rng.next_u64() & 0xFF) as u8;
        }

        match token_type {
            "uuid" => {
                // RFC 4122 version 4 UUID
                raw[6] = (raw[6] & 0x0F) | 0x40; // Version 4
                raw[8] = (raw[8] & 0x3F) | 0x80; // Variant 1
                let hex_str = raw.iter().map(|b| format!("{:02x}", b)).collect::<Vec<_>>();
                let uuid = format!(
                    "{}{}{}{}-{}{}-{}{}-{}{}-{}{}{}{}{}{}",
                    hex_str[0], hex_str[1], hex_str[2], hex_str[3],
                    hex_str[4], hex_str[5],
                    hex_str[6], hex_str[7],
                    hex_str[8], hex_str[9],
                    hex_str[10], hex_str[11], hex_str[12], hex_str[13], hex_str[14], hex_str[15]
                );
                (uuid, 122.0)
            }
            "base64" => {
                const B64_CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
                let mut out = String::new();
                for chunk in raw.chunks(3) {
                    let b0 = chunk[0] as usize;
                    let b1 = if chunk.len() > 1 { chunk[1] as usize } else { 0 };
                    let b2 = if chunk.len() > 2 { chunk[2] as usize } else { 0 };

                    out.push(B64_CHARS[(b0 >> 2) & 0x3F] as char);
                    out.push(B64_CHARS[((b0 & 0x03) << 4) | ((b1 >> 4) & 0x0F)] as char);
                    if chunk.len() > 1 {
                        out.push(B64_CHARS[((b1 & 0x0F) << 2) | ((b2 >> 6) & 0x03)] as char);
                    }
                    if chunk.len() > 2 {
                        out.push(B64_CHARS[b2 & 0x3F] as char);
                    }
                }
                let entropy = (raw.len() as f64) * 8.0;
                (out, entropy)
            }
            _ => {
                // Hex
                let hex_str = raw.iter().map(|b| format!("{:02x}", b)).collect::<String>();
                let entropy = (raw.len() as f64) * 8.0;
                (hex_str, entropy)
            }
        }
    }

    pub fn calculate_entropy(config: &PasswordConfig) -> (f64, &'static str) {
        let mut pool_size = 0usize;
        if config.use_uppercase {
            pool_size += if config.avoid_ambiguous { 24 } else { 26 };
        }
        if config.use_lowercase {
            pool_size += if config.avoid_ambiguous { 24 } else { 26 };
        }
        if config.use_numbers {
            pool_size += if config.avoid_ambiguous { 8 } else { 10 };
        }
        if config.use_symbols {
            pool_size += if config.avoid_ambiguous { 24 } else { 27 };
        }

        if pool_size == 0 || config.length == 0 {
            return (0.0, "Nulle");
        }

        let entropy = (config.length as f64) * (pool_size as f64).log2();
        let rating = match entropy {
            e if e < 28.0 => "Très faible (Critique)",
            e if e < 36.0 => "Faible (Vulnérable)",
            e if e < 60.0 => "Moyenne (Acceptable)",
            e if e < 80.0 => "Forte (Recommandée)",
            _ => "Excellente (Blindée)",
        };

        (entropy, rating)
    }

    pub fn format_duration(seconds: f64) -> String {
        if seconds <= 0.0 || seconds < 1.0 {
            return "< 1 sec".to_string();
        }
        if seconds < 60.0 {
            return format!("{:.0} s", seconds);
        }
        let minutes = seconds / 60.0;
        if minutes < 60.0 {
            return format!("{:.0} min", minutes);
        }
        let hours = minutes / 60.0;
        if hours < 24.0 {
            return format!("{:.0} h", hours);
        }
        let days = hours / 24.0;
        if days < 365.25 {
            return format!("{:.0} jours", days);
        }
        let years = days / 365.25;
        if years < 1_000.0 {
            return format!("{:.0} ans", years);
        }
        if years < 1_000_000.0 {
            return format!("{:.0} millénaires", years / 1_000.0);
        }
        if years < 1_000_000_000.0 {
            return format!("{:.0} millions d'ans", years / 1_000_000.0);
        }
        format!("{:.0} Mds d'ans", years / 1_000_000_000.0)
    }

    pub fn estimate_crack_times(entropy: f64) -> (String, String, String) {
        if entropy <= 0.0 {
            return (
                "Instantané".to_string(),
                "Instantané".to_string(),
                "Instantané".to_string(),
            );
        }

        let log2_combs = (entropy - 1.0).max(0.0);
        let log10_combs = log2_combs * std::f64::consts::LOG10_2;

        let log10_gpu = log10_combs - 11.0;
        let gpu_time = if log10_gpu > 15.0 {
            format!("> 10^{:.0} ans", (log10_gpu - 7.5).min(99.0))
        } else {
            Self::format_duration(10.0_f64.powf(log10_gpu))
        };

        let log10_online = log10_combs - 3.0;
        let online_fast = if log10_online > 15.0 {
            format!("> 10^{:.0} ans", (log10_online - 7.5).min(99.0))
        } else {
            Self::format_duration(10.0_f64.powf(log10_online))
        };

        let log10_throttled = log10_combs + 1.1;
        let online_throttled = if log10_throttled > 15.0 {
            format!("> 10^{:.0} ans", (log10_throttled - 7.5).min(99.0))
        } else {
            Self::format_duration(10.0_f64.powf(log10_throttled))
        };

        (gpu_time, online_fast, online_throttled)
    }

    pub fn audit(pwd: &str) -> (f64, i32, &'static str, Vec<String>, bool, bool, bool, bool, bool, (String, String, String)) {
        if pwd.is_empty() {
            return (
                0.0,
                0,
                "Vide",
                vec!["Entrez un mot de passe pour lancer l'audit en direct.".to_string()],
                false, false, false, false, false,
                ("Instantané".to_string(), "Instantané".to_string(), "Instantané".to_string()),
            );
        }

        let len = pwd.chars().count();
        let has_lower = pwd.chars().any(|c| c.is_lowercase());
        let has_upper = pwd.chars().any(|c| c.is_uppercase());
        let has_digit = pwd.chars().any(|c| c.is_ascii_digit());
        let has_symbol = pwd.chars().any(|c| !c.is_alphanumeric());

        let mut pool_size = 0usize;
        if has_lower { pool_size += 26; }
        if has_upper { pool_size += 26; }
        if has_digit { pool_size += 10; }
        if has_symbol { pool_size += 32; }

        let mut entropy = (len as f64) * (pool_size as f64).max(1.0).log2();

        let mut suggestions = Vec::new();
        let lower_pwd = pwd.to_lowercase();

        // 1. Vérification dictionnaire de fuites
        let mut is_weak_dictionary = false;
        for weak in COMMON_WEAK_PASSWORDS {
            if lower_pwd.contains(weak) {
                is_weak_dictionary = true;
                suggestions.push(format!("⚠️ Contient un motif faible courant ('{}').", weak));
                entropy = (entropy * 0.35).min(25.0);
                break;
            }
        }

        // 2. Répétitions consécutives (ex: aaa, 111)
        let mut max_repeat = 1;
        let mut current_repeat = 1;
        let mut prev_char = None;
        for c in pwd.chars() {
            if Some(c) == prev_char {
                current_repeat += 1;
                max_repeat = max_repeat.max(current_repeat);
            } else {
                current_repeat = 1;
            }
            prev_char = Some(c);
        }
        if max_repeat >= 3 {
            suggestions.push(format!("⚠️ Caractères répétés {} fois consécutivement.", max_repeat));
            entropy = (entropy * 0.8).max(10.0);
        }

        // 3. Longueur
        if len < 8 {
            suggestions.push("🚨 Longueur critique (< 8 caractères) : très vulnérable au brute-force.".to_string());
        } else if len < 12 {
            suggestions.push("💡 Conseil : Visez au moins 14 caractères pour une sécurité durable.".to_string());
        }

        // 4. Diversité
        let mut variety_count = 0;
        if has_lower { variety_count += 1; }
        if has_upper { variety_count += 1; }
        if has_digit { variety_count += 1; }
        if has_symbol { variety_count += 1; }

        if variety_count < 3 {
            suggestions.push("💡 Diversifiez les caractères (mélangez majuscules, chiffres et symboles).".to_string());
        }

        if suggestions.is_empty() {
            suggestions.push("✅ Excellent mot de passe : haute complexité et aucun motif vulnérable détecté.".to_string());
        }

        // Calcul score 0 à 100
        let mut score = (entropy / 85.0 * 100.0) as i32;
        if is_weak_dictionary {
            score = score.min(20);
        }
        if len < 8 {
            score = score.min(15);
        }
        score = score.clamp(0, 100);

        let rating = match score {
            s if s < 30 => "Critique",
            s if s < 55 => "Faible",
            s if s < 75 => "Moyen",
            s if s < 90 => "Robuste",
            _ => "Invulnérable",
        };

        let crack_times = Self::estimate_crack_times(entropy);
        let no_common_pattern = !is_weak_dictionary && max_repeat < 3;

        (
            entropy,
            score,
            rating,
            suggestions,
            len >= 12,
            has_upper && has_lower,
            has_digit,
            has_symbol,
            no_common_pattern,
            crack_times,
        )
    }
}

// Parseur minimal de requête HTTP
fn parse_query_param<'a>(query: &'a str, key: &str) -> Option<&'a str> {
    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        if let (Some(k), Some(v)) = (parts.next(), parts.next()) {
            if k == key {
                return Some(v);
            }
        }
    }
    None
}

fn url_decode(s: &str) -> String {
    let mut out = String::new();
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '%' {
            let h1 = chars.next().unwrap_or('0');
            let h2 = chars.next().unwrap_or('0');
            if let Ok(byte) = u8::from_str_radix(&format!("{}{}", h1, h2), 16) {
                out.push(byte as char);
            }
        } else if c == '+' {
            out.push(' ');
        } else {
            out.push(c);
        }
    }
    out
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0u8; 8192];
    let bytes_read = match stream.read(&mut buffer) {
        Ok(n) => n,
        Err(_) => return,
    };

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
    let first_line = match request.lines().next() {
        Some(line) => line,
        None => return,
    };

    let mut parts = first_line.split_whitespace();
    let _method = parts.next().unwrap_or("");
    let full_path = parts.next().unwrap_or("/");

    let (path, query) = match full_path.find('?') {
        Some(idx) => (&full_path[..idx], &full_path[idx + 1..]),
        None => (full_path, ""),
    };

    // Servir la page HTML principale
    if path == "/" || path == "/index.html" {
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            HTML_PAGE.len(),
            HTML_PAGE
        );
        let _ = stream.write_all(response.as_bytes());
        return;
    }

    // 1. API Générateur classique de caractères
    if path == "/api/generate" {
        let length: usize = parse_query_param(query, "length")
            .and_then(|v| v.parse().ok())
            .unwrap_or(16);
        let upper: bool = parse_query_param(query, "upper").map(|v| v == "true").unwrap_or(true);
        let lower: bool = parse_query_param(query, "lower").map(|v| v == "true").unwrap_or(true);
        let numbers: bool = parse_query_param(query, "numbers").map(|v| v == "true").unwrap_or(true);
        let symbols: bool = parse_query_param(query, "symbols").map(|v| v == "true").unwrap_or(true);
        let avoid_ambiguous: bool = parse_query_param(query, "avoid_ambiguous").map(|v| v == "true").unwrap_or(false);

        let config = PasswordConfig {
            length,
            use_uppercase: upper,
            use_lowercase: lower,
            use_numbers: numbers,
            use_symbols: symbols,
            avoid_ambiguous,
        };

        let start = Instant::now();
        let (entropy, rating) = PasswordGenerator::calculate_entropy(&config);
        let (gpu_time, online_fast, online_throttled) = PasswordGenerator::estimate_crack_times(entropy);

        let body = match PasswordGenerator::generate(&config) {
            Ok(pwd) => {
                let duration = start.elapsed().as_micros();
                format!(
                    r#"{{"password":"{}","entropy":{:.2},"rating":"{}","crack_gpu":"{}","crack_online":"{}","crack_throttled":"{}","duration_micros":{}}}"#,
                    pwd.replace('\\', "\\\\").replace('"', "\\\""),
                    entropy,
                    rating,
                    gpu_time,
                    online_fast,
                    online_throttled,
                    duration
                )
            }
            Err(err) => {
                format!(r#"{{"error":"{}"}}"#, err)
            }
        };

        send_json_response(&mut stream, &body);
        return;
    }

    // 2. API Générateur de Passphrase (Diceware)
    if path == "/api/passphrase" {
        let count: usize = parse_query_param(query, "count")
            .and_then(|v| v.parse().ok())
            .unwrap_or(4);
        let raw_sep = parse_query_param(query, "separator").unwrap_or("-");
        let sep = match raw_sep {
            "dash" => "-",
            "underscore" => "_",
            "dot" => ".",
            "space" => " ",
            s => s,
        };
        let capitalize = parse_query_param(query, "capitalize").map(|v| v == "true").unwrap_or(true);
        let num = parse_query_param(query, "number").map(|v| v == "true").unwrap_or(true);

        let start = Instant::now();
        let (phrase, entropy) = PasswordGenerator::generate_passphrase(count, sep, capitalize, num);
        let duration = start.elapsed().as_micros();
        let (gpu_time, online_fast, online_throttled) = PasswordGenerator::estimate_crack_times(entropy);

        let body = format!(
            r#"{{"passphrase":"{}","entropy":{:.2},"rating":"Blindée (Mnémonique)","crack_gpu":"{}","crack_online":"{}","crack_throttled":"{}","duration_micros":{}}}"#,
            phrase.replace('\\', "\\\\").replace('"', "\\\""),
            entropy,
            gpu_time,
            online_fast,
            online_throttled,
            duration
        );

        send_json_response(&mut stream, &body);
        return;
    }

    // 3. API Générateur de Token & Clé d'API
    if path == "/api/token" {
        let token_type = parse_query_param(query, "type").unwrap_or("hex");
        let bytes: usize = parse_query_param(query, "bytes")
            .and_then(|v| v.parse().ok())
            .unwrap_or(32);

        let start = Instant::now();
        let (token, entropy) = PasswordGenerator::generate_token(token_type, bytes);
        let duration = start.elapsed().as_micros();
        let (gpu_time, online_fast, online_throttled) = PasswordGenerator::estimate_crack_times(entropy);

        let body = format!(
            r#"{{"token":"{}","entropy":{:.2},"rating":"Cryptographique","crack_gpu":"{}","crack_online":"{}","crack_throttled":"{}","duration_micros":{}}}"#,
            token.replace('\\', "\\\\").replace('"', "\\\""),
            entropy,
            gpu_time,
            online_fast,
            online_throttled,
            duration
        );

        send_json_response(&mut stream, &body);
        return;
    }

    // 4. API Générateur de Code PIN
    if path == "/api/pin" {
        let length: usize = parse_query_param(query, "length")
            .and_then(|v| v.parse().ok())
            .unwrap_or(6);

        let start = Instant::now();
        let (pin, entropy) = PasswordGenerator::generate_pin(length);
        let duration = start.elapsed().as_micros();
        let (gpu_time, online_fast, online_throttled) = PasswordGenerator::estimate_crack_times(entropy);

        let body = format!(
            r#"{{"pin":"{}","entropy":{:.2},"rating":"Code PIN","crack_gpu":"{}","crack_online":"{}","crack_throttled":"{}","duration_micros":{}}}"#,
            pin,
            entropy,
            gpu_time,
            online_fast,
            online_throttled,
            duration
        );

        send_json_response(&mut stream, &body);
        return;
    }

    // 5. API Auditeur de Mot de Passe en Direct
    if path == "/api/audit" {
        let raw_pwd = parse_query_param(query, "pwd").unwrap_or("");
        let pwd = url_decode(raw_pwd);

        let (entropy, score, rating, suggestions, check_len, check_case, check_digit, check_symbol, check_patterns, crack_times) =
            PasswordGenerator::audit(&pwd);

        let suggestions_json = suggestions
            .into_iter()
            .map(|s| format!("\"{}\"", s.replace('"', "\\\"")))
            .collect::<Vec<_>>()
            .join(",");

        let body = format!(
            r#"{{"entropy":{:.2},"score":{},"rating":"{}","suggestions":[{}],"checks":{{"length":{},"case":{},"digits":{},"symbols":{},"patterns":{}}},"crack_gpu":"{}","crack_online":"{}","crack_throttled":"{}"}}"#,
            entropy,
            score,
            rating,
            suggestions_json,
            check_len,
            check_case,
            check_digit,
            check_symbol,
            check_patterns,
            crack_times.0,
            crack_times.1,
            crack_times.2
        );

        send_json_response(&mut stream, &body);
        return;
    }

    // 6. API Génération en lot (Batch)
    if path == "/api/batch" {
        let length: usize = parse_query_param(query, "length")
            .and_then(|v| v.parse().ok())
            .unwrap_or(16);
        let count: usize = parse_query_param(query, "count")
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);
        let upper = parse_query_param(query, "upper").map(|v| v == "true").unwrap_or(true);
        let lower = parse_query_param(query, "lower").map(|v| v == "true").unwrap_or(true);
        let numbers = parse_query_param(query, "numbers").map(|v| v == "true").unwrap_or(true);
        let symbols = parse_query_param(query, "symbols").map(|v| v == "true").unwrap_or(true);
        let avoid_ambiguous = parse_query_param(query, "avoid_ambiguous").map(|v| v == "true").unwrap_or(false);

        let config = PasswordConfig {
            length,
            use_uppercase: upper,
            use_lowercase: lower,
            use_numbers: numbers,
            use_symbols: symbols,
            avoid_ambiguous,
        };

        let mut list = Vec::new();
        for _ in 0..count {
            if let Ok(p) = PasswordGenerator::generate(&config) {
                list.push(format!("\"{}\"", p.replace('\\', "\\\\").replace('"', "\\\"")));
            }
        }

        let body = format!(r#"{{"passwords":[{}]}}"#, list.join(","));
        send_json_response(&mut stream, &body);
        return;
    }

    let not_found = "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
    let _ = stream.write_all(not_found.as_bytes());
}

fn send_json_response(stream: &mut TcpStream, body: &str) {
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json; charset=utf-8\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    let _ = stream.write_all(response.as_bytes());
}

fn main() {
    let listener = match TcpListener::bind("127.0.0.1:8080") {
        Ok(l) => l,
        Err(_) => match TcpListener::bind("127.0.0.1:0") {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Impossible de démarrer le serveur local : {}", e);
                return;
            }
        },
    };

    let local_addr = listener.local_addr().expect("Impossible de lire l'adresse locale");
    let url = format!("http://{}", local_addr);
    println!("=================================================================");
    println!("    🛡️  SENTRYPASS PRO - CYBERSECURITY & CRYPTO SENTINEL        ");
    println!("=================================================================");
    println!("  🚀 Serveur Web natif Rust actif sur : {}", url);
    println!("  💻 Application de bureau Windows prête.");
    println!("  ⚡ Fermez cette fenêtre ou faites Ctrl+C pour quitter.");
    println!("=================================================================");

    // Ouvre la fenêtre d'application de bureau Windows native
    open_desktop_window(&url);

    // Maintient le serveur actif sur le thread principal (ne se ferme JAMAIS tout seul !)
    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            std::thread::spawn(move || {
                handle_connection(stream);
            });
        }
    }
}

fn open_desktop_window(url: &str) {
    let edge_paths = [
        r"C:\Program Files (x86)\Microsoft\Edge\Application\msedge.exe",
        r"C:\Program Files\Microsoft\Edge\Application\msedge.exe",
    ];

    let mut launched = false;
    for path in edge_paths {
        if std::path::Path::new(path).exists() {
            let user_data = std::env::temp_dir().join("sentrypass_profile");
            let _ = process::Command::new(path)
                .args([
                    &format!("--app={}", url),
                    "--window-size=980,760",
                    &format!("--user-data-dir={}", user_data.to_string_lossy()),
                ])
                .spawn();
            launched = true;
            break;
        }
    }

    if !launched {
        let _ = process::Command::new("cmd")
            .args(["/C", "start", url])
            .spawn();
    }
}
