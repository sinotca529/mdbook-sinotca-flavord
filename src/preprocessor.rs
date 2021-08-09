use mdbook::book::{Book, BookItem};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};

pub struct Prep;

impl Prep {
    pub fn new() -> Self {
        Self
    }
}

// for mathjax
impl Prep {
    /// Convert
    /// - `\\` surrounded by `$$` or `$` to `\\\\`
    /// - `_` surrounded by `$$` or `$` to `\_`
    /// You can use backslash to escape delimiter `$`
    fn escape_special_chars_for_mathjax(content: &str) -> String {
        let escape_display = Self::escape_special_chars_for_mathjax_with_delimiter(content, "$$");
        Self::escape_special_chars_for_mathjax_with_delimiter(&escape_display, "$")
    }

    /// Convert
    /// - `\\` surrounded by `delimiter` to `\\\\`
    /// - `_` surrounded by `$$` or `$` to `\_`
    /// You can use backslash to escape delimiter `$`
    fn escape_special_chars_for_mathjax_with_delimiter(content: &str, delimiter: &str) -> String {
        split_inclusive(content, delimiter, true)
            .into_iter()
            .enumerate()
            .map(|(n, substr)| {
                if n % 2 == 0 {
                    substr
                }
                else {
                    substr
                        .replace(r"\\", r"\\\\")
                        .replace(r"_", r"\_")
                }
            })
            .fold(String::new(), |mut acc, substr| {
                acc.push_str(&substr);
                acc
            })
    }
}

impl Preprocessor for Prep {
    fn name(&self) -> &str {
        super::PREPROCESSOR_NAME
    }

    fn run(&self, _ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        book.for_each_mut(|item| {
            if let BookItem::Chapter(chapter) = item {
                chapter.content = Self::escape_special_chars_for_mathjax(&chapter.content);
            }
        });

        Ok(book)
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer == "html"
    }
}

fn split_inclusive(string: &str, delimiter: &str, escape_backslash: bool) -> Vec<String> {
    if escape_backslash {
        let mut splitted = string.split_inclusive(delimiter);
        let mut current_substr = splitted.next();
        let mut result = Vec::new();

        while let Some(substr) = current_substr {
            let mut substr = substr.to_string();
            while substr.ends_with(r"\$") {
                current_substr = splitted.next();
                if let Some(next) = current_substr {
                    substr.push_str(next);
                }
                else {
                    break;
                }
            }
            result.push(substr);
            current_substr = splitted.next();
        }
        result
    }
    else {
        string.split_inclusive(delimiter)
        .map(|substr| substr.to_string())
        .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_inclusive_test() {
        let s = r"don't escape → \\, _, \$ \\ \$";
        assert_eq!(
            split_inclusive(s, "$", true),
            vec![
                s.to_string(),
            ]
        );

        let s = r"foo\$a\$b$x$";
        assert_eq!(
            split_inclusive(s, "$", true),
            vec![
                s[0..10].to_string(), // foo\$a\$b$
                s[10..].to_string()   // x$
            ]
        );

        let s = r"$x$";
        assert_eq!(
            split_inclusive(s, "$", true),
            vec![
                s[0..1].to_string(), // $
                s[1..].to_string()   // x$
            ]
        );

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

    #[test]
    fn escape_special_chars_for_mathjax_test() {
        let s = r"don't escape → \\, _, \$ \\ \$";
        assert_eq!(
            Prep::escape_special_chars_for_mathjax(s),
            s.to_string()
        );

        let s = r"$\begin{align} f &= \sigma (x + 1) \label{eq:hoge} \\ &= \sigma (1 + x) \end{align}$";
        assert_eq!(
            Prep::escape_special_chars_for_mathjax(s),
            r"$\begin{align} f &= \sigma (x + 1) \label{eq:hoge} \\\\ &= \sigma (1 + x) \end{align}$".to_string()
        );

        let s = r"$$\begin{align} f &= \sigma (x + 1) \label{eq:hoge} \\ &= \sigma (1 + x) \end{align}$$";
        assert_eq!(
            Prep::escape_special_chars_for_mathjax(s),
            r"$$\begin{align} f &= \sigma (x + 1) \label{eq:hoge} \\\\ &= \sigma (1 + x) \end{align}$$".to_string()
        );

        let s = r"$t_n=max({t_{rap}}_n,{t_{tar}}_{n}+t_{mis}\frac{{t_{res}}_n}{t_{res}})$";
        assert_eq!(
            Prep::escape_special_chars_for_mathjax(s),
            r"$t\_n=max({t\_{rap}}\_n,{t\_{tar}}\_{n}+t\_{mis}\frac{{t\_{res}}\_n}{t\_{res}})$".to_string()
        );

        let s = r"\$←ドルマーク。$\bm{x}$です。$$\begin{align} f &= \sigma (x + 1) \label{eq:hoge} \\ &= \sigma (1 + x) \end{align}$$いぇい";
        assert_eq!(
            Prep::escape_special_chars_for_mathjax(s),
            r"\$←ドルマーク。$\bm{x}$です。$$\begin{align} f &= \sigma (x + 1) \label{eq:hoge} \\\\ &= \sigma (1 + x) \end{align}$$いぇい".to_string()
        );
    }
}