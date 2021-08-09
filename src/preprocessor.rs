use mdbook::book::Book;
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

pub struct Prep;

impl Prep {
    pub fn new() -> Self {
        Self
    }
}

impl Preprocessor for Prep {
    fn name(&self) -> &str {
        super::PREPROCESSOR_NAME
    }

    fn run(&self, ctx: &PreprocessorContext, book: Book) -> Result<Book, Error> {
        if let Some(nop_cfg) = ctx.config.get_preprocessor(self.name()) {
            if nop_cfg.contains_key("blow-up") {
                anyhow::bail!("Boom!1!");
            }
        }

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }
}

fn split_inclusive(string: &str, separator: &str, escape_backslash: bool) -> Vec<String> {
    if escape_backslash {
        let mut splitted = string.split_inclusive(separator);
        let mut current_substr = splitted.next();
        let mut result = Vec::new();

        while let Some(substr) = current_substr {
            let mut substr = substr.to_string();
            while substr.ends_with(r"\$") {
                current_substr = splitted.next();
                if let Some(next) = current_substr {
                    substr.push_str(next);
                }
            }
            result.push(substr);
            current_substr = splitted.next();
        }
        result
    }
    else {
        string.split_inclusive(separator)
        .map(|substr| substr.to_string())
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_inclusive_test1() {
        let s = r"foo\$a\$b$x$";
        assert_eq!(
            split_inclusive(s, "$", true),
            vec![
                s[0..10].to_string(), // foo\$a\$b$
                s[10..].to_string()   // x$
            ]
        );
    }
    #[test]
    fn split_inclusive_test2() {
        let s = r"$x$";
        assert_eq!(
            split_inclusive(s, "$", true),
            vec![
                s[0..1].to_string(), // $
                s[1..].to_string()   // x$
            ]
        );
    }

    #[test]
    fn split_inclusive_test3() {
        let s = r"foo\$a\$b$x$";
        assert_eq!(
            split_inclusive(s, "$", false),
            vec![
                s[0..5].to_string(), // foo\$
                s[5..8].to_string(), // a\$
                s[8..10].to_string(), //b$
                s[10..].to_string()   // x$
            ]
        );
    }
}