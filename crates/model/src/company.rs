use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CompanyError {
    #[error("ticker cannot be empty")]
    EmptyTicker,

    #[error("company name cannot be empty")]
    EmptyName,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Market {
    Moex,
    Spb,
    Nasdaq,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompanyStatus {
    Watch,
    Active,
    Archive,
}

/// Эмитент или бизнес, который анализируется в `market-lab`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Company {
    id: Option<u64>,
    ticker: String,
    name: String,
    market: Market,
    sector: Option<String>,
    status: CompanyStatus,
    note: Option<String>,
}

impl Company {
    pub fn new(ticker: &str, name: &str, market: Market) -> Result<Company, CompanyError> {
        if ticker.trim().is_empty() {
            return Err(CompanyError::EmptyTicker);
        }

        if name.trim().is_empty() {
            return Err(CompanyError::EmptyName);
        }

        Ok(Company {
            id: None,
            ticker: ticker.to_string(),
            name: name.to_string(),
            market,
            sector: None,
            status: CompanyStatus::Watch,
            note: None,
        })
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

    pub fn market(&self) -> Market {
        self.market
    }

    pub fn status(&self) -> CompanyStatus {
        self.status
    }

    pub fn sector(&self) -> Option<&str> {
        self.sector.as_deref()
    }

    pub fn note(&self) -> Option<&str> {
        self.note.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_company_keeps_identity_and_uses_default_state() {
        let company = Company::new("SBER", "Sberbank", Market::Moex).unwrap();

        assert_eq!(company.id(), None);
        assert_eq!(company.ticker(), "SBER");
        assert_eq!(company.name(), "Sberbank");
        assert_eq!(company.market(), Market::Moex);
        assert_eq!(company.sector(), None);
        assert_eq!(company.status(), CompanyStatus::Watch);
        assert_eq!(company.note(), None);
    }

    #[test]
    fn new_company_rejects_empty_ticker() {
        let result = Company::new("", "Sberbank", Market::Moex);

        assert_eq!(result, Err(CompanyError::EmptyTicker));
    }

    #[test]
    fn new_company_rejects_whitespace_only_ticker() {
        let result = Company::new(" \t\n", "Sberbank", Market::Moex);

        assert_eq!(result, Err(CompanyError::EmptyTicker));
    }

    #[test]
    fn new_company_rejects_empty_name() {
        let result = Company::new("SBER", "", Market::Moex);

        assert_eq!(result, Err(CompanyError::EmptyName));
    }

    #[test]
    fn new_company_rejects_whitespace_only_name() {
        let result = Company::new("SBER", " \t\n", Market::Moex);

        assert_eq!(result, Err(CompanyError::EmptyName));
    }
}
