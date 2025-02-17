use std::{
    error::Error as StdError,
    fmt::{self, Display, Formatter},
    fs, io, iter,
    marker::PhantomData,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use prelude::*;

pub mod prelude {
    pub use ::std::{self, io::prelude::*, prelude::v1::*};
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Error {
    inner: Box<dyn StdError + Send + Sync + 'static>,
}

impl Error {
    pub fn new<E>(err: E) -> Self
    where
        E: Into<Box<dyn StdError + Send + Sync + 'static>>,
    {
        Self { inner: err.into() }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.inner.as_ref())
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self { inner: err.into() }
    }
}

/// An abstraction over platform-specific functionality.
#[derive(Debug)]
pub struct System<P> {
    config: PathBuf,
    presets: PhantomData<P>,
}

impl<P> System<P> {
    pub fn new<I>(presets: I) -> Result<Self>
    where
        P: Serialize,
        I: IntoIterator<Item = (&'static str, P)>,
    {
        let dirs = ProjectDirs::from("", "", "albw-randomizer")
            .ok_or_else(|| Error::new("Could not find suitable configuration directory."))?;
        if !dirs.config_dir().exists() {
            let presets_dir = dirs.config_dir().join("presets");
            fs::create_dir(&presets_dir)?;
            for (name, preset) in
                iter::once_with(standard_preset).chain(presets.into_iter().map(|(name, preset)| {
                    (
                        name,
                        toml::to_string_pretty(&preset).expect("Could not create builtin presets."),
                    )
                }))
            {
                fs::write(presets_dir.join(format!("{}.toml", name)), preset)?
            }
        }
        Ok(Self {
            config: dirs.config_dir().into(),
            presets: PhantomData,
        })
    }

    pub fn preset(&self, name: &str) -> Result<P>
    where
        P: DeserializeOwned,
    {
        toml::from_slice(&fs::read(
            self.config.join("presets").join(format!("{}.toml", name)),
        )?)
        .map_err(Error::new)
    }

    pub fn get_or_create_paths<F>(&self, create: F) -> Result<Paths>
    where
        F: FnOnce() -> Result<Paths>,
    {
        let file = self.config.join("Rando.toml");
        if file.exists() {
            Ok(toml::from_slice::<Paths>(&fs::read(file)?).map_err(Error::new)?)
        } else {
            let paths = create()?;
            fs::write(
                file,
                toml::to_string_pretty(&paths).expect("Could not write config file."),
            )?;
            Ok(paths)
        }
    }
}

/// Paths to the game ROM and output directories.
#[derive(Debug, Deserialize, Serialize)]
pub struct Paths {
    rom: PathBuf,
    output: PathBuf,
}

impl Paths {
    /// Generates new paths with the specified ROM and output directory.
    pub fn new(rom: PathBuf, output: PathBuf) -> Self {
        Self { rom, output }
    }

    /// Gets the path of the ROM file.
    pub fn rom(&self) -> &Path {
        &self.rom
    }

    /// Gets the output directory.
    pub fn output(&self) -> &Path {
        &self.output
    }
}

fn standard_preset() -> (&'static str, String) {
    (
        "Standard",
        include_str!("../../presets/Standard.toml").to_string(),
    )
}
