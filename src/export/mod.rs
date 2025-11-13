use clap::ValueEnum;
use enum_iterator::Sequence;
use std::fs;
use std::path::PathBuf;

pub mod epub_export;
pub mod pdf_export;
pub mod txt_export;

// SPDX-License-Identifier: MIT
use crate::models::issue::Issue;
use crate::phrack_issue_manager_error::PhrackIssueManagerError;

#[derive(ValueEnum, Clone, Debug, Sequence)]
#[value(rename_all = "lowercase")]
pub enum ExportFormat {
    #[value(alias = "text")]
    TXT,
    PDF,
    EPUB,
}
pub struct ExportOptions {
    // Output folder for the exported file
    pub output_folder: PathBuf,
    pub issues_folder: PathBuf,
}

pub trait Exporter {
    fn export_all(&self, options: &ExportOptions) -> Result<(), PhrackIssueManagerError> {
        Issue::all_issues()?
            .into_iter()
            .try_for_each(|issue| self.export(issue, options))
    }
    fn export(&self, issue: Issue, options: &ExportOptions) -> Result<(), PhrackIssueManagerError>;

    fn get_article_paths(
        &self,
        issue: &Issue,
        options: &ExportOptions,
    ) -> Result<Vec<PathBuf>, PhrackIssueManagerError> {
        let issue_path = options
            .issues_folder
            .join(format!("{}", issue.issue_number));

        let mut articles: Vec<_> = fs::read_dir(&issue_path)?
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "txt"))
            .map(|e| e.path())
            .collect();

        articles.sort_by_key(|path| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .and_then(|s| s.parse::<u32>().ok())
                .unwrap_or(u32::MAX)
        });

        Ok(articles)
    }
}
