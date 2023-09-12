use anyhow::{anyhow, Result};
use clap::Parser;
use std::process::Command as Cmd;
use uuid::Uuid;

/// Returns build information, similar to: 0.1.0 (2be4034 2022-03-31).
const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("VERGEN_GIT_SHA"),
    " ",
    env!("VERGEN_GIT_COMMIT_DATE"),
    ")"
);

#[derive(Debug, Parser)]
#[clap(name = "spin cloud-gpu", version = VERSION)]
pub enum App {
    /// Deploy the fermyon-cloud-gpu Spin app to act as a cloud GPU proxy.
    Init,
    /// Create credentials to connect to the fermyon-cloud-gpu Spin app.
    Connect,
    /// Destroy the fermyon-cloud-gpu Spin app.
    Destroy,
}

fn main() -> Result<(), anyhow::Error> {
    match App::parse() {
        App::Init => init(),
        App::Connect => connect(),
        App::Destroy => destroy(),
    }
}

fn init() -> Result<(), anyhow::Error> {
    println!("Deploying fermyon-cloud-gpu Spin app ...");

    let auth_token = generate_auth_token();

    let result = Cmd::new(spin_bin_path()?)
        .arg("deploy")
        .arg("-f")
        .arg(spin_toml_path()?)
        .arg("--variable")
        .arg(format!("auth_token={auth_token}"))
        .output()?;

    if !result.status.success() {
        return Err(anyhow!(
            "Failed to deploy fermyon-cloud-gpu: {}",
            String::from_utf8_lossy(&result.stderr)
        ));
    }

    show_how_to_configure(auth_token);

    Ok(())
}

fn connect() -> Result<(), anyhow::Error> {
    println!("Connecting to fermyon-cloud-gpu Spin app ...");

    let auth_token = generate_auth_token();

    let result = Cmd::new(spin_bin_path()?)
        .arg("cloud")
        .arg("variables")
        .arg("set")
        .arg(format!("auth_token={auth_token}"))
        .arg("--app")
        .arg("fermyon-cloud-gpu")
        .output()?;

    if !result.status.success() {
        return Err(anyhow!(
            "Failed to update auth_token in fermyon-cloud-gpu: {}",
            String::from_utf8_lossy(&result.stderr)
        ));
    }

    show_how_to_configure(auth_token);

    Ok(())
}

fn destroy() -> Result<(), anyhow::Error> {
    println!("Destroying fermyon-cloud-gpu Spin app ...");

    let result = Cmd::new(spin_bin_path()?)
        .arg("cloud")
        .arg("apps")
        .arg("delete")
        .arg("fermyon-cloud-gpu")
        .output()?;

    if !result.status.success() {
        return Err(anyhow!(
            "Failed to delete fermyon-cloud-gpu: {}",
            String::from_utf8_lossy(&result.stderr)
        ));
    }

    Ok(())
}

fn generate_auth_token() -> String {
    Uuid::new_v4().to_string()
}

fn spin_bin_path() -> Result<String> {
    Ok(std::env::var("SPIN_BIN_PATH")?)
}

/// Returns the path to the spin.toml file of the fermyon-cloud-gpu Spin app.
fn spin_toml_path() -> Result<String> {
    Ok(std::env::current_exe()?
        .parent()
        .unwrap()
        .to_str()
        .ok_or(anyhow!("Could not get parent dir of executable"))?
        .to_owned()
        + "/fermyon-cloud-gpu/spin.toml")
}

fn show_how_to_configure(auth_token: String) {
    println!("Run the following command in your shell:");
    println!("export SPIN_CLOUD_GPU_AUTH_TOKEN={auth_token}");
}
