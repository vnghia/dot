use std::fmt::Debug;
use std::fs::Permissions;
use std::io::{Cursor, Read};
use std::marker::PhantomData;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use clap::ValueEnum;
use flate2::bufread::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use rand::distributions::{Alphanumeric, DistString};
use tar::Archive;
use tempfile::TempDir;
use zip::ZipArchive;

use super::BinaryArgs;
use crate::prefix::Prefix;
use crate::utils::unwrap_or_missing_argument;

// 1MiB
const CHUNK_SIZE: usize = 1024 * 1024;

pub const VERSION_PATTERN: &str = "%VERSION%";

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ArchiveType {
    #[clap(name = "tar.gz")]
    TarGz,
    Gz,
    Zip,
}

impl ArchiveType {
    fn extract_tar_gz(data: &[u8], dir: impl AsRef<Path>) -> Option<Vec<u8>> {
        Archive::new(GzDecoder::new(data)).unpack(dir).unwrap();
        None
    }

    fn extract_gz(data: &[u8], _: impl AsRef<Path>) -> Option<Vec<u8>> {
        let mut buf = vec![];
        GzDecoder::new(data).read_to_end(&mut buf).unwrap();
        Some(buf)
    }

    fn extract_zip(data: &[u8], dir: impl AsRef<Path>) -> Option<Vec<u8>> {
        ZipArchive::new(Cursor::new(data)).unwrap().extract(dir).unwrap();
        None
    }

    fn extract(self, data: &[u8], dir: impl AsRef<Path>) -> Option<Vec<u8>> {
        match self {
            ArchiveType::TarGz => Self::extract_tar_gz(data, dir),
            ArchiveType::Gz => Self::extract_gz(data, dir),
            ArchiveType::Zip => Self::extract_zip(data, dir),
        }
    }
}

pub struct Binary<'a, 'b, 'c, 'd, 't, T: Debug + 't>
where
    &'t T: IntoIterator<Item = &'t &'c str>,
    'c: 't,
{
    pub name: &'a str,
    pub url: &'b str,
    pub archive: Option<(ArchiveType, Option<T>)>,
    pub version_arg: &'d str,
    pub phantom_c: PhantomData<&'c str>,
    pub phantom_t: PhantomData<&'t T>,
}

impl<'s, 'c, 't, T: Debug + 't> Binary<'_, '_, 'c, '_, 't, T>
where
    &'t T: IntoIterator<Item = &'t &'c str>,
    'c: 't,
    's: 't,
{
    pub fn download(&'s self, prefix: &Prefix, bin_version: &str) {
        let bin_dir = prefix.bin();
        let bin_path = bin_dir.join(self.name);
        let url = self.url.replace(VERSION_PATTERN, bin_version);
        log::info!(name = self.name, url:% = url; "Downloading binary");

        let pb = ProgressBar::new_spinner().with_style(
            ProgressStyle::with_template(
                "{spinner:.green} [{elapsed_precise}] {bytes} {bytes_per_sec}",
            )
            .unwrap(),
        );

        let mut buf = vec![];
        let mut buf_len = 0;
        let mut reader = reqwest::blocking::get(&url).unwrap();
        loop {
            buf.extend_from_slice(&[0; CHUNK_SIZE]);
            let chunk = &mut buf.as_mut_slice()[buf_len..buf_len + CHUNK_SIZE];
            let read_len = reader.read(chunk).unwrap();

            if read_len == 0 {
                break;
            } else {
                buf_len += read_len;
                pb.set_position(buf_len as u64);
            }
        }
        buf.truncate(buf_len);

        pb.finish_and_clear();
        log::info!(name = self.name, elapsed:? = pb.elapsed(); "Finish downloading");

        let buf = if let Some((archive_type, archive_paths)) = self.archive.as_ref() {
            log::info!(name = self.name, archive:? = self.archive; "Extracting binary");
            let temp_dir = TempDir::new().unwrap();
            if let Some(buf) = archive_type.extract(&buf, &temp_dir) {
                Some(buf)
            } else {
                let mut archive_path = temp_dir.path().to_path_buf();
                if let Some(archive_paths) = archive_paths {
                    for path in archive_paths.into_iter() {
                        archive_path = archive_path.join(path);
                    }
                } else {
                    archive_path = archive_path.join(self.name);
                }
                let bin_tmp_path =
                    bin_dir.join(Alphanumeric.sample_string(&mut rand::thread_rng(), 16));
                std::fs::copy(archive_path, &bin_tmp_path).unwrap();
                std::fs::rename(&bin_tmp_path, &bin_path).unwrap();
                None
            }
        } else {
            Some(buf)
        };

        if let Some(buf) = buf {
            let bin_tmp_path =
                bin_dir.join(Alphanumeric.sample_string(&mut rand::thread_rng(), 16));
            std::fs::write(&bin_tmp_path, buf).unwrap();
            std::fs::rename(&bin_tmp_path, &bin_path).unwrap();
        }
        std::fs::set_permissions(&bin_path, Permissions::from_mode(0o777)).unwrap();

        log::info!(name = self.name, arg = self.version_arg; "Downloaded binary version");
        std::process::Command::new(&bin_path)
            .arg(self.version_arg)
            .spawn()
            .unwrap()
            .wait()
            .unwrap()
            .exit_ok()
            .unwrap();
    }
}

impl<'s, 'a, 'b, 'c, 'd, 't> TryFrom<&'s BinaryArgs> for Binary<'a, 'b, 'c, 'd, 't, Vec<&'c str>>
where
    's: 'a + 'b + 'c + 'd + 't,
{
    type Error = clap::Error;

    fn try_from(value: &'s BinaryArgs) -> Result<Self, Self::Error> {
        let name = unwrap_or_missing_argument(value.name.as_deref(), "name", None)?;
        let url = unwrap_or_missing_argument(value.url.as_deref(), "url", None)?;
        let version_arg =
            unwrap_or_missing_argument(value.version_arg.as_deref(), "version-arg", None)?;
        Ok(Self {
            name,
            url,
            archive: value.archive_type.map(|t| {
                (t, value.archive_paths.as_ref().map(|v| v.iter().map(String::as_str).collect()))
            }),
            version_arg: version_arg.trim_matches('^'),
            phantom_c: std::marker::PhantomData,
            phantom_t: std::marker::PhantomData,
        })
    }
}
