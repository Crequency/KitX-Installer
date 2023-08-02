use std::process::Command;

pub fn create_shortcut(
    location: String,
    target: String,
    arguments: Option<String>,
    working_dir: String,
    descr: Option<String>,
    icon_path: Option<String>,
    window_style: Option<i32>,
) {
    let raw_commands = format!(
        "
        $shell = New-Object -ComObject WScript.Shell;
        $lnk = $shell.CreateShortcut(\"{}\");
        $lnk.TargetPath = \"{}\";
        $lnk.Arguments = \"{}\";
        $lnk.WorkingDirectory = \"{}\";
        $lnk.Description = \"{}\";
        $lnk.WindowStyle = {};
        {}
        $lnk.Save();
        ",
        location,
        target,
        arguments.unwrap_or("".to_string()),
        working_dir,
        descr.unwrap_or("".to_string()),
        window_style.unwrap_or(1),
        if icon_path.is_some() {
            format!("$lnk.IconLocation = \"{}\";", icon_path.unwrap())
        } else {
            "".to_string()
        },
    );

    let output = Command::new("powershell")
        .arg("-ExecutionPolicy")
        .arg("Bypass") // 忽略脚本执行策略（仅在需要时使用）
        .arg("-Command")
        .arg(raw_commands)
        // .arg(format!("\"{}\"", script_path))
        .output()
        .expect("Failed to run powershell script in order to create shortcut.");
    if output.status.success() {
        // println!("Successfully created shortcut.");
    } else {
        // println!("Failed to create shortcut.");
    }
}
