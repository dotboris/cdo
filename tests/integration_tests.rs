use std::{
    env,
    ffi::OsString,
    fs::{self, Permissions},
    os::unix::fs::PermissionsExt,
    path::Path,
};

use anyhow::Result;
use assert_cmd::Command;

#[test]
fn test_full_run() -> Result<()> {
    let workdir = tempfile::tempdir()?;

    let run_dir = workdir.path().join("directory-to-run-in");
    fs::create_dir(&run_dir)?;

    let bin_dir = workdir.path().join("bin");
    fs::create_dir(&bin_dir)?;
    let command_path = bin_dir.join("command-to-run");
    fs::write(
        &command_path,
        concat!(
            "#!/bin/sh\n",
            "echo running command $(readlink -nf \"$0\")\n",
            "echo running in $(readlink -nf \"$PWD\")\n",
            "echo args are $@\n",
            ">&2 echo hello from stderr\n"
        ),
    )?;
    fs::set_permissions(&command_path, Permissions::from_mode(0o755))?;

    let mut expected_stdout = String::new();
    expected_stdout.push_str(&format!(
        "running command {}\n",
        fs::canonicalize(&command_path)?.display()
    ));
    expected_stdout.push_str(&format!(
        "running in {}\n",
        fs::canonicalize(&run_dir)?.display()
    ));
    expected_stdout.push_str("args are foo bar baz\n");

    Command::cargo_bin("cdo")?
        .env("PATH", path_with(&[&bin_dir])?)
        .arg(&run_dir)
        .arg("command-to-run")
        .args(["foo", "bar", "baz"])
        .assert()
        .success()
        .stdout(expected_stdout)
        .stderr("hello from stderr\n");

    Ok(())
}

#[test]
fn test_echo_command() -> Result<()> {
    let workdir = tempfile::tempdir()?;

    let run_dir = workdir.path().join("directory-to-run-in");
    fs::create_dir(&run_dir)?;

    Command::cargo_bin("cdo")?
        .arg(&run_dir)
        .arg("echo")
        .args(["fi", "fo", "fum"])
        .assert()
        .success()
        .stdout("fi fo fum\n");

    Ok(())
}

#[test]
fn test_pwd_command() -> Result<()> {
    let workdir = tempfile::tempdir()?;

    let run_dir = workdir.path().join("directory-to-run-in");
    fs::create_dir(&run_dir)?;

    Command::cargo_bin("cdo")?
        .arg(&run_dir)
        .arg("pwd")
        .assert()
        .success()
        .stdout(format!("{}\n", fs::canonicalize(&run_dir)?.display()));

    Ok(())
}

#[test]
fn test_command_absolute_path() -> Result<()> {
    let workdir = tempfile::tempdir()?;

    let run_dir = workdir.path().join("directory-to-run-in");
    fs::create_dir(&run_dir)?;

    let bin_dir = workdir.path().join("bin");
    fs::create_dir(&bin_dir)?;
    let command_path = bin_dir.join("command-to-run");
    fs::write(&command_path, concat!("#!/bin/sh\n", "echo it works\n",))?;
    fs::set_permissions(&command_path, Permissions::from_mode(0o755))?;

    Command::cargo_bin("cdo")?
        .arg(&run_dir)
        .arg(&command_path)
        .assert()
        .success()
        .stdout("it works\n");

    Ok(())
}

#[test]
fn test_pass_all_flags() -> Result<()> {
    let workdir = tempfile::tempdir()?;

    let run_dir = workdir.path().join("directory-to-run-in");
    fs::create_dir(&run_dir)?;

    Command::cargo_bin("cdo")?
        .arg(&run_dir)
        .arg("echo")
        .args([
            "--fi",
            "--fo",
            "--fum",
            "--help",
            "--version",
            "-h",
            "-V",
            "--",
            "--more-flags",
        ])
        .assert()
        .success()
        .stdout("--fi --fo --fum --help --version -h -V -- --more-flags\n");

    Ok(())
}

#[test]
fn test_crash_when_command_not_found() -> Result<()> {
    let workdir = tempfile::tempdir()?;

    let run_dir = workdir.path().join("directory-to-run-in");
    fs::create_dir(&run_dir)?;

    Command::cargo_bin("cdo")?
        .arg(&run_dir)
        .arg("command-that-does-not-exist")
        .assert()
        .failure();

    Ok(())
}

#[test]
fn test_crash_when_directory_not_found() -> Result<()> {
    Command::cargo_bin("cdo")?
        .arg("dir-that-does-not-exist")
        .arg("echo")
        .args(["foo", "bar", "baz"])
        .assert()
        .failure();

    Ok(())
}

#[test]
fn test_crash_when_directory_is_file() -> Result<()> {
    let workdir = tempfile::tempdir()?;

    let run_dir = workdir.path().join("directory-to-run-in");
    fs::write(&run_dir, "This is actually a file. HaHa!")?;

    Command::cargo_bin("cdo")?
        .arg(&run_dir)
        .arg("echo")
        .args(["foo", "bar", "baz"])
        .assert()
        .failure();

    Ok(())
}

fn path_with(extras: &[&Path]) -> Result<OsString> {
    let current_path = env::var_os("PATH").unwrap_or("".into());
    let mut parsed_path = env::split_paths(&current_path).collect::<Vec<_>>();
    parsed_path.append(&mut extras.iter().map(|extra| extra.into()).collect());

    Ok(env::join_paths(parsed_path)?)
}
