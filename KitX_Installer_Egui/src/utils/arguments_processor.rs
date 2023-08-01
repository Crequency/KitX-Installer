use crate::app_info::{AppInfo, RunMode};
use arguments::Arguments;

// Process arguments and return `AppInfo` if success.
pub fn args_to_app_info(args: Arguments) -> Option<AppInfo> {
    // `clia` means `command line interface argument`
    let mut clia_run_gui = args.get::<bool>("run-gui");
    let mut clia_run_cli = args.get::<bool>("run-cli");

    // when both `run-gui` and `run-cli` are not specified, auto detect.
    if clia_run_gui.is_none() && clia_run_cli.is_none() {
        clia_run_gui = Some(cfg!(target_os = "windows"));
        clia_run_cli = Some(!clia_run_gui.unwrap());
    }

    // when both `run-gui` and `run-cli` are specified to true, error and exit.
    if clia_run_gui.unwrap_or(false) && clia_run_cli.unwrap_or(false) {
        eprintln!("Cannot run GUI and CLI at the same time.");
        println!("Cannot run GUI and CLI at the same time.");
        return None;
    }

    if clia_run_gui.unwrap_or(false) || clia_run_cli.is_none() {
        clia_run_cli = Some(!clia_run_gui.unwrap_or(false));
    }

    if clia_run_cli.unwrap_or(false) || clia_run_gui.is_none() {
        clia_run_gui = Some(!clia_run_cli.unwrap_or(false));
    }

    let clia_run_gui = clia_run_gui.unwrap_or(false);
    let clia_run_cli = clia_run_cli.unwrap_or(false);

    let run_in_gui = (clia_run_gui || cfg!(target_os = "windows")) && !clia_run_cli;

    return Some(AppInfo {
        run_mode: if run_in_gui {
            Some(RunMode::Gui)
        } else {
            Some(RunMode::Cli)
        },
        version: env!("CARGO_PKG_VERSION").to_string(),
    });
}
