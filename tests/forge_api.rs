use puppet_forge_updates::src::puppet_module::Version;

#[test]
fn test_forge_live_response_to_version_struct() {
    // Fetch a real response from the Forge API for a well-known module
    let url = "https://forgeapi.puppetlabs.com/v3/modules/puppetlabs-stdlib";
    let response = reqwest::blocking::get(url).expect("Forge API not reachable");
    assert!(
        response.status().is_success(),
        "Forge API returned error status"
    );
    let module: serde_json::Value = response.json().expect("Response is not valid JSON");
    let version_str = module["current_release"]["version"]
        .as_str()
        .expect("Version not found in Forge API response");
    let version = Version::from(version_str);
    // The exact version may change, so we only check for reasonable values
    assert!(version.major > 0, "Major version should be greater than 0");
    // Remove useless comparison warning: only check that minor and patch are numbers (parsing would fail otherwise)
    let _ = version.minor;
    let _ = version.patch;
}
