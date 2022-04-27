use self_update::cargo_crate_version;

fn main() {
    update().unwrap_err();
    println!(
        "Thanks for downloading this program! Version ({})",
        cargo_crate_version!()
    );
}

fn update() -> Result<(), Box<dyn std::error::Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("chloe-woahie")
        .repo_name("distribution-practice")
        .bin_name("distribution-practice")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}
