use self_update::cargo_crate_version;
use self_update::update::ReleaseUpdate;
use semver::Version;

#[derive(Debug, Clone)]
pub struct ReleaseInfo {
    pub version: String,
    pub body: String,
}

pub fn check_update_available() -> Result<Option<ReleaseInfo>, String> {
    let updater = build_updater(None)?;
    let current_version_str = cargo_crate_version!();
    let current_version = Version::parse(current_version_str).map_err(|e| {
        format!(
            "Failed to parse current version '{}': {}",
            current_version_str, e
        )
    })?;

    let releases = updater
        .get_latest_releases(current_version_str)
        .map_err(|e| format!("Update check failed: {}", e))?;

    let release = releases
        .into_iter()
        .filter_map(|candidate| {
            let candidate_version = candidate.version.trim_start_matches('v');
            Version::parse(candidate_version)
                .ok()
                .filter(|v| *v > current_version)
                .map(|v| (candidate, v))
        })
        .max_by(|x, y| x.1.cmp(&y.1))
        .map(|(release, _)| release);

    Ok(release.map(|release| ReleaseInfo {
        version: release.version.trim_start_matches('v').to_string(),
        body: release.body.unwrap_or_default(),
    }))
}
pub fn apply_update_to_version(target_version: &str) -> Result<(), String> {
    // Ensure tag has 'v' prefix as GitHub tags are versioned like "v2.11.2"
    let tag = if target_version.starts_with('v') {
        target_version.to_string()
    } else {
        format!("v{}", target_version)
    };
    let updater = build_updater(Some(tag))?;
    updater
        .update()
        .map_err(|e| format!("Update failed: {}", e))?;
    Ok(())
}

fn build_updater(target_version: Option<String>) -> Result<Box<dyn ReleaseUpdate>, String> {
    let mut builder = self_update::backends::github::Update::configure();
    builder
        .repo_owner("RouHim")
        .repo_name("rhinco-tv")
        .bin_name("rhinco-tv")
        .show_download_progress(false)
        .show_output(false)
        .no_confirm(true)
        .current_version(cargo_crate_version!());
    if let Some(ver) = target_version {
        builder.target_version_tag(&ver);
    }
    builder
        .build()
        .map_err(|e| format!("Failed to configure updater: {}", e))
}
