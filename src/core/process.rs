use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::{Child, Command, Stdio};

/// Check whether mpvpaper is installed and reachable in PATH.
pub fn check_mpvpaper_installed() -> Result<()> {
    which_binary("mpvpaper")
}

/// Check whether a binary exists in PATH.
pub fn which_binary(name: &str) -> Result<()> {
    let output = Command::new("which")
        .arg(name)
        .output()
        .with_context(|| format!("Failed to run 'which {}'", name))?;
    if output.status.success() {
        Ok(())
    } else {
        bail!(
            "'{}' not found in PATH.\nTip: install it with: paru -S {}",
            name, name
        )
    }
}

/// Spawn mpvpaper for a given monitor and video file.
/// Returns the PID of the spawned process.
///
/// Command structure:
///   mpvpaper <monitor> <video_path> [extra_flags...]
pub fn spawn_mpvpaper(
    monitor: &str,
    video_path: &str,
    extra_flags: &[String],
) -> Result<u32> {
    // Validate mpvpaper is installed
    check_mpvpaper_installed()?;

    // Validate the video file exists
    if !Path::new(video_path).exists() {
        bail!(
            "Video file not found: {}\nTip: check the path and try again.",
            video_path
        );
    }

    let mut cmd = Command::new("mpvpaper");
    cmd.arg(monitor);
    cmd.arg(video_path);
    for flag in extra_flags {
        cmd.arg(flag);
    }
    // Detach stdout/stderr so the terminal stays clean
    cmd.stdout(Stdio::null());
    cmd.stderr(Stdio::null());

    let child: Child = cmd
        .spawn()
        .with_context(|| format!("Failed to spawn mpvpaper for monitor '{}'.", monitor))?;

    let pid = child.id();
    // Detach — we store the PID in state.json and manage it manually
    std::mem::forget(child);

    Ok(pid)
}

/// Kill a process by PID.
/// Returns Ok(()) if the process was killed or was already dead.
/// Never returns an error just because the process was already gone.
pub fn kill_pid(pid: u32) -> Result<()> {
    if !is_pid_alive(pid) {
        // Already dead — nothing to do, state will be cleaned by caller
        return Ok(());
    }
    let status = Command::new("kill")
        .arg("-TERM")
        .arg(pid.to_string())
        .status()
        .with_context(|| format!("Failed to run kill on PID {}", pid))?;

    if !status.success() {
        // Try SIGKILL as fallback
        let _ = Command::new("kill")
            .arg("-KILL")
            .arg(pid.to_string())
            .status();
    }
    Ok(())
}

/// Check whether a PID corresponds to a live process.
/// Uses /proc/<pid> for efficiency — no subprocess needed.
pub fn is_pid_alive(pid: u32) -> bool {
    Path::new(&format!("/proc/{}", pid)).exists()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_process_is_alive() {
        let pid = std::process::id();
        assert!(is_pid_alive(pid));
    }

    #[test]
    fn nonexistent_pid_is_not_alive() {
        // PID 0 is never a user process
        assert!(!is_pid_alive(0));
    }
}
