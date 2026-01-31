use std::collections::HashMap;
use urlencoding::encode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EcoIntent {
    CyboquaticCooling,
    CybocindricReactor,
    AirGlobeWBGT,
    EcoNetTokenomics,
    BiodegradableMaterials,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct RouteSpec {
    pub intent: EcoIntent,
    pub github_query: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct HexStampMeta {
    pub author_system: String,
    pub primary_addr: String,
    pub alt_addr: String,
    pub safe_addrs: Vec<String>,
    pub t_score: f32,
    pub p_score: f32,
    pub r_score: f32,
    pub c_score: f32,
    pub timestamp_utc: String,
}

#[derive(Debug, Clone)]
pub struct RoutedRequest {
    pub route: RouteSpec,
    pub hex_meta: Option<HexStampMeta>,
}

fn normalize(s: &str) -> String {
    s.to_lowercase()
}

/// Simple keyword → intent mapping; extendable via config later.
fn infer_intent(text: &str) -> EcoIntent {
    let t = normalize(text);
    if t.contains("cyboquatic") || t.contains("microfluidic") {
        EcoIntent::CyboquaticCooling
    } else if t.contains("cybocindric") || t.contains("sofc") || t.contains("reactor") {
        EcoIntent::CybocindricReactor
    } else if t.contains("air-globe") || t.contains("airglobe") || t.contains("wbgt") {
        EcoIntent::AirGlobeWBGT
    } else if t.contains("econet") || t.contains("biophysical-blockchain") {
        EcoIntent::EcoNetTokenomics
    } else if t.contains("biodegradab") || t.contains("polymer") || t.contains("plastic") {
        EcoIntent::BiodegradableMaterials
    } else {
        EcoIntent::Unknown
    }
}

/// Map intent → base GitHub search query.
fn intent_to_base_query(intent: &EcoIntent) -> &'static str {
    match intent {
        EcoIntent::CyboquaticCooling => "cyboquatic microfluidic wbgt exergy language:rust language:cpp",
        EcoIntent::CybocindricReactor => "cybocindric sofc exergy map reactor control language:rust language:cpp",
        EcoIntent::AirGlobeWBGT => "airglobe wbgt safety control econet language:rust language:js",
        EcoIntent::EcoNetTokenomics => "econet biophysical blockchain mint burn rollup language:rust language:js",
        EcoIntent::BiodegradableMaterials => "biodegradable polymer kinetics diffusion ml design language:rust",
        EcoIntent::Unknown => "depin eco net exergy wbgt language:rust language:js language:cpp",
    }
}

/// Build a GitHub code search URL.
fn build_github_search_url(query: &str, org_filters: &[&str]) -> String {
    let mut q = String::from(query);
    for org in org_filters {
        q.push(' ');
        q.push_str("user:");
        q.push_str(org);
    }
    let encoded = encode(&q);
    format!("https://github.com/search?q={}&type=code", encoded)
}

/// Parse the ALNDIDBostromStampV1 block from free text into structured metadata.
/// This is deliberately strict on keys and tolerant on spacing.
pub fn parse_hex_stamp_block(text: &str) -> Option<HexStampMeta> {
    let marker = "ALNDIDBostromStampV1";
    let idx = text.find(marker)?;
    let block = &text[idx..];

    let mut kv: HashMap<&str, &str> = HashMap::new();
    for token in block.split_whitespace() {
        if let Some((k, v)) = token.split_once('=') {
            kv.insert(k.trim(), v.trim());
        }
    }

    let author_system = kv.get("authorsystem")?.to_string();
    let primary_addr = kv.get("primarybostromaddr")?.to_string();
    let alt_addr = kv.get("altbostromaddr")?.to_string();
    let safe = kv
        .get("safeaddrs")
        .map(|s| s.split(',').map(|x| x.to_string()).collect())
        .unwrap_or_else(Vec::new);

    let t_score = kv
        .get("Tscore0to1")
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(0.0);
    let p_score = kv
        .get("Pscore0to1")
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(0.0);
    let r_score = kv
        .get("Rscore0to1")
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(0.0);
    let c_score = kv
        .get("Cscore0to1")
        .and_then(|s| s.parse::<f32>().ok())
        .unwrap_or(0.0);

    let timestamp_utc = kv
        .get("timestamputciso8601")
        .cloned()
        .unwrap_or("")
        .to_string();

    Some(HexStampMeta {
        author_system,
        primary_addr,
        alt_addr,
        safe_addrs: safe,
        t_score,
        p_score,
        r_score,
        c_score,
        timestamp_utc,
    })
}

/// Main router: from chat text (including optional hex-stamp) to a GitHub search URL plus structured intent.
pub fn route_request(chat_text: &str) -> RoutedRequest {
    let intent = infer_intent(chat_text);
    let base_q = intent_to_base_query(&intent);

    // You can parameterize this with Techgician / Doctor0Evil orgs.
    let org_filters = ["Doctor0Evil", "Techgician"];
    let url = build_github_search_url(base_q, &org_filters);

    let tags = vec![
        format!("intent::{:?}", intent),
        "ecosystem::Techgician".to_string(),
        "router::v1".to_string(),
    ];

    let route = RouteSpec {
        intent,
        github_query: url,
        tags,
    };

    let hex_meta = parse_hex_stamp_block(chat_text);

    RoutedRequest { route, hex_meta }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_stamp() {
        let s = "ALNDIDBostromStampV1 authorsystem=Perplexity-GPT-5.1-Techgician \
                 primarybostromaddr=bostrom18sd2u \
                 altbostromaddr=bostrom1ldg \
                 safeaddrs=zeta12x0u,0x519fC0eB \
                 Tscore0to1=0.83 Pscore0to1=0.80 Rscore0to1=0.14 Cscore0to1=0.52 \
                 timestamputciso8601=2026-01-31T21:54:00Z";

        let meta = parse_hex_stamp_block(s).expect("parse hex stamp");
        assert_eq!(meta.primary_addr, "bostrom18sd2u");
        assert!((meta.t_score - 0.83).abs() < 1e-6);
        assert_eq!(meta.safe_addrs.len(), 2);
    }

    #[test]
    fn test_route_intent_and_url() {
        let chat = "Help me design air-globes that keep WBGT safe while tying into EcoNet rewards.";
        let routed = route_request(chat);
        assert_eq!(routed.route.intent, EcoIntent::AirGlobeWBGT);
        assert!(routed.route.github_query.starts_with("https://github.com/search?q="));
    }
}
