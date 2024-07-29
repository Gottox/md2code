use markdown::mdast::Code;
use std::path::Path;

pub use crate::templates::LANGS;

pub struct Language {
    pub template: &'static str,
    pub names: &'static [&'static str],
    pub extensions: &'static [&'static str],
}

impl Language {
    pub fn has_name_code(&self, code: &Code) -> bool {
        code.lang
            .as_deref()
            .map_or(false, |lang| self.has_name(lang))
    }
    pub fn has_name(&self, name: &str) -> bool {
        self.names.contains(&name)
    }
    pub fn has_extension(&self, path: &str) -> bool {
        Path::new(path).extension().map_or(false, |ext| {
            self.extensions.contains(&ext.to_str().unwrap())
        })
    }
}

pub struct LanguageList(pub &'static [Language]);

impl LanguageList {
    pub fn find_by_name(&self, name: &str) -> Option<&'static Language> {
        self.0
            .iter()
            .find(|lang| lang.has_name(&name.to_lowercase()))
    }
    pub fn find_by_extension(&self, ext: &str) -> Option<&'static Language> {
        self.0
            .iter()
            .find(|lang| lang.has_extension(&ext.to_lowercase()))
    }
}
