use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum SourceError {
    #[error("title cannot be empty")]
    EmptyTitle,
    #[error("publisher cannot be empty if exists")]
    EmptyPublisher,
    #[error("at least one of url, local_path or note must be set")]
    MissingSourceReference,
    #[error("url cannot be empty")]
    EmptyUrl,
    #[error("local_path cannot be empty")]
    EmptyLocalPath,
    #[error("note cannot be empty")]
    EmptyNote,
}

/// Тип источника информации.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceType {
    /// Отчетность или презентация компании.
    CompanyReport,
    /// Новостная статья.
    NewsArticle,
    /// Официальное раскрытие через биржу или раскрывающий центр.
    ExchangeDisclosure,
    /// Сайт компании.
    CompanyWebsite,
    /// Книга или учебный источник.
    Book,
    MarketData,
    /// Другой источник, который пока не выделен отдельно.
    Other,
}

#[derive(Debug)]
pub struct SourceOptions {
    pub url: Option<String>,
    pub local_path: Option<String>,
    pub publisher: Option<String>,
    pub note: Option<String>,
    pub published_at: Option<DateTime<Utc>>,
}

/// Источник, на котором основана заметка, событие или инвестиционная идея.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    id: Option<u64>,
    company_id: Option<u64>,
    /// Категория источника.
    source_type: SourceType,
    /// Название источника.
    title: String,
    /// Ссылка на источник, если она есть.
    url: Option<String>,
    local_path: Option<String>,
    published_at: Option<DateTime<Utc>>,
    /// Автор, издатель, компания или площадка публикации.
    publisher: Option<String>,
    note: Option<String>,
}

impl Source {
    pub fn new(
        source_type: SourceType,
        title: String,
        company_id: Option<u64>,
        opts: SourceOptions,
    ) -> Result<Self, SourceError> {
        if opts.url.is_none() && opts.local_path.is_none() && opts.note.is_none() {
            return Err(SourceError::MissingSourceReference);
        }

        if let Some(val) = &opts.publisher
            && val.trim().is_empty()
        {
            return Err(SourceError::EmptyPublisher);
        }

        Ok(Self {
            id: None,
            company_id,
            source_type,
            title: Source::string_not_empty(title, SourceError::EmptyTitle)?,
            url: Source::string_exists_not_empty(opts.url, SourceError::EmptyUrl)?,
            local_path: Source::string_exists_not_empty(
                opts.local_path,
                SourceError::EmptyLocalPath,
            )?,
            published_at: opts.published_at,
            publisher: opts.publisher,
            note: Source::string_exists_not_empty(opts.note, SourceError::EmptyNote)?,
        })
    }

    pub fn id(&self) -> Option<u64> {
        self.id
    }

    pub fn source_type(&self) -> SourceType {
        self.source_type
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    pub fn publisher(&self) -> Option<&str> {
        self.publisher.as_deref()
    }

    pub fn company_id(&self) -> Option<u64> {
        self.company_id
    }

    pub fn local_path(&self) -> Option<&str> {
        self.local_path.as_deref()
    }

    pub fn published_at(&self) -> Option<DateTime<Utc>> {
        self.published_at
    }

    pub fn note(&self) -> Option<&str> {
        self.note.as_deref()
    }
}

impl Source {
    fn string_not_empty(val: String, e: SourceError) -> Result<String, SourceError> {
        if val.trim().is_empty() {
            return Err(e);
        }

        Ok(val)
    }

