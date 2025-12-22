use chrono::{DateTime, SecondsFormat, Utc};
use serde_json::Value;

pub struct Assertions;

impl Assertions {
    pub fn json_path_any_assert_eq(
        json: &str,
        path: &str,
        expected: &str,
    ) {
        let value: Vec<Value> = jsonpath_lib::select_as(json, path).unwrap();
        for v in value {
            if v.is_string() {
                if v.as_str().unwrap() == expected {
                    return;
                }
            } else if v.is_boolean() {
                if v.as_bool().unwrap().to_string() == expected {
                    return;
                }
            } else {
                if v.to_string() == expected {
                    return;
                }
            }
        }
        assert!(false, "None value matches");
    }

    pub fn json_path_any_assert_eq_date(
        json: &str,
        path: &str,
        expected: &DateTime<Utc>,
    ) {
        let value: Vec<Value> = jsonpath_lib::select_as(json, path).unwrap();
        for v in value {
            let string_date = v.as_str().unwrap();
            if string_date == expected.to_rfc3339_opts(SecondsFormat::Micros, true)
                || string_date == expected.to_rfc3339_opts(SecondsFormat::Millis, true)
                || string_date == expected.to_rfc3339_opts(SecondsFormat::Secs, true)
            {
                return;
            }
        }
        assert!(false, "None value matches with date: {}", expected);
    }
}
