use std::path::PathBuf;

use super::prelude::*;

pub struct Storage {
    folder: PathBuf
}

impl Storage {
    #[inline]
    pub async fn new(folder: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let folder = folder.into();

        if !folder.exists() {
            tokio::fs::create_dir_all(&folder).await?;
        }

        Ok(Self {
            folder
        })
    }

    /// Remove special characters from the given string
    /// to prevent abuse of FS features
    pub fn sanitize_name(name: impl AsRef<str>) -> String {
        name.as_ref()
            .chars()
            .filter(char::is_ascii)
            .collect::<String>()
            .replace("..", "")
            .replace(['/', '\\'], "")
    }

    #[inline]
    /// Build path to the storage entry using given package info
    /// 
    /// This method will use sanitizer for the package's name to prevent FS features abuse
    pub fn build_path(&self, name: impl AsRef<str>, hash: &Hash) -> PathBuf {
        self.folder.join(format!("{hash}-{}", Self::sanitize_name(name)))
    }

    /// Find path to the stored entity if it is
    /// already installed
    pub async fn search_hash(&self, hash: &Hash) -> anyhow::Result<Option<PathBuf>> {
        let mut dir = tokio::fs::read_dir(&self.folder).await?;

        let hash = hash.to_string();

        while let Some(entry) = dir.next_entry().await? {
            if entry.file_name().to_string_lossy().starts_with(&hash) {
                return Ok(Some(entry.path()));
            }
        }

        Ok(None)
    }

    /// Search for paths for the packages with given name
    /// 
    /// Under the hood this method compares given name with
    /// endings of the entries in the storage folder
    /// 
    /// This method will use sanitizer for the package's name to prevent FS features abuse
    pub async fn search_name(&self, name: impl AsRef<str>) -> anyhow::Result<Vec<PathBuf>> {
        let mut dir = tokio::fs::read_dir(&self.folder).await?;

        let name = Self::sanitize_name(name);

        // We expect to find at least 1 value, and unlikely more
        let mut paths = Vec::with_capacity(1);

        while let Some(entry) = dir.next_entry().await? {
            if entry.file_name().to_string_lossy().ends_with(&name) {
                paths.push(entry.path());
            }
        }

        Ok(paths)
    }

    /// Install package to the packages storage
    /// 
    /// This method will resolve package's dependency tree
    /// and process every input and output from it
    /// 
    /// This method will call given callback with currently
    /// installed packages count, total count and currently
    /// installing package's name
    pub async fn install(&self, package: Package, callback: impl Fn(usize, usize, &str)) -> anyhow::Result<()> {
        let dependencies = Resolver::resolve_dependencies(package).await?;

        let total_install_total = dependencies.len();

        let client = reqwest::Client::new();

        // Install dependencies
        for (installed_count, dependency) in dependencies.into_iter().enumerate() {
            callback(installed_count, total_install_total, dependency.name());

            // If this dependency is not installed in the current storage
            if dependency.resolve(self).await?.is_none() {
                // Fetch dependency body
                let response = client.get(dependency.uri())
                    .send().await?;

                // Return immediately if failed
                if !response.status().is_success() {
                    // Packages as inputs are special case
                    let is_package = matches!(dependency, Dependency::Input { input: ManifestInput { format: ManifestInputFormat::Package, .. }, .. });

                    if !is_package {
                        anyhow::bail!("Failed to fetch '{}' from '{}': HTTP code {}", dependency.name(), dependency.uri(), response.status().as_u16());
                    }
                }

                // Otherwise install it to the storage
                match dependency {
                    Dependency::Input { input, name, .. } => {
                        let path = self.build_path(&name, &input.hash);

                        match input.format {
                            ManifestInputFormat::File => {
                                // TODO: input files can weight a lot, there should be another callback, or better - a Downloader
                                tokio::fs::write(path, response.bytes().await?).await?;
                            }

                            ManifestInputFormat::Package => {
                                // This is a little special case. Package inputs reference to the
                                // package's root, e.g. "https://raw.githubusercontent.com/an-anime-team/game-integrations/main/games/an-anime-game"
                                // so they will always return 404 or something like this
                                // We don't need to download here anything (I think)
                            }

                            _ => unimplemented!()
                        }
                    }

                    Dependency::Output { output, .. } => {
                        let path = self.build_path(&output.metadata.name, &output.hash);

                        // Outputs are lua scripts - launcher integrations or arbitrary packages
                        // We don't really care about their size or specific installation methods here
                        tokio::fs::write(path, response.bytes().await?).await?;
                    }
                }
            }
        }

        Ok(())
    }
}