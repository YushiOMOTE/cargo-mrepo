use anyhow::{bail, Result};
use cargo_metadata::{MetadataCommand, Package};
use std::{
    collections::HashMap,
    env,
    path::{Path, PathBuf},
};
use structopt::StructOpt;
use walkdir::WalkDir;

fn package(p: &Path) -> Option<Package> {
    let mut cmd = MetadataCommand::new();
    cmd.manifest_path(p);
    let metadata = cmd.exec().unwrap();
    metadata.root_package().cloned()
}

#[derive(StructOpt)]
#[structopt(bin_name = "cargo mrepo")]
struct Opt {}

#[derive(Clone, Debug)]
struct LocalRepo {
    path: PathBuf,
    package: Package,
}

impl LocalRepo {
    fn new(path: &Path, package: &Package) -> Self {
        Self {
            path: path.to_owned(),
            package: package.clone(),
        }
    }
}

#[derive(Default, Debug)]
struct LocalRepos {
    packages: HashMap<String, LocalRepo>,
}

impl LocalRepos {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, path: &Path) -> Result<()> {
        if let Some(package) = package(path) {
            let name = package.name.clone();

            let repo = LocalRepo::new(path, &package);

            if let Some(existing) = self.packages.insert(name.clone(), repo.clone()) {
                bail!(
                    "found duplicate package `{}`: `{}` and `{}`",
                    name,
                    repo.path.display(),
                    existing.path.display()
                );
            }
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let args = env::args().skip_while(|arg| arg != "mrepo");
    let opt = Opt::from_iter(args);

    let paths: Vec<_> = WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| !e.file_type().is_dir())
        .filter(|e| e.file_name() == "Cargo.toml")
        .map(|e| e.into_path())
        .collect();

    let mut repos = LocalRepos::new();
    for p in paths {
        repos.add(&p);
    }

    println!("Local repos: {:#?}", repos);

    Ok(())
}
