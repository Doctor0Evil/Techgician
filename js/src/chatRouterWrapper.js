// Minimal mirror of Rust's intent classification for client-side UX.
// This does NOT replace on-chain or Rust-side validation.
function inferIntent(text) {
  const t = text.toLowerCase();
  if (t.includes("wbgt") || t.includes("wet bulb") || t.includes("wet-bulb") || t.includes("heat stress") || t.includes("thermal comfort") || t.includes("thermal resilience") || t.includes("air-globe") || t.includes("airglobe")) return "AirGlobeWBGT";
  if (t.includes("cyboquatic") || t.includes("microfluidic")) return "CyboquaticCooling";
  if (t.includes("cybocindric") || t.includes("sofc") || t.includes("reactor")) return "CybocindricReactor";
  if (t.includes("econet") || t.includes("biophysical-blockchain")) return "EcoNetTokenomics";
  if (t.includes("biodegradab") || t.includes("polymer") || t.includes("plastic")) return "BiodegradableMaterials";
  return "Unknown";
}

function intentToBaseQuery(intent) {
  switch (intent) {
    case "CyboquaticCooling":
      return "cyboquatic microfluidic wbgt exergy language:rust language:cpp";
    case "CybocindricReactor":
      return "cybocindric sofc exergy map reactor control language:rust language:cpp";
    case "AirGlobeWBGT":
      return "airglobe wbgt safety control econet language:rust language:js";
    case "EcoNetTokenomics":
      return "econet biophysical blockchain mint burn rollup language:rust language:js";
    case "BiodegradableMaterials":
      return "biodegradable polymer kinetics diffusion ml design language:rust";
    default:
      return "depin eco net exergy wbgt language:rust language:js language:cpp";
  }
}

function intentToBuildSpec(intent) {
  const edition = "2021";
  const coreDeps = ["serde", "tokio", "sha2", "time", "reqwest"];
  switch (intent) {
    case "CyboquaticCooling":
      return { crateName: "cyboquatic_cooling", edition, coreDeps, needsCppFfi: true, needsJsBinding: false };
    case "CybocindricReactor":
      return { crateName: "cybocindric_reactor", edition, coreDeps, needsCppFfi: true, needsJsBinding: false };
    case "AirGlobeWBGT":
      return { crateName: "airglobe_wbgt", edition, coreDeps, needsCppFfi: false, needsJsBinding: true };
    case "EcoNetTokenomics":
      return { crateName: "econet_tokenomics", edition, coreDeps, needsCppFfi: false, needsJsBinding: true };
    case "BiodegradableMaterials":
      return { crateName: "biodegradable_materials", edition, coreDeps, needsCppFfi: false, needsJsBinding: false };
    default:
      return { crateName: "eco_net_unknown", edition, coreDeps, needsCppFfi: true, needsJsBinding: true };
  }
}

function buildGithubSearchUrl(query, orgFilters = ["Doctor0Evil", "Techgician"]) {
  const q = `${query} ${orgFilters.map(o => `user:${o}`).join(" ")}`.trim();
  const encoded = encodeURIComponent(q);
  return `https://github.com/search?q=${encoded}&type=code`;
}

/**
 * Turn a chat text into a structured routing object with a clickable GitHub URL and build-spec.
 */
export function routeChatToGithub(chatText) {
  const intent = inferIntent(chatText);
  const baseQuery = intentToBaseQuery(intent);
  const githubUrl = buildGithubSearchUrl(baseQuery);
  const buildSpec = intentToBuildSpec(intent);
  return {
    intent,
    githubUrl,
    tags: [
      `intent::${intent}`,
      "ecosystem::Techgician",
      "router::v1"
    ],
    buildSpec
  };
}

// Example integration hook for a web UI.
export function createGithubLinkElement(chatText, documentRef = document) {
  const route = routeChatToGithub(chatText);
  const a = documentRef.createElement("a");
  a.href = route.githubUrl;
  a.target = "_blank";
  a.rel = "noopener noreferrer";
  a.textContent = `Search Techgician code for ${route.intent}`;
  return { element: a, route };
}
