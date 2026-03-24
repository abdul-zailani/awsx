/// Matching logic untuk AWS profile ↔ kubectl context
pub fn normalize_name(name: &str) -> String {
    name.to_lowercase().replace('_', "-")
}

pub fn tokenize(name: &str) -> Vec<&str> {
    name.split(|c: char| c == '-' || c == '_' || c == '.')
        .filter(|s| !s.is_empty())
        .collect()
}

pub fn match_score(profile: &str, kube_ctx: &str) -> u32 {
    let pn = normalize_name(profile);
    let kn = normalize_name(kube_ctx);

    if pn == kn {
        return 100;
    }

    let p_tokens = tokenize(&pn);
    let k_tokens = tokenize(&kn);

    if p_tokens.is_empty() || k_tokens.is_empty() {
        return 0;
    }

    let matched: usize = p_tokens.iter().filter(|t| k_tokens.contains(t)).count();
    let total = p_tokens.len().max(k_tokens.len());

    let shorter = p_tokens.len().min(k_tokens.len());
    let shorter_matched = if p_tokens.len() <= k_tokens.len() {
        p_tokens.iter().filter(|t| k_tokens.contains(t)).count()
    } else {
        k_tokens.iter().filter(|t| p_tokens.contains(t)).count()
    };

    if shorter_matched < shorter {
        return 0;
    }

    ((matched as f64 / total as f64) * 100.0) as u32
}

pub fn find_kube_match(profile: &str, kube_contexts: &[String]) -> Option<String> {
    find_kube_match_threshold(profile, kube_contexts, 50)
}

pub fn find_kube_match_threshold(profile: &str, kube_contexts: &[String], threshold: u32) -> Option<String> {
    if kube_contexts.contains(&profile.to_string()) {
        return Some(profile.to_string());
    }
    let mut best: Option<(String, u32)> = None;
    for kctx in kube_contexts {
        let score = match_score(profile, kctx);
        if score >= threshold {
            if best.as_ref().map_or(true, |(_, s)| score > *s) {
                best = Some((kctx.clone(), score));
            }
        }
    }
    best.map(|(ctx, _)| ctx)
}

pub fn detect_environment(name: &str) -> Option<String> {
    let lower = name.to_lowercase();
    if lower.contains("prd") || lower.contains("prod") {
        Some("production".to_string())
    } else if lower.contains("stg") || lower.contains("staging") {
        Some("staging".to_string())
    } else if lower.contains("dev") {
        Some("development".to_string())
    } else {
        None
    }
}
