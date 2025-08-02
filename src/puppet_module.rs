use colored::*;
use reqwest;
use serde::Deserialize;

static FORGE_URL: &'static str = "https://forgeapi.puppetlabs.com";

// Struct to Version separated to Major, Minor and Patch
#[derive(Deserialize, Debug)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum VersionUpdate {
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

    pub(crate) fn determine_update(&self) -> Option<VersionUpdate> {
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
        if line.starts_with("moduledir") {
            continue;
        }

        if line.starts_with("mod") {
            let mut line = line.split_whitespace();
            line.next();
            // the name needs to be normilized, removing the quotes and the comma from the whole string
            let name = line
                .next()
                .unwrap()
                .replace("'", "")
                .replace(",", "")
                .replace("\"", "");
            // the version needs to be normilized, removing the quotes and the comma from the whole string
            // if the version is not specified, skip the module, could be a git repo
            let version = match line.next() {
                Some(version) => version.replace("'", "").replace(",", "").replace("\"", ""),
                None => continue,
            };
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
            // if there is no update, do not print anything, also no empty lines
            None => write!(f, ""),
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

#[cfg(test)]
mod tests {
    use super::*;

    // Hilfsfunktion für Mock-Module ohne Forge-API
    impl PuppetModule {
        fn new_mock(name: &str, current_version: &str, latest_version: Version) -> PuppetModule {
            PuppetModule {
                name: name.to_string(),
                current_version: Version::from(current_version),
                latest_version,
            }
        }
    }

    #[test]
    fn test_version_from() {
        let version = Version::from("1.2.3");
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
    }

    #[test]
    fn test_version_to_string() {
        let version = Version {
            major: 1,
            minor: 2,
            patch: 3,
        };
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn test_version_display() {
        let version = Version {
            major: 4,
            minor: 5,
            patch: 6,
        };
        assert_eq!(format!("{}", version), "4.5.6");
    }

    #[test]
    fn test_puppet_module_new_mock() {
        let latest = Version {
            major: 8,
            minor: 5,
            patch: 0,
        };
        let module = PuppetModule::new_mock("puppetlabs-stdlib", "5.2.0", latest);
        assert_eq!(module.name, "puppetlabs-stdlib");
        assert_eq!(module.current_version.major, 5);
        assert_eq!(module.current_version.minor, 2);
        assert_eq!(module.current_version.patch, 0);
        assert_eq!(module.latest_version.major, 8);
        assert_eq!(module.latest_version.minor, 5);
        assert_eq!(module.latest_version.patch, 0);
    }

    #[test]
    fn test_puppet_module_determine_update_major() {
        let latest = Version {
            major: 8,
            minor: 5,
            patch: 0,
        };
        let module = PuppetModule::new_mock("puppetlabs-stdlib", "5.2.0", latest);
        assert_eq!(module.determine_update(), Some(VersionUpdate::Major));
    }

    #[test]
    fn test_puppet_module_determine_update_minor() {
        let latest = Version {
            major: 5,
            minor: 3,
            patch: 0,
        };
        let module = PuppetModule::new_mock("puppetlabs-stdlib", "5.2.0", latest);
        assert_eq!(module.determine_update(), Some(VersionUpdate::Minor));
    }

    #[test]
    fn test_puppet_module_determine_update_patch() {
        let latest = Version {
            major: 5,
            minor: 2,
            patch: 1,
        };
        let module = PuppetModule::new_mock("puppetlabs-stdlib", "5.2.0", latest);
        assert_eq!(module.determine_update(), Some(VersionUpdate::Patch));
    }

    #[test]
    fn test_puppet_module_determine_update_none() {
        let latest = Version {
            major: 5,
            minor: 2,
            patch: 0,
        };
        let module = PuppetModule::new_mock("puppetlabs-stdlib", "5.2.0", latest);
        assert_eq!(module.determine_update(), None);
    }

    #[test]
    fn test_puppet_module_display_major() {
        let latest = Version {
            major: 8,
            minor: 5,
            patch: 0,
        };
        let module = PuppetModule::new_mock("puppetlabs-stdlib", "5.2.0", latest);
        assert_eq!(
            format!("{}", module),
            format!(
                "{} {} -> {}",
                module.name,
                module.current_version,
                module.latest_version.to_string().red()
            )
        );
    }

    #[test]
    fn test_puppet_module_display_minor() {
        let latest = Version {
            major: 5,
            minor: 3,
            patch: 0,
        };
        let module = PuppetModule::new_mock("puppetlabs-stdlib", "5.2.0", latest);
        assert_eq!(
            format!("{}", module),
            format!(
                "{} {} -> {}",
                module.name,
                module.current_version,
                module.latest_version.to_string().yellow()
            )
        );
    }

    #[test]
    fn test_puppet_module_display_patch() {
        let latest = Version {
            major: 5,
            minor: 2,
            patch: 1,
        };
        let module = PuppetModule::new_mock("puppetlabs-stdlib", "5.2.0", latest);
        assert_eq!(
            format!("{}", module),
            format!(
                "{} {} -> {}",
                module.name,
                module.current_version,
                module.latest_version.to_string().blue()
            )
        );
    }

    #[test]
    fn test_puppet_module_display_none() {
        let latest = Version {
            major: 5,
            minor: 2,
            patch: 0,
        };
        let module = PuppetModule::new_mock("puppetlabs-stdlib", "5.2.0", latest);
        assert_eq!(format!("{}", module), "");
    }

    #[test]
    fn test_forge_live_response_to_version_struct() {
        // Holt eine echte Antwort von der Forge-API für ein bekanntes Modul
        let url = "https://forgeapi.puppetlabs.com/v3/modules/puppetlabs-stdlib";
        let response = reqwest::blocking::get(url).expect("Forge API nicht erreichbar");
        assert!(
            response.status().is_success(),
            "Forge API liefert Fehlerstatus"
        );
        let module: serde_json::Value = response.json().expect("Antwort ist kein JSON");
        let version_str = module["current_release"]["version"]
            .as_str()
            .expect("Version nicht gefunden");
        let version = Version::from(version_str);
        // Die genaue Version kann sich ändern, daher prüfen wir nur auf sinnvolle Werte
        assert!(version.major > 0, "Major-Version sollte > 0 sein");
        assert!(version.minor >= 0, "Minor-Version sollte >= 0 sein");
        assert!(version.patch >= 0, "Patch-Version sollte >= 0 sein");
    }
}
