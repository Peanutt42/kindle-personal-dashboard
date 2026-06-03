use crate::KindleScreensaverError;
use std::path::PathBuf;
use std::process::Command;

pub fn is_kindle_automatic_screensaver_blocked() -> Result<bool, KindleScreensaverError> {
    let program = "lipc-get-prop";
    // -i means int property;
    // -e means no new line in output
    let args = &["-i", "-e", "com.lab126.powerd", "preventScreenSaver"];
    let cmd = format_cmd(program, args);
    let while_doing = "querying if automatic screensaver is blocked".to_string();

    let output = Command::new(program).args(args).output();

    let output = match output {
        Ok(output) => output,
        Err(cause) => {
            return Err(KindleScreensaverError::ProgramFailedToRun {
                cmd,
                while_doing,
                cause,
            });
        }
    };

    if !output.status.success() {
        return Err(KindleScreensaverError::ProcessExitedUnsuccessfully {
            cmd,
            while_doing,
            exit_code: output.status.code(),
        });
    }

    if output.stdout == [b'1'] {
        Ok(true)
    } else if output.stdout == [b'0'] {
        Ok(false)
    } else {
        let stdout_output = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr_output = String::from_utf8_lossy(&output.stderr).to_string();

        Err(KindleScreensaverError::UnknownOutputOfLipcGetProp {
            cmd,
            stdout_output,
            stderr_output,
        })
    }
}

pub fn set_kindle_automatic_screensaver_blocked(
    blocked: bool,
) -> Result<(), KindleScreensaverError> {
    let program = "lipc-set-prop";
    // -i means int property;
    let args = &[
        "-i",
        "com.lab126.powerd",
        "preventScreenSaver",
        if blocked { "1" } else { "0" },
    ];
    let cmd = format_cmd(program, args);
    let while_doing = format!(
        "{} automatic screensaver",
        if blocked { "blocking" } else { "unblocking" }
    );

    match Command::new(program).args(args).status() {
        Ok(status) => {
            if status.success() {
                Ok(())
            } else {
                Err(KindleScreensaverError::ProcessExitedUnsuccessfully {
                    cmd,
                    while_doing,
                    exit_code: status.code(),
                })
            }
        }
        Err(cause) => Err(KindleScreensaverError::ProgramFailedToRun {
            cmd,
            while_doing,
            cause,
        }),
    }
}

pub fn get_config_filepath() -> PathBuf {
    PathBuf::from("/mnt/us/extensions/kindle-personal-dashboard/kpd_config.toml")
}

fn format_cmd(program: &str, args: &[&str]) -> String {
    format!(
        "{program} {}",
        args.iter()
            .map(|str| str.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    )
}
