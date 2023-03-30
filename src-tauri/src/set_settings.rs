use std::process::Command;

pub fn set_brightness(value: i32) {
    let cmd = format!("(Get-WmiObject -Namespace root/WMI -Class WmiMonitorBrightnessMethods).WmiSetBrightness(1,{})", value);
    Command::new("powershell")
        .arg("-Command")
        .arg(&cmd)
        .output()
        .expect("Failed to execute command");
}
pub fn get_brightness() -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new("powershell")
        .args(&["-Command", r#"(Get-Ciminstance -Namespace root/WMI -ClassName WmiMonitorBrightness).CurrentBrightness"#])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
use winreg::enums::*;
use winreg::RegKey;

fn parse_root_key(key_path: &str) -> Result<(winreg::HKEY, String), Box<dyn std::error::Error>> {
    let path_parts: Vec<&str> = key_path.split("\\").collect();
    let root_key_str = path_parts[0].to_uppercase();
    let root_key = match root_key_str.as_str() {
        "HKEY_CLASSES_ROOT" => HKEY_CLASSES_ROOT,
        "HKEY_CURRENT_USER" => HKEY_CURRENT_USER,
        "HKEY_LOCAL_MACHINE" => HKEY_LOCAL_MACHINE,
        "HKEY_USERS" => HKEY_USERS,
        "HKEY_PERFORMANCE_DATA" => HKEY_PERFORMANCE_DATA,
        "HKEY_CURRENT_CONFIG" => HKEY_CURRENT_CONFIG,
        "HKEY_DYN_DATA" => HKEY_DYN_DATA,
        _ => return Err(format!("Invalid root key: {}", root_key_str).into()),
    };
    let sub_key_path = path_parts[1..].join("\\");
    Ok((root_key, sub_key_path))
}

pub fn set_registry_value(
    key_path: &str,
    value_name: &str,
    value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let (root_key, sub_key_path) = parse_root_key(key_path)?;
    let root = RegKey::predef(root_key);
    let (key, _) = root.create_subkey(&sub_key_path)?;
    key.set_value(value_name, &value.to_string())?;
    Ok(())
}

pub fn get_registry_value(
    key_path: &str,
    value_name: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let (root_key, sub_key_path) = parse_root_key(key_path)?;
    let root = RegKey::predef(root_key);
    let key = root.open_subkey(&sub_key_path)?;
    let value: String = key.get_value(value_name)?;
    Ok(value)
}
