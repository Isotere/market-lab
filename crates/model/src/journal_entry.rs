use crate::{Company, Instrument, MarketEvent, Source};

/// Действие или решение, которое фиксируется в журнале.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionType {
    /// Решение купить или план покупки.
    Buy,
    /// Решение продать или план продажи.
    Sell,
    /// Решение наблюдать без сделки.
    Watch,
    /// Сознательный отказ от идеи.
    Skip,
}

/// Жизненный цикл записи в журнале.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JournalStatus {
    /// Черновик: мысль еще не готова к действию.
    Draft,
    /// План готов, но действие еще не совершено.
    Planned,
    /// Сделка или наблюдение активны.
    Open,
    /// Запись завершена и разобрана.
    Closed,
    /// Идея отменена до исполнения или стала неактуальной.
    Cancelled,
}

/// Эмоциональное состояние в момент решения.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmotionTag {
    /// Решение принято спокойно, по плану.
    Calm,
    /// Страх упустить движение.
    Fomo,
    /// Страх потери или неопределенности.
    Fear,
    /// Чрезмерная уверенность в своей правоте.
    Overconfidence,
    /// Эмоция не распознана или не записана точно.
    Unclear,
}

/// Ошибка процесса, которую нужно отслеживать статистически.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MistakeTag {
    /// Решение принято без заранее записанного плана.
    NoPlan,
    /// Критерий отмены идеи был сдвинут после входа.
    MovedInvalidation,
    /// Вход был поздним относительно собственного плана.
    LateEntry,
    /// Риск был завышен относительно правил.
    OversizedRisk,
    /// Триггер сделки был проигнорирован или подменен.
    IgnoredTrigger,
    /// Ошибки процесса не найдено.
    NoMistake,
}

/// Базовая запись журнала решений.
///
/// Хранит стабильные идентификаторы и условия решения, но не тянет за собой полный
/// объект компании. Полная картина собирается отдельно в `JournalEntryContext`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalEntry {
    /// Внутренний идентификатор записи после сохранения в БД.
    id: Option<u64>,
    /// Ссылка на компанию или эмитента.
    company_id: u64,
    /// Ссылка на конкретный инструмент, если решение относится к бумаге.
    instrument_id: Option<u64>,
    /// Тип действия: купить, продать, наблюдать или пропустить.
    action: ActionType,
    /// Текущий статус записи.
    status: JournalStatus,
    /// Тезис идеи: почему она вообще рассматривается.
    thesis: String,
    /// Триггер: что должно произойти, чтобы идея стала актуальной.
    trigger: String,
    /// Критерий отмены идеи.
    invalidation: String,
    /// Плановый риск в рублях.
    planned_risk_rub: Option<u64>,
    /// Плановый риск в единицах `R`.
    planned_r: Option<u8>,
    /// Эмоциональное состояние при решении.
    emotion_tag: Option<EmotionTag>,
    /// Ошибка процесса, если она есть.
    mistake_tag: Option<MistakeTag>,
    /// Урок после разбора идеи или сделки.
    lesson: Option<String>,
}

impl JournalEntry {
    pub fn new(
        company_id: u64,
        instrument_id: Option<u64>,
        action: ActionType,
        thesis: String,
        trigger: String,
        invalidation: String,
    ) -> Self {
        Self {
            id: None,
            company_id,
            instrument_id,
            action,
            status: JournalStatus::Draft,
            thesis,
            trigger,
            invalidation,
            planned_risk_rub: None,
            planned_r: None,
            emotion_tag: None,
            mistake_tag: None,
            lesson: None,
        }
    }

    pub fn id(&self) -> Option<u64> {
        self.id
    }

    pub fn company_id(&self) -> u64 {
        self.company_id
    }

    pub fn instrument_id(&self) -> Option<u64> {
        self.instrument_id
    }

    pub fn action(&self) -> ActionType {
        self.action
    }

    pub fn status(&self) -> JournalStatus {
        self.status
    }

    pub fn set_status(&mut self, status: JournalStatus) {
        self.status = status
    }

    pub fn thesis(&self) -> &str {
        &self.thesis
    }

    pub fn trigger(&self) -> &str {
        &self.trigger
    }

    pub fn invalidation(&self) -> &str {
        &self.invalidation
    }

    pub fn planned_risk_rub(&self) -> Option<u64> {
        self.planned_risk_rub
    }

    pub fn planned_r(&self) -> Option<u8> {
        self.planned_r
    }

    pub fn set_planned_risk(&mut self, planned_risk_rub: u64, planned_r: u8) {
        self.planned_risk_rub = Some(planned_risk_rub);
        self.planned_r = Some(planned_r);
    }

    pub fn emotion_tag(&self) -> Option<EmotionTag> {
        self.emotion_tag
    }

    pub fn set_emotion_tag(&mut self, emotion_tag: EmotionTag) {
        self.emotion_tag = Some(emotion_tag);
    }

