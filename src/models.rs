use std::fmt;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Response<T> {
    pub error: u8,
    pub message: String,
    pub code: Option<u16>,
    pub data: Option<T>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct EmptyResponse {
    pub error: u8,
    pub message: String,
    pub code: Option<u16>,
}

/// Die Kategorie ist eine simple Unterscheidung zwischen Anime und Manga.
///
/// # Description
///
/// * `Anime` - Ist ein Anime
/// * `Manga` - Ist ein Managa
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum Kategorie {
    #[serde(rename = "anime")] Anime,
    Manga,
}

impl fmt::Display for Kategorie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Kategorie::Anime => "anime",
            Kategorie::Manga => "manga",
        };
        write!(f, "{}", printable)
    }
}

/// Gibt an um was für eine Medienquelle es sich handelt.
///
/// # Description
///
/// * `Animeseries` - Das Medium ist eine Animeserie
/// * `Movie` - Das Medium ist ein Movie
/// * `OVA` - Das Medium ist ein OVA
/// * `Hentai` - Das Medium ist ein Hentai
/// * `Mangaseries` - Das Medium ist ein Managa/Managserie
/// * `Oneshot` - Das Medium ist Oneshot
/// * `Doujin` - Das Medium ist ein Doujin
/// * `Hmanga` - Das Medium ist ein Hmanaga
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum Medium {
    Animeseries,
    Movie,
    OVA,
    Hentai,
    Mangaseries,
    Oneshot,
    Doujin,
    Hmanga,
}

impl fmt::Display for Medium {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Medium::Animeseries => "animeseries",
            Medium::Movie => "movie",
            Medium::OVA => "ova",
            Medium::Hentai => "hentai",
            Medium::Mangaseries => "mangaseries",
            Medium::Oneshot => "onshot",
            Medium::Doujin => "doujin",
            Medium::Hmanga => "hmanga",
        };
        write!(f, "{}", printable)
    }
}

///	Nach was die angefragte List sortiert werden soll.
///
/// # Description
///
/// Dieser Parameter gibt an, wie die Liste sortiert werden soll,
/// erlaubte Eingaben (Fehlerhafte Eingaben werden auf den Default-Wert gezwungen):
///
/// * `nameASC` - Sortierung nach Entry-Name Aufsteigend
/// * `nameDESC` - Sortierung nach Entry-Name Absteigend
/// * `stateNameASC` - Sortierung nach Status des Entrys, dann Entry-Name Aufsteigend (default Wert)
/// * `stateNameDESC` - Sortierung nach Status des Entrys, dann Entry-Name Absteigend
/// * `changeDateASC` - Sortierung nach letzter Änderung Aufsteigend
/// * `changeDateDESC` - Sortierung nach letzter Änderung Absteigend
/// * `stateChangeDateASC` - Sortierung nach Status des Entrys, dann letzter Änderung Aufsteigend
/// * `stateChangeDateDESC` - Sortierung nach Status des Entrys, dann letzter Änderung Absteigend
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum Sort {
    NameASC,
    NameDESC,
    StateNameASC,
    StateNameDESC,
    ChangeDateASC,
    ChangeDateDESC,
    StateChangeDateASC,
    StateChangeDateDESC,
}

impl fmt::Display for Sort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Sort::NameASC => "nameASC",
            Sort::NameDESC => "nameDESC",
            Sort::StateNameASC => "stateNameASC",
            Sort::StateNameDESC => "stateNameDESC",
            Sort::ChangeDateASC => "changeDateASC",
            Sort::ChangeDateDESC => "changeDateDESC",
            Sort::StateChangeDateASC => "stateChangeDateASC",
            Sort::StateChangeDateDESC => "stateChangeDateDESC",
        };
        write!(f, "{}", printable)
    }
}

/// Gibt an durch welche Firma der Anime, Managa oder etc. veröffenlicht wurde.
///
/// #Description
///
/// Eignetlich selbst erklärend, dennoch:
///
/// * `Publisher` - Der Publisher des Animes/Mangas
/// * `Studio` - Das Studio des Animes/Mangas
/// * `Producer` -  Ein Producer
/// * `RecordLabel` -  Ein RecordLabel
/// * `TalentAgent` -  Eine TalentAgent
/// * `Streaming` -  Eine Streaming Seite eins Animes/Mangas
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum Firma {
    Publisher,
    Studio,
    Producer,
    RecordLabel,
    TalentAgent,
    Streaming,
}

impl fmt::Display for Firma {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Firma::Publisher => "publisher",
            Firma::Studio => "studio",
            Firma::Producer => "producer",
            Firma::RecordLabel => "record_label",
            Firma::TalentAgent => "talent_agent",
            Firma::Streaming => "streaming",
        };
        write!(f, "{}", printable)
    }
}

/// Gibt die Sprache und die Übermittlung der Sprache an.
/// (Über Untertitel oder Synchronisierung)
///
/// #Description
///
/// * `GerSub` - German Subtitles
/// * `GerDub` - German Synchronized
/// * `EngSub` - English Subtitles
/// * `EngDub` - English Synchronized
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum Language {
    GerSub,
    GerDub,
    EngSub,
    EngDub,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Language::GerSub => "gersub",
            Language::GerDub => "gerdub",
            Language::EngSub => "engsub",
            Language::EngDub => "engdub",
        };
        write!(f, "{}", printable)
    }
}

