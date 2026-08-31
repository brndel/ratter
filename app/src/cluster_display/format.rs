use std::fmt::Display;

pub const NULL_FORMAT: &'static str = "???";

pub fn format_millis(milli_value: i64, unit: ValueUnit) -> String {
    let (value, unit_prefix) = transform_with_scale_prefix(milli_value);

    format!("{} {}{}", value, unit_prefix, unit)
}

fn transform_with_scale_prefix(milli_value: i64) -> (i64, &'static str) {
    let prefixes = ["m", "", "k", "m", "g"];

    let mut value_at_current_scale = milli_value;

    for prefix in prefixes {
        if value_at_current_scale <= 1000 {
            return (value_at_current_scale, prefix);
        }
        value_at_current_scale /= 1000;
    }

    (value_at_current_scale, "t")
}

pub fn format_100_scaled_value(value: impl Into<i64>, unit: ValueUnit) -> String {
    let value = value.into();

    let comma_value = value % 100;
    let decimal_value = value / 100;

    format!("{},{:02} {}", decimal_value, comma_value, unit)
}

pub enum ValueUnit {
    Volt,
    Watt,
    WattHour,
    Celcius,
    Percent,
}

impl Display for ValueUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueUnit::Volt => write!(f, "V"),
            ValueUnit::Watt => write!(f, "W"),
            ValueUnit::WattHour => write!(f, "Wh"),
            ValueUnit::Celcius => write!(f, "°C"),
            ValueUnit::Percent => write!(f, "%"),
        }
    }
}


pub fn format_percent(percent: f32) -> String {
    if percent > 1.0 {
        ">100 %".to_owned()
    } else if percent < 0.0 {
        "<0 %".to_owned()
    } else {
        format!("{} %", (percent * 100.0) as u8)
    }
}