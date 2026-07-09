/// Тип финансового инструмента, связанного с компанией или эмитентом.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstrumentType {
    /// Обыкновенная акция.
    CommonShare,
    /// Привилегированная акция.
    PreferredShare,
    /// Облигация.
    Bond,
    /// Биржевой фонд.
    Etf,
    /// Другой тип инструмента, который пока не выделен отдельно.
    Other,
}

/// Конкретная торгуемая бумага или инструмент.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instrument {
    /// Внутренний идентификатор инструмента после сохранения в БД.
    id: Option<u64>,
    /// Стабильная ссылка на компанию или эмитента.
    company_id: u64,
    /// Тикер конкретного инструмента.
    ticker: String,
    /// Тип инструмента: акция, преф, облигация, ETF или другое.
    instrument_type: InstrumentType,
}

impl Instrument {
    pub fn new(company_id: u64, ticker: &str, instrument_type: InstrumentType) -> Self {
        Self {
            id: None,
            company_id,
            ticker: ticker.to_string(),
            instrument_type,
        }
    }

    pub fn id(&self) -> Option<u64> {
        self.id
    }

    pub fn company_id(&self) -> u64 {
        self.company_id
    }

    pub fn ticker(&self) -> &str {
        &self.ticker
    }

    pub fn instrument_type(&self) -> InstrumentType {
        self.instrument_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_instrument_links_to_company_by_id() {
        let instrument = Instrument::new(42, "SBER", InstrumentType::CommonShare);

        assert_eq!(instrument.id(), None);
        assert_eq!(instrument.company_id(), 42);
        assert_eq!(instrument.ticker(), "SBER");
        assert_eq!(instrument.instrument_type(), InstrumentType::CommonShare);
    }
}
