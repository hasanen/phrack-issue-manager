use crate::export::{ExportOptions, Exporter};
use crate::models::issue::Issue;

use genpdf::elements::{Paragraph, Text};
use genpdf::style::{Style, StyledString};
use genpdf::{Alignment, Document, fonts};
use std::fs;

pub struct PDFExporter;

impl Exporter for PDFExporter {
    fn export(
        &self,
        issue: Issue,
        options: &ExportOptions,
    ) -> Result<(), crate::phrack_issue_manager_error::PhrackIssueManagerError> {
        let article_paths = self.get_article_paths(&issue, &options)?;

        let output_path = options
            .output_folder
            .join(format!("phrack-{}-merged.pdf", issue.issue_number));

        let font_family = fonts::from_files("./fonts", "LiberationMono", None)?;

        let mut doc = Document::new(font_family);
        doc.set_title(format!("Phrack Issue {}", issue.issue_number));
        doc.set_font_size(11);

        let mut decorator = genpdf::SimplePageDecorator::new();
        decorator.set_margins(13);
        decorator.set_header(|page| {
            let styled = StyledString::new(
                format!("Page {}", page),
                Style::new().bold().with_font_size(10),
            );
            let mut header = Paragraph::from(vec![styled]);
            header.set_alignment(Alignment::Right);
            header
        });
        doc.set_page_decorator(decorator);

        for path in article_paths {
            let content = fs::read_to_string(&path)?;
            for line in content.lines() {
                doc.push(Text::new(line));
            }
        }

        doc.render_to_file(&output_path)?;

        Ok(())
    }
}
