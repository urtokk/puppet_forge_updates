use serde::Deserialize;
use colored::*;

static FORGE_URL: &'static str = "https://forgeapi.puppetlabs.com";

// Struct to Version separated to Major, Minor and Patch
#[derive(Deserialize, Debug)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

#[derive(Debug, Eq, PartialEq)]
enum VersionUpdate {
    Major,
    Minor,
    Patch,
}

// Struct for a module with its name,current version and latest version
// it also implements the method to get the latest version.
#[derive(Debug)]
pub(crate) struct PuppetModule {
    name: String,
    current_version: Version,
    latest_version: Version,
}

impl PuppetModule {
    pub(crate) fn new(name: &str, current_version: &str) -> PuppetModule {
        let name = String::from(name);
        let current_version = Version::from(current_version);
        let latest_version = PuppetModule::get_latest_version(name.as_str());
        PuppetModule {
            name,
            current_version,
            latest_version,
        }
    }

    fn get_latest_version(name: &str) -> Version {
        let url = format!("{}/v3/modules/{}", FORGE_URL, name);
        let response = reqwest::blocking::get(url).unwrap();
        let module: serde_json::Value = response.json().unwrap();
        // if a version is not found print an error with the response and exit
        if module["current_release"]["version"].is_null() {
            println!("Error: {}", module);
            std::process::exit(1);
        }
        Version::from(module["current_release"]["version"].as_str().unwrap())
    }

    fn determine_update(&self) -> Option<VersionUpdate> {
        if self.current_version.major < self.latest_version.major {
            Some(VersionUpdate::Major)
        } else if self.current_version.minor < self.latest_version.minor {
            Some(VersionUpdate::Minor)
        } else if self.current_version.patch < self.latest_version.patch {
            Some(VersionUpdate::Patch)
        } else {
            None
        }
    }
}

// function to read a Puppetfile and return a vector of PuppetModule
pub(crate) fn read_puppetfile(path: &str) -> Vec<PuppetModule> {
    let file = std::fs::read_to_string(path).unwrap();
    let mut modules = Vec::new();
    for line in file.lines() {
        if line.starts_with("mod") {
            let mut line = line.split_whitespace();
            line.next();
            // the name needs to be normilized, removing the quotes and the comma from the whole string
            let name = line.next().unwrap().replace("'", "").replace(",", "");
            // the version needs to be normilized, removing the quotes and the comma from the whole string
            let version = line.next().unwrap().replace("'", "").replace(",", "");
            modules.push(PuppetModule::new(name.as_str(), version.as_str()));
        }
    }
    modules
}

// implement Disiplay for PuppetModule.
// It will determine the update and display tho new version depending on the enum VersionUpdate
// If it is None, it will print the version normally.
// If it is Major the new version will be red.
// If it is Minor the new version will be yellow.
// If it is Patch the new version will be bleu.
impl std::fmt::Display for PuppetModule {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.determine_update() {
            Some(VersionUpdate::Major) => write!(
                f,
                "{} {} -> {}",
                self.name,
                self.current_version,
                self.latest_version.to_string().red()
            ),
            Some(VersionUpdate::Minor) => write!(
                f,
                "{} {} -> {}",
                self.name,
                self.current_version,
                self.latest_version.to_string().yellow()
            ),
            Some(VersionUpdate::Patch) => write!(
                f,
                "{} {} -> {}",
                self.name,
                self.current_version,
                self.latest_version.to_string().blue()
            ),
            None => write!(f, "{} {}", self.name, self.current_version),
        }
    }
}

impl Version {
    fn from(version: &str) -> Version {
        let version: Vec<&str> = version.split(".").collect();
        Version {
            major: version[0].parse().unwrap(),
            minor: version[1].parse().unwrap(),
            patch: version[2].parse().unwrap(),
        }
    }

    // return the version as a string
    fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

// implement Display for Version.
// It will display the version in the format Major.Minor.Patch
impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

mod tests {
    use colored::*;

    // test for Version struct
    #[test]
    fn test_version_from() {
        let version = super::Version::from("1.2.3");
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }

    // test for PuppetModule struct
    #[test]
    fn test_puppet_module_new() {
        let module = super::PuppetModule::new("puppetlabs-stdlib", "5.2.0");
        assert_eq!(module.name, "puppetlabs-stdlib");
        assert_eq!(module.current_version.major, 5);
        assert_eq!(module.current_version.minor, 2);
        assert_eq!(module.current_version.patch, 0);
        assert_eq!(module.latest_version.major, 8);
        assert_eq!(module.latest_version.minor, 5);
        assert_eq!(module.latest_version.patch, 0);
    }

    // test for PuppetModule::determine_update
    #[test]
    fn test_puppet_module_determine_update() {
        let module = super::PuppetModule::new("puppetlabs-stdlib", "5.2.0");
        assert_eq!(module.determine_update(), Some(super::VersionUpdate::Major));
        let module2 = super::PuppetModule::new("puppetlabs-stdlib", "8.5.0");
        assert_eq!(module2.determine_update(), None);
    }

    // test if the version is displayed correctly and colored
    #[test]
    fn test_puppet_module_display() {
        let module = super::PuppetModule::new("puppetlabs-stdlib", "5.2.0");
        assert_eq!(
            format!("{}", module),
            format!(
                "{} {} -> {}",
                module.name,
                module.current_version,
                module.latest_version.to_string().red()
            )
        );
        let module2 = super::PuppetModule::new("puppetlabs-stdlib", "8.5.0");
        assert_eq!(format!("{}", module2), "puppetlabs-stdlib 8.5.0");
    }
}
