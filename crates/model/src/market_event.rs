/// Тип рыночного или корпоративного события.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    /// Финансовый отчет или операционные результаты.
    Earnings,
    /// Дивидендное событие: рекомендация, отсечка, выплата, отмена.
    Dividend,
    /// Новость из СМИ или агрегатора.
    News,
    /// Официальное раскрытие информации.
    Disclosure,
    /// Макроэкономическое событие: ставка, инфляция, регуляторика.
    Macro,
    /// Другое событие, которое пока не выделено в отдельный тип.
    Other,
}

/// Событие, которое может повлиять на инвестиционную или торговую идею.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketEvent {
    /// Категория события.
    event_type: EventType,
    /// Короткий заголовок события.
    title: String,
    /// Дополнительное описание, если заголовка недостаточно.
    description: Option<String>,
}

impl MarketEvent {
    pub fn new(event_type: EventType, title: String) -> Self {
        Self {
            event_type,
            title,
            description: None,
        }
    }

    pub fn event_type(&self) -> EventType {
        self.event_type
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn market_event_keeps_type_title_and_optional_description() {
        let mut event = MarketEvent::new(EventType::Dividend, "Dividend announced".to_string());

        assert_eq!(event.event_type(), EventType::Dividend);
        assert_eq!(event.title(), "Dividend announced");
        assert_eq!(event.description(), None);

        event.set_description("Board recommended dividend payment".to_string());

        assert_eq!(
            event.description(),
            Some("Board recommended dividend payment")
        );
    }
}
