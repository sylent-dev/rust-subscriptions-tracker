use super::*;
use std::collections::BTreeSet;

/// The identifiers of the messages of the file of `language`.
fn file_ids(language: Language) -> BTreeSet<String> {
    let resource =
        FluentResource::try_new((*MESSAGES.of(language)).to_owned()).expect("the messages parse");
    resource
        .entries()
        .filter_map(|entry| match entry {
            fluent_syntax::ast::Entry::Message(message) => Some(message.id.name.to_owned()),
            _ => None,
        })
        .collect()
}

#[test]
fn the_code_and_every_file_have_the_same_messages() {
    let code_ids: BTreeSet<String> = Message::ALL
        .iter()
        .map(|message| message.id())
        .chain([VERSION_ID])
        .map(str::to_owned)
        .collect();

    for language in Language::ALL {
        assert_eq!(file_ids(language), code_ids, "{language:?}");
    }
}

#[test]
fn every_message_formats_without_errors() {
    // The errors fail the `debug_assert` of `format`.
    for language in Language::ALL {
        let translator = Translator::new(language);
        for &message in Message::ALL {
            assert_ne!(translator.text(message), "", "{message:?} in {language:?}");
        }
        assert!(
            translator.version("1.2.3").contains("1.2.3"),
            "{language:?}"
        );
    }
}

#[test]
fn each_language_has_its_own_text() {
    assert_eq!(
        Translator::new(Language::English).text(Message::LanguageLabel),
        "Language"
    );
    assert_eq!(
        Translator::new(Language::French).text(Message::LanguageLabel),
        "Langue"
    );
}
