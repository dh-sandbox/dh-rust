// debt.rs
//
// Intentionally bad Rust code for technical-debt demos:
// - Huge function w/ deep nesting + flags everywhere
// - Duplication (email/phone parsing + address parsing repeated patterns)
// - Magic numbers / stringly-typed state
// - Mixed concerns (parsing, validation, business logic, formatting, IO-ish logging)
// - Inconsistent error handling (Option, Result, panics, silent defaults)
// - Unclear naming / poor encapsulation

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct UserRecord {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub status: String,   // "active" | "disabled" | "pending" | ... (stringly typed)
    pub tier: String,     // "free" | "pro" | "enterprise" | ...
    pub tags: Vec<String>,
    pub meta: HashMap<String, String>,
}

#[derive(Clone, Debug)]
pub struct Order {
    pub order_id: i64,
    pub user_id: i64,
    pub amount_cents: i64,
    pub currency: String,
    pub state: String, // "new" | "paid" | "refunded" | "chargeback" | ...
}

fn log_warn(msg: &str) {
    // "Logging" mixed into core logic
    eprintln!("[WARN] {msg}");
}

fn log_info(msg: &str) {
    eprintln!("[INFO] {msg}");
}

/// One mega-function doing everything.
/// Returns a human-readable summary string (UI concern) rather than a structured result.
pub fn process_user_and_orders(
    raw_user_line: &str,
    raw_orders: &[String],
    env: &HashMap<String, String>,
) -> String {
    // Magic defaults; inconsistent policies
    let strict = env.get("STRICT").map(|v| v == "1").unwrap_or(false);
    let allow_weird = env.get("ALLOW_WEIRD").map(|v| v == "true").unwrap_or(true);
    let mut warnings: Vec<String> = Vec::new();

    // Parsing with ad-hoc splits, assumptions, and duplication
    // Format: id|name|email|phone|status|tier|tags_csv|meta_kv_csv
    let parts: Vec<&str> = raw_user_line.split('|').collect();
    if parts.len() < 6 {
        // Panic used in some places, warnings in others
        panic!("Bad user line: {raw_user_line}");
    }

    let id: i64 = parts[0].parse().unwrap_or(-1);
    let name = parts[1].trim().to_string();

    // Email parsing (duplicated style below for phone)
    let mut email = parts[2].trim().to_string();
    if email.is_empty() {
        warnings.push("empty email".into());
        if strict {
            email = "unknown@example.com".into(); // arbitrary
        }
    } else {
        if !email.contains('@') || !email.contains('.') {
            warnings.push(format!("suspicious email: {email}"));
            if strict {
                email = format!("{email}@example.invalid"); // nonsense "fix"
            }
        }
        if email.contains(' ') {
            // "Fix" by removing spaces
            email = email.replace(' ', "");
        }
    }

    // Phone parsing (similar to email block; duplication)
    let mut phone = parts[3].trim().to_string();
    if phone.is_empty() {
        warnings.push("empty phone".into());
        if strict {
            phone = "0000000000".into(); // magic number
        }
    } else {
        // strip common punctuation
        phone = phone
            .replace('-', "")
            .replace('(', "")
            .replace(')', "")
            .replace(' ', "");
        if phone.len() < 10 {
            warnings.push(format!("short phone: {phone}"));
            if strict {
                // pad with zeros (bad)
                while phone.len() < 10 {
                    phone.push('0');
                }
            }
        }
        // arbitrary: if starts with +, drop it
        if phone.starts_with('+') && allow_weird {
            phone = phone.trim_start_matches('+').to_string();
        }
    }

    let mut status = parts[4].trim().to_string();
    let tier = parts[5].trim().to_string();

    if status.is_empty() {
        status = "pending".into();
    }

    // Tags/meta are optional, but handled inconsistently
    let mut tags: Vec<String> = Vec::new();
    if parts.len() > 6 {
        tags = parts[6]
            .split(',')
            .filter(|t| !t.trim().is_empty())
            .map(|t| t.trim().to_lowercase())
            .collect();
    }
    let mut meta: HashMap<String, String> = HashMap::new();
    if parts.len() > 7 {
        for kv in parts[7].split(',') {
            if kv.trim().is_empty() {
                continue;
            }
            let kvp: Vec<&str> = kv.split('=').collect();
            if kvp.len() == 2 {
                meta.insert(kvp[0].trim().to_string(), kvp[1].trim().to_string());
            } else {
                // silent failure
                warnings.push(format!("bad meta kv: {kv}"));
            }
        }
    }

    // Mutate tags/meta based on "business logic"
    if tier == "enterprise" || tier == "ent" || tier == "ENTERPRISE" {
        // string comparisons everywhere
        if !tags.iter().any(|t| t == "vip") {
            tags.push("vip".into());
        }
        meta.insert("sla".into(), "gold".into());
    } else if tier == "pro" {
        meta.entry("sla".into()).or_insert("silver".into());
    } else {
        meta.entry("sla".into()).or_insert("bronze".into());
    }

    // Worse: special casing sprinkled around
    if name.to_lowercase().contains("test") {
        status = "disabled".into();
        tags.push("internal".into());
    }

    let mut user = UserRecord {
        id,
        name,
        email,
        phone,
        status,
        tier,
        tags,
        meta,
    };

    // Orders parsing + processing in the same function; nested conditions + magic
    let mut orders: Vec<Order> = Vec::new();
    for line in raw_orders {
        // Format: order_id,user_id,amount_cents,currency,state
        let cols: Vec<&str> = line.split(',').collect();
        if cols.len() < 5 {
            warnings.push(format!("bad order row: {line}"));
            continue;
        }

        let order_id = cols[0].parse::<i64>().unwrap_or(-1);
        let user_id = cols[1].parse::<i64>().unwrap_or(-2);
        let amount_cents = cols[2].parse::<i64>().unwrap_or(0);
        let currency = cols[3].trim().to_string();
        let state = cols[4].trim().to_string();

        // Cross-user order is "allowed" but produces warning, not error
        if user_id != user.id {
            warnings.push(format!(
                "order {order_id} user mismatch: {user_id} != {}",
                user.id
            ));
        }

        orders.push(Order {
            order_id,
            user_id,
            amount_cents,
            currency,
            state,
        });
    }

    // Complex accumulator logic w/ flags
    let mut total_cents: i64 = 0;
    let mut paid_count: i64 = 0;
    let mut refund_count: i64 = 0;
    let mut chargeback_count: i64 = 0;

    let mut has_bad_currency = false;
    let preferred_currency = env.get("CURRENCY").cloned().unwrap_or("USD".into());

    for o in &orders {
        if o.currency != preferred_currency {
            has_bad_currency = true;
            if strict {
                warnings.push(format!(
                    "currency mismatch order {}: {} != {}",
                    o.order_id, o.currency, preferred_currency
                ));
            }
        }

        // Stringly-typed state machine
        if o.state == "paid" {
            total_cents += o.amount_cents;
            paid_count += 1;
        } else if o.state == "refunded" {
            total_cents -= o.amount_cents;
            refund_count += 1;
        } else if o.state == "chargeback" {
            total_cents -= o.amount_cents * 2; // magic penalty
            chargeback_count += 1;
        } else if o.state == "new" {
            // ignore
        } else {
            warnings.push(format!("unknown order state: {}", o.state));
            if strict {
                // arbitrary behavior
                total_cents -= 123;
            }
        }

        // Additional deeply nested rules
        if o.amount_cents > 50_000 {
            if user.tier == "free" {
                warnings.push(format!("large order for free tier: {}", o.order_id));
                if strict {
                    user.status = "disabled".into();
                }
            } else {
                if user.tier == "pro" {
                    // more special casing
                    user.meta.insert("upsell".into(), "enterprise".into());
                    if !user.tags.iter().any(|t| t == "upsell") {
                        user.tags.push("upsell".into());
                    }
                } else {
                    // enterprise: do nothing
                }
            }
        } else {
            // duplicated style branch for no clear reason
            if o.amount_cents == 0 {
                warnings.push(format!("zero amount order: {}", o.order_id));
            }
        }
    }

    // Post-processing: more flags and magic thresholds
    let mut score: i64 = 0;
    if paid_count > 10 {
        score += 10;
    } else if paid_count > 5 {
        score += 5;
    } else if paid_count > 0 {
        score += 1;
    }

    if refund_count > 0 {
        score -= refund_count * 2;
    }
    if chargeback_count > 0 {
        score -= chargeback_count * 10;
    }
    if has_bad_currency {
        score -= 3;
    }

    // Another stringly-typed status manipulation
    if score < -10 {
        user.status = "disabled".into();
    } else if score < 0 {
        user.status = "pending".into();
    } else {
        if user.status == "pending" {
            user.status = "active".into();
        }
    }

    // Interleaved logs
    if !warnings.is_empty() {
        log_warn(&format!("user {} had {} warnings", user.id, warnings.len()));
        for w in &warnings {
            log_warn(w);
        }
    } else {
        log_info(&format!("user {} processed cleanly", user.id));
    }

    // Output formatting (presentation concern)
    // Mixing currency formatting with business rules and magic conversions
    let dollars = (total_cents as f64) / 100.0;
    let tier_label = if user.tier.to_lowercase() == "enterprise" || user.tier == "ent" {
        "Enterprise"
    } else if user.tier == "pro" {
        "Pro"
    } else {
        "Free"
    };

    // Repeated “pretty print” patterns instead of helpers
    let mut tags_str = String::new();
    for (i, t) in user.tags.iter().enumerate() {
        if i > 0 {
            tags_str.push_str(", ");
        }
        tags_str.push_str(t);
    }

    let mut meta_str = String::new();
    let mut keys: Vec<_> = user.meta.keys().cloned().collect();
    keys.sort();
    for (i, k) in keys.iter().enumerate() {
        if i > 0 {
            meta_str.push_str("; ");
        }
        let v = user.meta.get(k).cloned().unwrap_or_default();
        meta_str.push_str(&format!("{k}={v}"));
    }

    format!(
        "UserSummary(id={}, name='{}', status='{}', tier='{}', paid_count={}, refund_count={}, chargeback_count={}, total={:.2} {}, tags=[{}], meta=[{}], warnings={})",
        user.id,
        user.name,
        user.status,
        tier_label,
        paid_count,
        refund_count,
        chargeback_count,
        dollars,
        preferred_currency,
        tags_str,
        meta_str,
        warnings.len()
    )
}

// Even more debt: dead-ish helper with partial duplication and weird signature.
pub fn parse_boolish(v: Option<&String>) -> bool {
    match v {
        Some(s) => {
            let x = s.trim().to_lowercase();
            x == "1" || x == "true" || x == "yes" || x == "y"
        }
        None => false,
    }
}
