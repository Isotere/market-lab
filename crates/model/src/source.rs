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
    /// Другой источник, который пока не выделен отдельно.
    Other,
}

/// Источник, на котором основана заметка, событие или инвестиционная идея.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    /// Категория источника.
    source_type: SourceType,
    /// Название источника.
    title: String,
    /// Ссылка на источник, если она есть.
    url: Option<String>,
    /// Автор, издатель, компания или площадка публикации.
    publisher: Option<String>,
}

impl Source {
    pub fn new(source_type: SourceType, title: String) -> Self {
        Self {
            source_type,
            title,
            url: None,
            publisher: None,
        }
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

    pub fn set_url(&mut self, url: String) {
        self.url = Some(url);
    }

    pub fn publisher(&self) -> Option<&str> {
        self.publisher.as_deref()
    }

    pub fn set_publisher(&mut self, publisher: String) {
        self.publisher = Some(publisher);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_keeps_type_title_url_and_publisher() {
        let mut source = Source::new(SourceType::CompanyReport, "Annual report 2025".to_string());

        assert_eq!(source.source_type(), SourceType::CompanyReport);
        assert_eq!(source.title(), "Annual report 2025");
        assert_eq!(source.url(), None);
        assert_eq!(source.publisher(), None);

        source.set_url("https://example.com/report.pdf".to_string());
        source.set_publisher("Example issuer".to_string());

        assert_eq!(source.url(), Some("https://example.com/report.pdf"));
        assert_eq!(source.publisher(), Some("Example issuer"));
    }
}
