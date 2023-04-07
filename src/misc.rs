const TIME_UNITS: &[(&str, u16)] = &[
    ("second", 60),
    ("minute", 60),
    ("hour", 24),
    ("day", 30),
    ("month", 12),
    ("year", 0),
];

pub trait TryStrip {
    fn try_strip(&self, prefix: &str) -> &str;
}

impl TryStrip for str {
    fn try_strip(&self, prefix: &str) -> &str {
        if let Some(i) = self.strip_prefix(prefix) {
            return i;
        }

        self
    }
}

pub fn best_time(secs: u64) -> String {
    if secs <= 1 {
        return "just now".to_owned();
    }

    let mut secs = secs as f64;

    for i in TIME_UNITS {
        if i.1 == 0 || secs < i.1 as f64 {
            secs = secs.round();
            return format!("{} {}{} ago", secs, i.0, if secs > 1.0 { "s" } else { "" });
        }

        secs /= i.1 as f64;
    }

    format!("{} years ago", secs.round())
}
