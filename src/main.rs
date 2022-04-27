use clap::crate_version;

fn main() {
    update().expect_err("Program updated, please restart the program!");
    println!(
        "Thanks for downloading this program! Version ({})",
        crate_version!()
    );
}

use self_update::cargo_crate_version;

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("jaemk")
        .repo_name("self_update")
        .bin_name("github")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}
