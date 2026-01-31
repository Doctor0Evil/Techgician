function inferIntent(text) {
  const t = text.toLowerCase();
  if (t.includes("cyboquatic") || t.includes("microfluidic")) return "CyboquaticCooling";
  if (t.includes("cybocindric") || t.includes("sofc") || t.includes("reactor")) return "CybocindricReactor";
  if (t.includes("air-globe") || t.includes("airglobe") || t.includes("wbgt")) return "AirGlobeWBGT";
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

function buildGithubSearchUrl(query, orgFilters = ["Doctor0Evil", "Techgician"]) {
  const q = `${query} ${orgFilters.map(o => `user:${o}`).join(" ")}`.trim();
  const encoded = encodeURIComponent(q);
  return `https://github.com/search?q=${encoded}&type=code`;
}

/**
 * Turn a chat text into a structured routing object with a clickable GitHub URL.
 */
export function routeChatToGithub(chatText) {
  const intent = inferIntent(chatText);
  const baseQuery = intentToBaseQuery(intent);
  const githubUrl = buildGithubSearchUrl(baseQuery);
  return {
    intent,
    githubUrl,
    tags: [
      `intent::${intent}`,
      "ecosystem::Techgician",
      "router::v1"
    ]
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
