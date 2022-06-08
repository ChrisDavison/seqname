use std::path::Path;

pub struct Renamer {
    prefix: String,
    suffix: String,
    verbose: bool,
    keep_filename: bool,
    dry_run: bool,
    separator: String,
}

impl Renamer {
    pub fn new(
        prefix: Option<String>,
        suffix: Option<String>,
        verbose: bool,
        keep_filename: bool,
        dry_run: bool,
        separator: Option<String>,
    ) -> Renamer {
        let prefix = prefix.unwrap_or(String::new());
        let suffix = suffix.unwrap_or(String::new());
        let separator = separator.unwrap_or(String::from("--"));
        Renamer {
            prefix,
            suffix,
            verbose,
            keep_filename,
            dry_run,
            separator,
        }
    }

    fn stem(&self, path: &Path) -> String {
        match (self.keep_filename, path.file_stem()) {
            (true, Some(s)) => s.to_string_lossy().to_string(),
            _ => String::new(),
        }
    }

    fn new_filename(&self, path: &Path, index: usize) -> String {
        let ext = match path.extension() {
            Some(e) => format!(".{}", e.to_string_lossy()),
            None => "".into(),
        };
        let file_stem = self.stem(&path);
        let parts: String = vec![
            self.prefix.clone(),
            format!("{:04}", index),
            file_stem,
            self.suffix.clone(),
        ]
        .iter()
        .filter(|s| !s.is_empty())
        .cloned()
        .collect::<Vec<String>>()
        .join(&self.separator);
        parts + &ext
    }

    pub fn rename(&self, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let valid_paths = dir
            .read_dir()?
            .filter_map(|x| x.ok())
            .filter(|x| x.path().is_file());
        for (i, path) in valid_paths.enumerate() {
            let mut new = path.path().clone();
            new.set_file_name(self.new_filename(&path.path(), i));
            if self.verbose || self.dry_run {
                println!("{:?} ->\n\t{:?}", path.path(), new);
            }
            if !self.dry_run {
                std::fs::rename(path.path(), new)?;
            }
        }
        Ok(())
    }
}
