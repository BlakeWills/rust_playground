use std::fmt::Display;

pub fn enumerations_main() {
    let eco_saw_result = PowerTool::new(10);
    match eco_saw_result {
        Ok(s) => println!("saw: {s}"),
        Err(err) => println!("{err}")
    }
}

#[derive(Debug)]
enum ToolSetting {
    Off,
    Eco(i32),
    Turbo(i32)
}

#[derive(Debug)]
struct PowerTool {
    power_mode: ToolSetting
}

impl PowerTool {
    fn new(battery_pct: i32) -> Result<Self, String> {
        ToolSetting::try_from(battery_pct).map(|mode| Self { power_mode: mode})
    }
}

impl TryFrom<i32> for ToolSetting {
    type Error = String;

    fn try_from(battery_pct: i32) -> Result<Self, Self::Error> {
        match battery_pct {
            0 => Ok(ToolSetting::Off),
            b if b <= 50 => Ok(ToolSetting::Eco(b)),
            b if b <= 100 => Ok(ToolSetting::Turbo(b)),
            b => Err(format!("Invalid battery level: {b}"))
        }
    }
}

impl Display for ToolSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let battery_pct_str = match self {
            ToolSetting::Off => String::from("Off (0)"),
            ToolSetting::Eco(b) => format!("Eco ({b})"),
            ToolSetting::Turbo(b) => format!("Turbo ({b})")
        };

        write!(f,"{battery_pct_str}")
    }
}

impl Display for PowerTool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "power_mode: {0}", self.power_mode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_off_mode() {
        let result = PowerTool::new(0);
        assert!(result.is_ok(), "Expected an Ok result, but got {:?}", result);

        let tool = result.unwrap();
        assert_eq!("power_mode: Off (0)", &tool.to_string());
    }

    #[test]
    fn test_eco_mode() {
        let result = PowerTool::new(10);
        assert!(result.is_ok(), "Expected an Ok result, but got {:?}", result);

        let tool = result.unwrap();
        assert_eq!("power_mode: Eco (10)", &tool.to_string());
    }

    #[test]
    fn test_turbo_mode() {
        let result = PowerTool::new(51);
        assert!(result.is_ok(), "Expected an Ok result, but got {:?}", result);

        let tool = result.unwrap();
        assert_eq!("power_mode: Turbo (51)", &tool.to_string());
    }

    #[test]
    fn test_inavlid_battery() {
        let result = PowerTool::new(101);
        assert!(result.is_err(), "Expected an Err result, but got {:?}", result);
    }
}