/// Beschreibt ob das Medium noch geschaut wird, Favoriesiert ist oder Abgeschlossen ist.
///
/// #Description
///
/// * `Note` - Wird noch geschaut
/// * `Favor` - Favoriten (Favorisiert)
/// * `Finish` - Abgeschlossen
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum WatchType {
    Note,
    Favor,
    Finish,
}

impl fmt::Display for WatchType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            WatchType::Note => "note",
            WatchType::Favor => "favor",
            WatchType::Finish => "finish",
        };
        write!(f, "{}", printable)
    }
}
/// Nach welchem Parameter bei der Suche sotiert werden soll.
///
/// # Description
///
/// Wie die Ergebnisse sortiert werden sollen. Erlaubte Werte:
///
/// * `relevance` - Qualität des Name-Matches, Default
/// * `clicks` - Sotierung nach Clicks
/// * `rating` - Sortierung erst nach Anzahl und dann Wertung der Stimmen
/// * `count` - Anzahl der Kapitel/Episoden
/// * `name` - Alphabetisch
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum SearchSort {
    Relevance,
    Clicks,
    Count,
    Name,
}

impl fmt::Display for SearchSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            SearchSort::Relevance => "relevance",
            SearchSort::Clicks => "clicks",
            SearchSort::Count => "count",
            SearchSort::Name => "name",
        };
        write!(f, "{}", printable)
    }
}

/// Ob der Parameter "length" als Minimal- oder Maximalwert verwendet werden soll.
///
/// # Description
///
/// Erlaubte Werte:
///
/// * `up` - größer gleich
/// * `down` - kleiner gleich, Default
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum LengthLimit {
    Up,
    Down,
}

impl fmt::Display for LengthLimit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            LengthLimit::Up => "up",
            LengthLimit::Down => "down",
        };
        write!(f, "{}", printable)
    }
}

/// (Betrifft nur Tags vom Typ entry_tag, alle anderen Typen haben immer Kategorie "Sonstige").
/// Default: Alle. Erlaubte Werte:
///
/// # Description
///
/// * `misc` - Sonstige
/// * `persoenlichkeiten` - Persönlichkeiten
/// * `gefuehle` - Gefühle und menschliche Angewohnheiten
/// * `zeichnung` - Zeichnung/Animation
/// * `uebernatuerliches` - Übernatürliches
/// * `sport` - Sport
/// * `menschen` - Menschen
/// * `zukunft` - Zukunft und Technik
/// * `story` - Story-Inhalte
/// * `prota` - Protagonist
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum SubType {
    Misc,
    Persoenlichkeiten,
    Gefuehle,
    Zeichungen,
    Uebernatuerliches,
    Sport,
    Menschen,
    Zukunft,
    Story,
    Protagonist,
}

impl fmt::Display for SubType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            SubType::Misc => "misc",
            SubType::Persoenlichkeiten => "persoenlichkeiten",
            SubType::Gefuehle => "gefuehle",
            SubType::Zeichungen => "zeichnung",
            SubType::Uebernatuerliches => "uebernatuerliches",
            SubType::Sport => "sport",
            SubType::Menschen => "menschen",
            SubType::Zukunft => "zukunft",
            SubType::Story => "story",
            SubType::Protagonist => "prota",
        };
        write!(f, "{}", printable)
    }
}

/// Gibt den Stand der Übersetzung an.
///
/// # Description
///
/// * `undefined` - Nicht definierter Zustand
/// * `abgeschlossen` - Abgeschlossenes Projekt
/// * `am laufen` - Das Projekt wird derzeit übersetzt.
/// * `geplant` - Ein Projekt das auf der TODO Liste steht.
/// * `abgebrochen` - Abgebrochenes Projekt
/// * `lizenziert` - Lizensiertes Projekt
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum TranslationStatus {
    Undefined,
    Abgeschlossen,
    AmLaufen,
    Geplant,
    Abgebrochen,
    Lizensiert,
}

impl fmt::Display for TranslationStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            TranslationStatus::Undefined => "0",
            TranslationStatus::Abgeschlossen => "1",
            TranslationStatus::AmLaufen => "2",
            TranslationStatus::Geplant => "3",
            TranslationStatus::Abgebrochen => "4",
            TranslationStatus::Lizensiert => "5",
        };
        write!(f, "{}", printable)
    }
}

/// Beschreibt die Konferenz genauer.
///
/// # Description
///
///	* `favour` - Favorisierte Konferenzen.
///	* `block` - Blockierte Konferenzen.
///	* `group` - Listet ausschließlich Gruppenkonferenzen auf.
///	* `default` - Listet unblockierte Konferenzen.
#[derive(Deserialize, Clone, Debug, PartialEq)]
pub enum ConferenceOption {
    Favour,
    Block,
    Group,
    Default,
}

impl fmt::Display for ConferenceOption {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            ConferenceOption::Favour => "favor",
            ConferenceOption::Block => "block",
            ConferenceOption::Group => "group",
            ConferenceOption::Default => "default",
        };
        write!(f, "{}", printable)
    }
}
