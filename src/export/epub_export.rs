use crate::export::{ExportOptions, Exporter};
use crate::models::issue::Issue;
use crate::phrack_issue_manager_error::PhrackIssueManagerError;

use epub_builder::{EpubBuilder, EpubContent, ZipLibrary};
use std::fs::{self, File};

pub struct EpubExporter;

const XHTML_TEMPLATE: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml">
<head>
    <title>{title}</title>
</head>
<body>
<pre>{content}</pre>
</body>
</html>"#;

const EPUB_CSS: &str = r#"
pre {
    font-family: monospace, 'Courier New', Courier;
    white-space: pre;
    font-size: 0.85em;
    line-height: 1.2;
    margin: 0;
    padding: 0.5em;
}
body {
    margin: 0;
    padding: 0;
}
"#;

trait EpubResultExt<T> {
    fn to_epub_err(self) -> Result<T, PhrackIssueManagerError>;
}

impl<T, E: std::fmt::Display> EpubResultExt<T> for Result<T, E> {
    fn to_epub_err(self) -> Result<T, PhrackIssueManagerError> {
        self.map_err(|e| PhrackIssueManagerError::EpubGeneration(e.to_string()))
    }
}

impl Exporter for EpubExporter {
    fn export(&self, issue: Issue, options: &ExportOptions) -> Result<(), PhrackIssueManagerError> {
        let article_paths = self.get_article_paths(&issue, &options)?;

        let output_path = options
            .output_folder
            .join(format!("phrack-{}-merged.epub", issue.issue_number));

        let zip_library = ZipLibrary::new().to_epub_err()?;
        let mut builder = EpubBuilder::new(zip_library).to_epub_err()?;

        builder
            .metadata("title", format!("Phrack Issue {}", issue.issue_number))
            .to_epub_err()?;
        builder.metadata("author", "Phrack Staff").to_epub_err()?;
        builder.metadata("lang", "en").to_epub_err()?;

        builder.stylesheet(EPUB_CSS.as_bytes()).to_epub_err()?;

        for (i, path) in article_paths.iter().enumerate() {
            let content = fs::read_to_string(&path)?;
            let article_num = i + 1;
            let article_title = format!("Article {}", article_num);

            let html = XHTML_TEMPLATE
                .replace("{title}", &article_title)
                .replace("{content}", &html_escape(&content));

            builder
                .add_content(
                    EpubContent::new(format!("article{}.xhtml", article_num), html.as_bytes())
                        .title(article_title),
                )
                .to_epub_err()?;
        }

        let mut output = File::create(&output_path)?;
        builder.generate(&mut output).to_epub_err()?;

        Ok(())
    }
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
