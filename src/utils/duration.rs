/// Formats a number of seconds as a [protobuf Duration](https://protobuf.dev/reference/protobuf/google.protobuf/#duration),
/// which JSON-encodes as a string holding the number of seconds with an `s` suffix.
/// Fractional seconds are permitted, e.g. `1.5s`.
pub(crate) fn duration_string(seconds: f64) -> String {
    format!("{}s", seconds)
}

#[cfg(test)]
mod tests {
    use super::duration_string;

    #[test]
    fn formats_whole_and_fractional_seconds() {
        assert_eq!(duration_string(0.0), "0s");
        assert_eq!(duration_string(1.0), "1s");
        assert_eq!(duration_string(1.5), "1.5s");
        assert_eq!(duration_string(3600.0), "3600s");
    }
}
