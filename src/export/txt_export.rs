use crate::export::{ExportOptions, Exporter};
use crate::models::issue::Issue;

use std::io::Write;
use std::{fs, fs::File};

pub struct TxtExporter;

impl Exporter for TxtExporter {
    fn export(
        &self,
        issue: Issue,
        options: &ExportOptions,
    ) -> Result<(), crate::phrack_issue_manager_error::PhrackIssueManagerError> {
        let article_paths = self.get_article_paths(&issue, &options)?;

        let output_path = options
            .output_folder
            .join(format!("phrack-{}-merged.txt", issue.issue_number));

        let output_file = File::create(&output_path)?;

        for path in article_paths {
            let content = fs::read_to_string(&path)?;
            writeln!(&output_file, "{}\n", content)?;
        }

        Ok(())
    }
}
