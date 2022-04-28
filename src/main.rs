use self_update::cargo_crate_version;

fn main() {
    try_update().expect_err("Program updated. Please start it again!");
    println!(
        "Thanks for downloading this program! Version ({})",
        cargo_crate_version!()
    );
}

/// If this function returns an Ok, the program updated itself and the binary should likely be restarted
fn try_update() -> Result<(), Box<dyn std::error::Error>> {
    let update_system_build_result = self_update::backends::github::Update::configure()
        .repo_owner("chloe-woahie")
        .repo_name("distribution-practice")
        .bin_name("distribution-practice")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build();

    let update_result = match update_system_build_result {
        Ok(update_system) => update_system.update(),
        Err(e) => {
            panic!("Failed to build update system! ({})", e);
        }
    };

    match update_result {
        // If update_result is Ok, then the updater updated the program
        Ok(_) => return Ok(()),
        Err(e) => {
            // Update Errors are allowed because they mean the user decided not to update
            match e {
                self_update::errors::Error::Update(_) => {}
                _ => panic!("Updater failed! ({})", e),
            }

            return Err("Program did not update.".into());
        }
    }
}
