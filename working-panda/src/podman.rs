use std::{process::{Output, Command}, io::Error, str::FromStr, path::Path};

pub struct Podman {
    registry_url: String,
}

impl Podman {
    pub fn new(registry_url: &str) -> Podman {
        return Podman { registry_url: String::from_str(registry_url).expect("") };
    }

    pub fn build(&mut self, path: &Path, tag: &String) -> Result<Output, Error> {
        return Command::new("podman")
            .args(["build", "-t", tag, "-f", path.to_str().unwrap()])
            .output();
    }

    pub fn tag(&mut self, tag: &String) -> Result<Output, Error> {
        return Command::new("podman")
            .args(["tag", &format!("localhost/{}", &tag), &format!("{}/{}", &self.registry_url, &tag)])
            .output();
    }

    pub fn push(&mut self, tag: &String) -> Result<Output, Error> {
        return Command::new("podman")
            .args(["push", &format!("{}/{}", &self.registry_url, &tag)])
            .output();
    }
}
