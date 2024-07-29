use crate::language::{Language, LanguageList};

pub const LANGS: LanguageList = LanguageList(&[
    Language {
        template: include_str!("template.c"),
        names: &["c"],
        extensions: &["c"],
    },
    Language {
        template: include_str!("template.sh"),
        names: &["bash", "sh"],
        extensions: &["bash", "sh"],
    },
]);