    pub fn mistake_tag(&self) -> Option<MistakeTag> {
        self.mistake_tag
    }

    pub fn set_mistake_tag(&mut self, mistake_tag: MistakeTag) {
        self.mistake_tag = Some(mistake_tag);
    }

    pub fn lesson(&self) -> Option<&str> {
        self.lesson.as_deref()
    }

    pub fn set_lesson(&mut self, lesson: String) {
        self.lesson = Some(lesson);
    }
}

/// Полностью загруженный контекст журнальной записи.
///
/// Нужен для модулей, которым удобнее работать с готовым графом объектов без
/// дополнительных запросов: запись, компания, инструмент, события и источники.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalEntryContext {
    /// Базовая запись журнала.
    entry: JournalEntry,
    /// Загруженная компания.
    company: Company,
    /// Загруженный инструмент, если он есть.
    instrument: Option<Instrument>,
    /// События рынка, связанные с идеей.
    events: Vec<MarketEvent>,
    /// Источники, на которых основана идея.
    sources: Vec<Source>,
}

impl JournalEntryContext {
    pub fn new(
        entry: JournalEntry,
        company: Company,
        instrument: Option<Instrument>,
        events: Vec<MarketEvent>,
        sources: Vec<Source>,
    ) -> Self {
        Self {
            entry,
            company,
            instrument,
            events,
            sources,
        }
    }

    pub fn entry(&self) -> &JournalEntry {
        &self.entry
    }

    pub fn company(&self) -> &Company {
        &self.company
    }

    pub fn instrument(&self) -> Option<&Instrument> {
        self.instrument.as_ref()
    }

    pub fn events(&self) -> &[MarketEvent] {
        &self.events
    }

    pub fn sources(&self) -> &[Source] {
        &self.sources
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_journal_entry_starts_as_draft_decision_record() {
        let entry = JournalEntry::new(
            42,
            Some(1001),
            ActionType::Watch,
            "Dividend recovery thesis".to_string(),
            "Board recommends dividends".to_string(),
            "Dividend cancellation".to_string(),
        );

        assert_eq!(entry.id(), None);
        assert_eq!(entry.company_id(), 42);
        assert_eq!(entry.instrument_id(), Some(1001));
        assert_eq!(entry.action(), ActionType::Watch);
        assert_eq!(entry.status(), JournalStatus::Draft);
        assert_eq!(entry.thesis(), "Dividend recovery thesis");
        assert_eq!(entry.trigger(), "Board recommends dividends");
        assert_eq!(entry.invalidation(), "Dividend cancellation");
        assert_eq!(entry.planned_risk_rub(), None);
        assert_eq!(entry.emotion_tag(), None);
        assert_eq!(entry.mistake_tag(), None);
    }

    #[test]
    fn journal_entry_tracks_risk_tags_and_lesson() {
        let mut entry = JournalEntry::new(
            42,
            None,
            ActionType::Buy,
            "Strong report and acceptable valuation".to_string(),
            "Breakout after report".to_string(),
            "Close below support".to_string(),
        );

        entry.set_status(JournalStatus::Planned);
        entry.set_planned_risk(5_000, 1);
        entry.set_emotion_tag(EmotionTag::Calm);
        entry.set_mistake_tag(MistakeTag::NoMistake);
        entry.set_lesson("Plan was written before action".to_string());

        assert_eq!(entry.status(), JournalStatus::Planned);
        assert_eq!(entry.planned_risk_rub(), Some(5_000));
        assert_eq!(entry.planned_r(), Some(1));
        assert_eq!(entry.emotion_tag(), Some(EmotionTag::Calm));
        assert_eq!(entry.mistake_tag(), Some(MistakeTag::NoMistake));
        assert_eq!(entry.lesson(), Some("Plan was written before action"));
    }

    #[test]
    fn journal_entry_context_keeps_loaded_objects_separate_from_entry_identity() {
        let entry = JournalEntry::new(
            42,
            Some(1001),
            ActionType::Watch,
            "Dividend recovery thesis".to_string(),
            "Board recommends dividends".to_string(),
            "Dividend cancellation".to_string(),
        );
        let company = Company::new("GAZP", "Gazprom", "Integrated gas company".to_string());
        let instrument = Instrument::new(42, "GAZP", crate::InstrumentType::CommonShare);
        let event = MarketEvent::new(crate::EventType::Dividend, "Dividend announced".to_string());
        let source = Source::new(
            crate::SourceType::CompanyReport,
            "Issuer report".to_string(),
        );

        let context =
            JournalEntryContext::new(entry, company, Some(instrument), vec![event], vec![source]);

        assert_eq!(context.entry().company_id(), 42);
        assert_eq!(context.company().ticker(), "GAZP");
        assert_eq!(context.instrument().map(Instrument::ticker), Some("GAZP"));
        assert_eq!(context.events().len(), 1);
        assert_eq!(context.sources().len(), 1);
    }
}
