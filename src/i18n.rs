//! Interface translations: the Fluent messages of `locales/`, embedded in the executable.
//!
//! The code never names a message by a string: the messages without arguments are the
//! variants of [`Message`], and each message with arguments has its own method of
//! [`Translator`], the only place that knows the names of its arguments.

use fluent_bundle::{FluentArgs, FluentBundle, FluentResource};

use crate::language::{Language, PerLanguage};

/// The Fluent messages of each language, embedded in the executable.
const MESSAGES: PerLanguage<&str> = PerLanguage {
    english: include_str!("../locales/en.ftl"),
    french: include_str!("../locales/fr.ftl"),
};

/// Declares [`Message`], its list for the tests and the identifier of each message in the
/// `.ftl` files, from one table, so that a message without arguments is declared once.
macro_rules! messages {
    ($($(#[$attribute:meta])* $variant:ident => $id:literal,)*) => {
        /// A message of the interface without arguments.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Message {
            $($(#[$attribute])* $variant,)*
        }

        impl Message {
            /// Every message, for the tests.
            #[cfg(test)]
            const ALL: &'static [Self] = &[$(Self::$variant,)*];

            /// Its identifier in the `.ftl` files.
            fn id(self) -> &'static str {
                match self {
                    $(Self::$variant => $id,)*
                }
            }
        }
    };
}

messages! {
    Greeting => "greeting",
    LanguageLabel => "language-label",
    LanguageSystem => "language-system",
    LanguageEnglish => "language-english",
    LanguageFrench => "language-french",
    AlwaysOnTop => "always-on-top",
}

/// The identifiers of the messages with arguments, each used by its method of [`Translator`].
const VERSION_ID: &str = "version";

/// The messages of one language.
pub struct Translator {
    bundle: FluentBundle<FluentResource>,
}

impl Translator {
    /// # Panics
    ///
    /// If the embedded messages of `language` can't be parsed, which the tests rule out.
    #[must_use]
    pub fn new(language: Language) -> Self {
        let resource = FluentResource::try_new((*MESSAGES.of(language)).to_owned())
            .unwrap_or_else(|(_, errors)| panic!("invalid messages for {language:?}: {errors:?}"));
        let mut bundle = FluentBundle::new(vec![language.identifier()]);
        // Without the Unicode isolation marks around the arguments, which egui would draw as
        // boxes.
        bundle.set_use_isolating(false);
        bundle
            .add_resource(resource)
            .unwrap_or_else(|errors| panic!("duplicate messages for {language:?}: {errors:?}"));
        Self { bundle }
    }

    /// The text of `message`.
    #[must_use]
    pub fn text(&self, message: Message) -> String {
        self.format(message.id(), None)
    }

    /// The version of the application, e.g. `Version 0.1.0`.
    #[must_use]
    pub fn version(&self, version: &str) -> String {
        let mut args = FluentArgs::new();
        args.set("version", version);
        self.format(VERSION_ID, Some(&args))
    }

    /// The message `id` with `args`. A missing message or an error in it fails the debug
    /// builds, which the tests run; a release shows the identifier or the text it could format.
    fn format(&self, id: &str, args: Option<&FluentArgs>) -> String {
        let Some(pattern) = self
            .bundle
            .get_message(id)
            .and_then(|message| message.value())
        else {
            debug_assert!(false, "missing message {id}");
            return id.to_owned();
        };
        let mut errors = Vec::new();
        let text = self
            .bundle
            .format_pattern(pattern, args, &mut errors)
            .into_owned();
        debug_assert!(errors.is_empty(), "errors in message {id}: {errors:?}");
        text
    }
}

#[cfg(test)]
mod tests;
