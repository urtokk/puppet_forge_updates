use colored::*;
use reqwest;
use serde::Deserialize;

static FORGE_URL: &str = "https://forgeapi.puppetlabs.com";

// Struct to Version separated to Major, Minor and Patch
#[derive(Deserialize, Debug)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub enum VersionUpdate {
    Major,
    Minor,
    Patch,
}

// Struct for a module with its name,current version and latest version
// it also implements the method to get the latest version.
#[derive(Debug)]
pub struct PuppetModule {
    pub name: String,
    pub current_version: Version,
    pub latest_version: Version,
}

impl PuppetModule {
    pub(crate) fn new(name: &str, current_version: &str) -> Result<PuppetModule, String> {
        let name = String::from(name);
        let current_version = Version::from(current_version);
        let latest_version = PuppetModule::get_latest_version(name.as_str())?;
        Ok(PuppetModule {
            name,
            current_version,
            latest_version,
        })
    }

    fn get_latest_version(name: &str) -> Result<Version, String> {
        let url = format!("{FORGE_URL}/v3/modules/{name}");
        let response = reqwest::blocking::get(url).map_err(|e| e.to_string())?;
        let module: serde_json::Value = response.json().map_err(|e| e.to_string())?;
        // if a version is not found print an error with the response and exit
        if module["current_release"]["version"].is_null() {
            return Err(format!("Error: {module}"));
        }
        Ok(Version::from(
            module["current_release"]["version"].as_str().unwrap(),
        ))
    }

    pub fn determine_update(&self) -> Option<VersionUpdate> {
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
pub fn read_puppetfile(path: &str) -> Vec<PuppetModule> {
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
            if let Ok(module) = PuppetModule::new(name.as_str(), version.as_str()) {
                modules.push(module);
            }
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
    pub fn from(version: &str) -> Version {
        let version: Vec<&str> = version.split('.').collect();
        if version.len() != 3 {
            panic!(
                "Invalid version string '{}'. Expected format 'x.y.z', got {} parts.",
                version.join("."),
                version.len()
            );
        }
        Version {
            major: version[0]
                .parse()
                .unwrap_or_else(|_| panic!("Invalid major version in '{}'", version[0])),
            minor: version[1]
                .parse()
                .unwrap_or_else(|_| panic!("Invalid minor version in '{}'", version[1])),
            patch: version[2]
                .parse()
                .unwrap_or_else(|_| panic!("Invalid patch version in '{}'", version[2])),
        }
    }

    // return the version as a string
    // Removed inherent to_string to avoid clippy::inherent_to_string_shadow_display
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
    use std::fs::File;
    use std::io::Write;

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
        assert_eq!(version.major, 1, "Major version should be 1");
        assert_eq!(version.minor, 2, "Minor version should be 2");
        assert_eq!(version.patch, 3, "Patch version should be 3");
    }

    #[test]
    fn test_version_to_string() {
        let version = Version {
            major: 1,
            minor: 2,
            patch: 3,
        };
        assert_eq!(
            version.to_string(),
            "1.2.3",
            "Version string should be '1.2.3'"
        );
    }

    #[test]
    #[should_panic]
    fn test_version_from_invalid() {
        // Should panic if version string is invalid
        let _ = Version::from("1.2");
    }

    #[test]
    fn test_invalid_module_slug() {
        // Should return Err for invalid module slug
        let result = PuppetModule::new("foo", "1.0.0");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_puppetfile_nonexistent() {
        // Should panic if file does not exist
        let result = std::panic::catch_unwind(|| {
            let _ = read_puppetfile("this_file_should_not_exist.puppetfile");
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_read_puppetfile_empty() {
        let path = "test_empty_puppetfile";
        File::create(path).unwrap();
        let modules = read_puppetfile(path);
        assert!(modules.is_empty());
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_read_puppetfile_module_without_version() {
        let path = "test_no_version_puppetfile";
        let mut file = File::create(path).unwrap();
        writeln!(file, "mod 'foo'").unwrap();
        file.flush().unwrap();
        let modules = read_puppetfile(path);
        assert!(modules.is_empty());
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_read_puppetfile_valid_module() {
        let path = "test_valid_puppetfile";
        let mut file = File::create(path).unwrap();
        writeln!(file, "mod 'foo', '1.2.3'").unwrap();
        file.flush().unwrap();
        let modules = read_puppetfile(path);
        // Da "foo" kein gültiges Modul ist, sollte das Ergebnis leer sein
        assert!(modules.is_empty());
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_version_display() {
        let version = Version {
            major: 4,
            minor: 5,
            patch: 6,
        };
        assert_eq!(
            format!("{version}"),
            "4.5.6",
            "Display output should be '4.5.6'"
        );
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
            format!("{module}"),
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
            format!("{module}"),
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
            format!("{module}"),
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
        assert_eq!(format!("{module}"), "");
    }
}