    fn string_exists_not_empty(
        val: Option<String>,
        e: SourceError,
    ) -> Result<Option<String>, SourceError> {
        if let Some(v) = val {
            let s = Source::string_not_empty(v, e)?;

            return Ok(Some(s));
        }

        Ok(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options_with_url(url: &str) -> SourceOptions {
        SourceOptions {
            url: Some(url.to_string()),
            local_path: None,
            publisher: None,
            note: None,
            published_at: None,
        }
    }

    #[test]
    fn new_company_source_preserves_input_and_has_no_id() {
        let published_at = "2026-03-01T10:30:00Z"
            .parse::<DateTime<Utc>>()
            .expect("test timestamp must be valid");
        let options = SourceOptions {
            url: Some("https://example.com/report.pdf".to_string()),
            local_path: Some("reports/annual-2025.pdf".to_string()),
            publisher: Some("Example issuer".to_string()),
            note: Some("Audited annual report".to_string()),
            published_at: Some(published_at),
        };

        let source = Source::new(
            SourceType::CompanyReport,
            "Annual report 2025".to_string(),
            Some(42),
            options,
        )
        .expect("valid company source must be created");

        assert_eq!(source.id(), None);
        assert_eq!(source.company_id(), Some(42));
        assert_eq!(source.source_type(), SourceType::CompanyReport);
        assert_eq!(source.title(), "Annual report 2025");
        assert_eq!(source.url(), Some("https://example.com/report.pdf"));
        assert_eq!(source.local_path(), Some("reports/annual-2025.pdf"));
        assert_eq!(source.publisher(), Some("Example issuer"));
        assert_eq!(source.note(), Some("Audited annual report"));
        assert_eq!(source.published_at(), Some(published_at));
    }

    #[test]
    fn new_general_source_has_no_company() {
        let options = SourceOptions {
            url: None,
            local_path: Some("books/ai-engineering.pdf".to_string()),
            publisher: None,
            note: None,
            published_at: None,
        };

        let source = Source::new(
            SourceType::Book,
            "AI Engineering".to_string(),
            None,
            options,
        )
        .expect("valid general source must be created");

        assert_eq!(source.company_id(), None);
        assert_eq!(source.local_path(), Some("books/ai-engineering.pdf"));
        assert_eq!(source.published_at(), None);
    }

    #[test]
    fn note_is_sufficient_as_the_only_source_reference() {
        let options = SourceOptions {
            url: None,
            local_path: None,
            publisher: None,
            note: Some("Physical book, chapter 3".to_string()),
            published_at: None,
        };

        let result = Source::new(
            SourceType::Book,
            "Printed reference".to_string(),
            None,
            options,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn new_source_rejects_empty_title() {
        let result = Source::new(
            SourceType::CompanyReport,
            String::new(),
            Some(42),
            options_with_url("https://example.com/report.pdf"),
        );

        assert_eq!(result, Err(SourceError::EmptyTitle));
    }

    #[test]
    fn new_source_rejects_whitespace_only_title() {
        let result = Source::new(
            SourceType::CompanyReport,
            " \t\n".to_string(),
            Some(42),
            options_with_url("https://example.com/report.pdf"),
        );

        assert_eq!(result, Err(SourceError::EmptyTitle));
    }

    #[test]
    fn new_source_requires_at_least_one_reference() {
        let options = SourceOptions {
            url: None,
            local_path: None,
            publisher: None,
            note: None,
            published_at: None,
        };

        let result = Source::new(
            SourceType::CompanyReport,
            "Annual report 2025".to_string(),
            Some(42),
            options,
        );

        assert_eq!(result, Err(SourceError::MissingSourceReference));
    }

    #[test]
    fn publisher_is_not_a_source_reference() {
        let options = SourceOptions {
            url: None,
            local_path: None,
            publisher: Some("Example issuer".to_string()),
            note: None,
            published_at: None,
        };

        let result = Source::new(
            SourceType::CompanyReport,
            "Annual report 2025".to_string(),
            Some(42),
            options,
        );

        assert_eq!(result, Err(SourceError::MissingSourceReference));
    }

    #[test]
    fn new_source_rejects_whitespace_only_url() {
        let result = Source::new(
            SourceType::NewsArticle,
            "Market update".to_string(),
            None,
            options_with_url(" \t\n"),
        );

        assert_eq!(result, Err(SourceError::EmptyUrl));
    }

    #[test]
    fn new_source_rejects_whitespace_only_local_path() {
        let options = SourceOptions {
            url: None,
            local_path: Some(" \t\n".to_string()),
            publisher: None,
            note: None,
            published_at: None,
        };

        let result = Source::new(
            SourceType::CompanyReport,
            "Annual report 2025".to_string(),
            Some(42),
            options,
        );

        assert_eq!(result, Err(SourceError::EmptyLocalPath));
    }

    #[test]
    fn new_source_rejects_whitespace_only_note() {
        let options = SourceOptions {
            url: None,
            local_path: None,
            publisher: None,
            note: Some(" \t\n".to_string()),
            published_at: None,
        };

        let result = Source::new(
            SourceType::Book,
            "Printed reference".to_string(),
            None,
            options,
        );

        assert_eq!(result, Err(SourceError::EmptyNote));
    }

    #[test]
    fn new_source_rejects_whitespace_only_publisher() {
        let mut options = options_with_url("https://example.com/report.pdf");
        options.publisher = Some(" \t\n".to_string());

        let result = Source::new(
            SourceType::CompanyReport,
            "Annual report 2025".to_string(),
            Some(42),
            options,
        );

        assert_eq!(result, Err(SourceError::EmptyPublisher));
    }
}
