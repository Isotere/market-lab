/// Приоритет компании в личной системе анализа.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompanyPriority {
    /// Компания в основном списке для первичного анализа.
    Primary,
    /// Компания интересна, но пока только под наблюдением.
    Watch,
    /// Компания не подходит под текущие критерии или имеет низкий приоритет.
    Outsider,
}

/// Эмитент или бизнес, который анализируется в `market-lab`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Company {
    /// Внутренний идентификатор компании после сохранения в БД.
    id: Option<u64>,
    /// Основной тикер компании, удобный для поиска и отображения.
    ticker: String,
    /// Название компании.
    name: String,
    /// Короткое описание бизнеса простыми словами.
    description: String,
    /// Сильные стороны бизнеса или инвестиционного кейса.
    strengths: Vec<String>,
    /// Слабые стороны, риски и спорные места кейса.
    weaknesses: Vec<String>,
    /// Короткая заметка об оценке: дешево, дорого, справедливо или что проверить.
    valuation_note: Option<String>,
    /// Текущий приоритет компании в личном анализе.
    priority: CompanyPriority,
}

impl Company {
    pub fn new(ticker: &str, name: &str, description: String) -> Self {
        Company {
            id: None,
            ticker: ticker.to_string(),
            name: name.to_string(),
            description,
            strengths: Vec::new(),
            weaknesses: Vec::new(),
            valuation_note: None,
            priority: CompanyPriority::Watch,
        }
    }

    pub fn id(&self) -> Option<u64> {
        self.id
    }

    pub fn ticker(&self) -> &str {
        &self.ticker
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn strengths(&self) -> &[String] {
        &self.strengths
    }

    pub fn weaknesses(&self) -> &[String] {
        &self.weaknesses
    }

    pub fn valuation_note(&self) -> Option<&str> {
        self.valuation_note.as_deref()
    }

    pub fn priority(&self) -> CompanyPriority {
        self.priority
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_company_has_identity_and_default_priority() {
        let company = Company::new("SBER", "Sberbank", "Largest Russian bank".to_string());

        assert_eq!(company.id(), None);
        assert_eq!(company.ticker(), "SBER");
        assert_eq!(company.name(), "Sberbank");
        assert_eq!(company.description(), "Largest Russian bank");
        assert_eq!(company.priority(), CompanyPriority::Watch);
        assert!(company.strengths().is_empty());
        assert!(company.weaknesses().is_empty());
        assert_eq!(company.valuation_note(), None);
    }
}
