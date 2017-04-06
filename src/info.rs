use serde_json;

use ::error::Error;
use ::Proxer;
use ::models::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullEntry
{
	/// Die ID des Entrys
	pub id: u64,
	/// Der Originaltitel des Entrys
	pub name: String,
	/// Die Genre des Entrys (Ein durch Leerzeichen getrennter String, jedes Wort ist ein Genre)
	pub genre: String,
	/// Die Gefahrensymbole des Entrys (Ein durch Leerzeichen getrennter String, jedes Wort ist ein Eintrag)
	pub fsk: String,
	/// Die Beschreibung des Entrys
	pub description:String,
	/// Das Medium des Entrys (animeseries,movie,ova,hentai,mangaseries,oneshot,doujin,hmanga)
	pub medium: Medium,
	/// Die Anzahl der Kapitel/Episoden (Müssen nicht hochgeladen sein!)
	pub count: u64,
	/// Ein Integer-Wert, der den Status des Entrys beschreibt (0 = x....)
	pub state: u8,
	/// Die Summe aller Bewertungen
	pub rate_sum: u64,
	/// Die Anzahl der Bewertungen
	pub rate_count: u64,
	/// Die Anzahl der Klicks, die ein Anime/Manga bekommen hat. (Wird alle 3 Monate resettet)
	pub clicks: u64,
	/// Die Kategorie des Entrys ("anime" oder "manga")
	pub kat: Kategorie,
	/// 0= Unbekannt, 1 = Nicht lizenziert, 2 = lizenziert
	pub license: u8,
	/// Ein boolean-Wert, der anzeigt, ob der Anime ab 18 ist oder nicht. (Ist dieser Wert "true", so sollte das Alter des Users geprüft werden)
	pub gate: bool,
	/// Ein Array von Objekten der Synonyme des Entrys
	pub names: Vec<String>,
	/// Die Sprachen, in denen der Anime verfügbar ist. (Ein Array von Strings, Werte: "de" => Deutsch, "en" => Englisch)
	pub lang: Vec<String>,
	/// Die Seasons des Animes (Array von Objekten)
	pub seasons: Vec<FullEntryDataSeasons>,
	/// Die eingetragenen Subgruppen (Array von Objekten)
	pub groups: Vec<FullEntryDataGroups>,
	/// Die Industrie-Einträge (Array von Objekten)
	pub publisher: Vec<FullEntryDataPublisher>,
	/// Die Tags (Array von Objekten)
	pub tags: Vec<FullEntryDataTags>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullEntryDataSeasons
{
	/// Die ID des Eintrags
	pub id: u64,
	/// ... (Nicht näher definiert)
	#[serde(rename="type")]
	pub entry_type: String,
	/// Das Jahr der Season
	pub year: i32,
	/// Eine Zahl: 0=Keine Season,1...4=winter, fühling, sommer, herbst
	pub season: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullEntryDataGroups
{
	/// Die ID der Gruppe
	pub id: u64,
	/// Der Name der Gruppe
	pub name: String,
	/// Die Sprache der Gruppe
	pub country: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullEntryDataPublisher
{
	/// Die ID des Publishers
	pub id: u64,
	/// Der Name des Publishers
	pub name: String,
	/// Der Typ des Publishers
	#[serde(rename="type")]
	pub entry_type: String,
	/// Die Sprache des Publishers
	pub country: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullEntryDataTags
{
	/// Die ID des Entrytags (NICHT die ID des Tags, sondern der Verknüpfung von Tag und Entry)
	pub id: u64,
	/// Die ID des Tags
	pub tid: u64,
	/// Der Zeitpunkt, zu dem der Tag in den Entry eingetragen wurde. (Format: YYYY-MM-DD HH:ii:ss)
	pub timestamp: String,
	/// Ob der Tag zum Entry passt (0 = Unbestimmt, 1 = passt zum Entry, unpassende Tags werden nicht ausgegeben)
	pub rate_flag: u8,
	/// Ob der Tag ein Spoiler ist (0 = Kein Spoiler, 1 = Spoiler).
	/// WARNUNG: "Kein Spoiler" ist der Default-Wert. Wenn also ein Tag noch nicht genug gevoted wurde,
	/// um ein Spoiler zu sein, obwohl er es ist, so hat er trotzdem den Wert 0.
	/// Deswegen ist es sinnvoll, alle "unbestimmten" Tags wie Spoiler zu behandeln.
	/// (Bzw. ihn zwar getrennt von den Spoilern anzuzeigen, aber nicht sofort offensichtlich zu machen)
	pub spoiler_flag: u8,
	/// Der Name des Tags
	pub tag: String,
	/// Die Beschreibung des Tags
	pub description: String,
}

/// Diese Funktion liefert die Daten eines Animes/Mangas anhand seiner ID (Dies bezieht sich NUR auf die Kerndaten des Anime,
/// nicht die Daten, die über die folgenden Funktionen abgefragt werden können)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry
{
	/// Die ID des Entrys
	pub id: u64,
	/// Der Originaltitel des Entrys
	pub name: String,
	/// Die Genre des Entrys (Ein durch Leerzeichen getrennter String, jedes Wort ist ein Genre)
	pub genre: String,
	/// Die Gefahrensymbole des Entrys (Ein durch Leerzeichen getrennter String, jedes Wort ist ein Eintrag)
	pub fsk: String,
	/// Die Beschreibung des Entrys
	pub description: String,
	/// Das Medium des Entrys (animeseries,movie,ova,hentai,mangaseries,oneshot,doujin,hmanga)
	pub medium: Medium,
	/// Die Anzahl der Kapitel/Episoden (Müssen nicht hochgeladen sein!)
	pub count: u64,
	/// Ein Integer-Wert, der den Status des Entrys beschreibt (0 = x....)
	pub state: u64,
	/// Die Summe aller Bewertungen
	pub rate_sum: u64,
	/// Die Anzahl der Bewertungen
	pub rate_count: u64,
	/// Die Anzahl der Klicks, die ein Anime/Manga bekommen hat. (Wird alle 3 Monate resettet)
	pub clicks: u64,
	/// Die Kategorie des Entrys ("anime" oder "manga")
	pub kat: Kategorie,
	/// 0= Unbekannt, 1 = Nicht lizenziert, 2 = lizenziert
	pub license: u8,
}

/// Diese Funktion liefert die unterschiedlichen Synonyme eines Animes/Mangas anhand seiner ID
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Name
{
	/// Die ID des Namens
	pub id: u64,
	/// Die ID des Entrys
	pub eid: u64,
	/// Der Typ des Namens (...)
	#[serde(rename="type")]
	pub name_type: String,
	/// Der Name.
	pub name: String,
}

/// Diese Funktion liefert die für einen Anime/Manga Eingetragenen Seasons.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Season
{
	/// Die ID des Eintrags
	pub id: u64,
	/// Die ID des Entrys
	pub eid: u64,
	/// ...
	#[serde(rename="type")]
	pub season_type: String,
	/// Das Jahr der Season
	pub year: i32, // time crate saves years, min etc as i32,
	/// Eine Zahl: 0=Keine Season,1...4=winter, fühling, sommer, herbst
	pub season: u8,
}

/// Diese Funktion liefert die für einen Anime/Manga Eingetragenen Übersetzergruppen.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group
{
	/// Die ID der Gruppe
	pub id: u64,
	/// Der Name der Gruppe
	pub name: String,
	/// Die Sprache der Gruppe
	pub country: String,
}

/// Diese Funktion liefert die für einen Anime/Manga Eingetragenen Publisher.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Publisher
{
	/// Die ID des Publishers
	pub id: u64,
	/// Der Name des Publishers
	pub name: String,
	/// Der Typ des Publishers
	#[serde(rename="type")]
	pub publisher_type: String,
	/// Die Sprache des Publishers
	pub country: String,
}

/// Diese Funktion liefert eine Liste aller Episoden/Kapitel eines Entrys anhand dessen ID.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListInfo
{
	/// Die Nummer des ersten Kapitels
	pub start: u64,
	/// Die Nummer des letzten Kapitels
	pub end: u64,
	/// Die Kategorie des Entrys ("anime" oder "manga")
	pub kat: Kategorie,
	/// Ein Array von Strings, das die vorhanden Sprachen enthält
	pub lang: String,
	/// Dieser Wert stellt den momentanen Stand des Users bei diesem Anime dar.
	/// Wenn kein User eingeloggt ist, so ist dieser Wert 0.
	pub state: u64,
	/// Die Daten der einzelnen Episoden/Kapitel.
	/// (Array von Objekten, erst sortiert nach Sprache, dann nach Episodennummer).
	/// Diese enthalten abhängig von der Kategorie unterschiedliche Werte:
	pub episodes: Vec<ListInfoDataEpisode>,

}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListInfoDataEpisode
{
	/// Die Nummer des Kapitels/Episode
	pub no: u64,
	/// Der Titel des Kapitels
	pub title: Option<String>,
	/// Die Sprache des Kapitels/Episode
	pub typ: String,
	/// Ein Komma-separierter String von Typen (vorhandene Hoster)
	pub types: Option<String>,
	/// Ein Komma-separierter String von Bildlinks der Hoster
	pub typeimg: Option<String>,
}

/// Diese Funktion liefert die für einen Anime/Manga abgegebenen Kommentare (mit mehr als 300 Zeichen).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment
{
	/// Die ID des Kommentars
	pub id: u64,
	/// Die ID des Entrys
	pub tid: u64,
	/// Der Typ des Kommentars
	#[serde(rename="type")]
	pub comment_type: String,
	/// Der beim Kommentar angegebene Status (0 = geschaut, 1 = am schauen, 2 = wird geschaut, 3 = abgebrochen)
	pub state: u8,
	/// Zusätzliche informationen, die als json-String abgespeichert werden.
	/// Es kann sich hierbei beispielsweise um die "Genre"-Bewertung handeln.
	pub data: String,
	/// Der Kommentar-Text
	pub comment: String,
	/// Die Bewertung des Entrys
	pub rating: f32,
	/// Die Episode, bis zu der der Kommentarschreiber geschaut hat.
	pub episode: 	u64,
	/// Wie viele Personen den Kommentar hilfreich finden.
	pub positive: u64,
	/// Der Zeitpunkt der letzten Bearbeitung/Erstellung (?) des Kommentars (Unix-Timestamp in Sekunden)
	pub timestamp: i64,
	/// Der Username des Erstellers des Kommentars
	pub username: String,
	/// Die User-ID des Erstellers des Kommentars
	pub uid: u64,
	/// Das Profilbild des Erstellers des Kommentars
	pub avatar: String,
}

/// Diese Funktion liefert alle Verbindungen eines Entrys.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Relation
{
	/// Die ID des Entrys
	pub id: u64,
	/// Der Originaltitel des Entrys
	pub name: String,
	/// Die Genre des Entrys (Ein durch Leerzeichen getrennter String, jedes Wort ist ein Genre)
	pub genre: String,
	/// Die Gefahrensymbole des Entrys (Ein durch Leerzeichen getrennter String, jedes Wort ist ein Eintrag)
	pub fsk: String,
	/// Die Beschreibung des Entrys
	pub description: String,
	/// Das Medium des Entrys (animeseries,movie,ova,hentai,mangaseries,oneshot,doujin,hmanga)
	pub medium: Medium,
	/// Die Anzahl der Kapitel/Episoden (Müssen nicht hochgeladen sein!)
	pub count: u64,
	/// Ein Integer-Wert, der den Status des Entrys beschreibt (0 = x....)
	pub state: u64,
	/// Die Summe aller Bewertungen
	pub rate_sum: u64,
	/// Die Anzahl der Bewertungen
	pub rate_count: u64,
	/// Die Anzahl der Klicks, die ein Anime/Manga bekommen hat. (Wird alle 3 Monate resettet)
	pub clicks: u64,
	/// Die Kategorie des Entrys ("anime" oder "manga")
	pub kat: Kategorie,
	/// 0= Unbekannt, 1 = Nicht lizenziert, 2 = lizenziert
	pub license: u8,
	/// Die Sprachen, in denen der Entry verfügbar ist, als Komma-separierter String
	pub language: String,
	/// Das Jahr der Season
	pub year: i32,
	/// Die Season
	pub season: u8,
}

/// Diese Funktion liefert alle Tags eines Entrys.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntryTag
{
	/// Die ID des Entrytags (NICHT die ID des Tags, sondern der Verknüpfung von Tag und Entry)
	pub id: u64,
	/// Die ID des Tags
	pub tid: u64,
	/// Der Zeitpunkt, zu dem der Tag in den Entry eingetragen wurde. (Format: YYYY-MM-DD HH:ii:ss)
	pub timestamp: i64,
	/// Ob der Tag zum Entry passt (0 = Unbestimmt, 1 = passt zum Entry, unpassende Tags werden nicht ausgegeben)
	pub rate_flag: u8,
	/// Ob der Tag ein Spoiler ist (0 = Kein Spoiler, 1 = Spoiler). WARNUNG: "Kein Spoiler" ist der Default-Wert. Wenn also ein Tag noch nicht genug gevoted wurde, um ein Spoiler zu sein, obwohl er es ist, so hat er trotzdem den Wert 0. Deswegen ist es sinnvoll, alle "unbestimmten" Tags wie Spoiler zu behandeln. (Bzw. ihn zwar getrennt von den Spoilern anzuzeigen, aber nicht sofort offensichtlich zu machen)
	pub spoiler_flag: u8,
	/// Der Name des Tags
	pub tag: String,
	/// Die Beschreibung des Tags
	pub description: String,
}

/// Diese Funktion liefert alle Daten zu einer Sub/Scanlation Gruppe anhand ihrer ID.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranslatorGroup
{
	/// Die id der Gruppe
	pub id: u64,
	/// Der Name der Gruppe
	pub name: String,
	/// Der Link zur Website der Gruppe
	pub link: String,
	/// Die Sprache der Gruppe (misc, de, en)
	pub country: String,
	/// Ein Bildlink für ein Bild der Gruppe (kann leer sein)
	pub image: Option<String>,
	/// Die Beschreibung der Gruppe
	pub description: String,
	/// ...
	pub count: String,
	/// ...
	pub cprojects: String,
}

/// Diese Funktion liefert alle Daten zu einer Firma anhand ihrer ID.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Industry
{
	/// Die id der Gruppe
	pub id: u64,
	/// Der Typ der Firma (publisher, studio, producer, record_label, talent_agent, streaming)
	#[serde(rename="type")]
	pub industry_type: Firma,
	/// Der Name der Firma
	pub name: String,
	/// Das Land der Firma (misc, de, us (England/USA), jp)
	pub country: String,
	/// Ein Link zur Website der Firma
	pub link: String,
	/// Die Beschreibung der Firma
	pub description: String,
}

impl Industry
{
	/// Folgender Link enthält das Coverbild einer Firma: https://cdn.proxer.me/industry/<id>.jpg
	pub fn get_industry_cover(&self)
	-> String
	{
		format!("https://cdn.proxer.me/industry/{}.jpg", self.id)
	}
}

/// Diese Klasse beinhaltet alle Schnittstellen, die sich auf das Info-System Proxers beziehen,
/// insbesondere in Bezug auf Informationen zu Animes und Mangas, aber auch zu den weiteren Verzeichnissen.
#[derive(Debug)]
pub struct Info<'info>
{
	proxer: &'info Proxer,
}

impl<'info> Info<'info>
{
	#[doc(hidden)]
	pub fn new(p_proxer: &'info Proxer)
	-> Info<'info>
	{
		Info
		{
			proxer: p_proxer,
		}
	}

	/// Diese Funktion liefert ALLE Daten eines Animes/Mangas anhand seiner ID.
	/// Diese Schnittstelle belastet den Server stark und sollte nur verwendet werden,
	/// wenn auch wirklich ALLE Informationen des Animes benötigt werden (wie etwa bei Detailseiten).
	///
	/// # Arguments
	///
	/// * `id` - Die ID des gewünschten Animes/Mangas.
	pub fn get_fullentry(&self, p_id: u64)
	-> Result<FullEntry, Error>
	{
		let url = url!("info", "fullentry");
		let body = param_build!("api_key" => Some(&self.proxer.api_key), "id" => Some(p_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<FullEntry> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert die Daten eines Animes/Mangas anhand seiner ID
	/// (Dies bezieht sich NUR auf die Kerndaten des Anime, nicht die Daten,
	/// die über die folgenden Funktionen abgefragt werden können)
	///
	/// # Arguments
	///
	/// * `id` - Die ID des gewünschten Animes/Mangas.
	pub fn get_entry(&self, p_id: u64)
	-> Result<Entry, Error>
	{
		let url = url!("info", "entry");
		let body = param_build!("api_key" => Some(&self.proxer.api_key), "id" => Some(p_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Entry> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert die unterschiedlichen Synonyme eines Animes/Mangas anhand seiner ID
	///
	/// # Arguments
	///
	/// * `id` - Die ID des gewünschten Animes/Mangas.
	pub fn get_name(&self, p_id: u64)
	-> Result<Vec<Name>, Error>
	{
		let url = url!("info", "names");
		let body = param_build!("api_key" => Some(&self.proxer.api_key), "id" => Some(p_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<Name>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert einen boolean Wert zurück,
	/// ob der per ID spezifizierte Anime/Manga eine ab-18 Prüfung erfordert.
	///
	/// # Arguments
	///
	/// * `id` - Die ID des gewünschten Animes/Mangas.
	pub fn get_gate(&self, p_id: u64)
	-> Result<bool, Error>
	{
		let url = url!("info", "gate");
		let body = param_build!("api_key" => Some(&self.proxer.api_key), "id" => Some(p_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<bool> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert die für einen Anime/Manga Eingetragenen Sprachen
	/// (in denen der Eintrag auf Proxer erhältlich ist).
	///
	/// # Arguments
	///
	/// * `id` - Die ID des gewünschten Animes/Mangas.
	pub fn get_language(&self, p_id: u64)
	-> Result<Vec<String>, Error>
	{
		let url = url!("info", "lang");
		let body = param_build!("api_key" => Some(&self.proxer.api_key), "id" => Some(p_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<String>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert die für einen Anime/Manga Eingetragenen Seasons.
	///
	/// # Arguments
	///
	/// * `id` - Die ID des gewünschten Animes/Mangas.
	pub fn get_season(&self, p_id: u64)
	-> Result<Vec<Season>, Error>
	{
		let url = url!("info", "season");
		let body = param_build!("api_key" => Some(&self.proxer.api_key), "id" => Some(p_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<Season>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert die für einen Anime/Manga Eingetragenen Übersetzergruppen.
	///
	/// # Arguments
	///
	/// * `id` - Die ID des gewünschten Animes/Mangas.
	pub fn get_groups(&self, p_id: u64)
	-> Result<Vec<Group>, Error>
	{
		let url = url!("info", "groups");
		let body = param_build!("api_key" => Some(&self.proxer.api_key), "id" => Some(p_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<Group>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert die für einen Anime/Manga Eingetragenen Publisher.
	///
	/// # Arguments
	///
	/// * `id` - Die ID des gewünschten Animes/Mangas.
	pub fn get_publisher(&self, p_id: u64)
	-> Result<Vec<Publisher>, Error>
	{
		let url = url!("info", "publisher");
		let body = param_build!("api_key" => Some(&self.proxer.api_key), "id" => Some(p_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<Publisher>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert eine Liste aller Episoden/Kapitel eines Entrys anhand dessen ID.
	///
	/// # Arguments
	///
	/// * `id` - Die ID des gewünschten Animes/Mangas.
	/// * `p` - (optional): Die zu ladende Seite der Liste, Start bei 0. Default 0.
	/// * `limit` - (optional): Die Nummer der zu ladenden Episoden/Kapitel pro Seite. Default 50.
	pub fn get_listinfo(&self, p_id: u64, p_page: Option<u64>, p_limit: Option<u64>)
	-> Result<ListInfo, Error>
	{
		let url = url!("info", "listinfo");
		let body = param_build!("api_key" => Some(&self.proxer.api_key),
			"id" => Some(p_id),
			"p" => p_page,
			"limit" => p_limit);
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<ListInfo> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert die für einen Anime/Manga abgegebenen Kommentare (mit mehr als 300 Zeichen).
	///
	/// # Arguments
	///
	/// * `id` - Die ID des gewünschten Animes/Mangas.
	/// * `p` - Die zu ladende Seite der Liste, Start bei 0. Default 0.
	/// * `limit` - Die Nummer der zu ladenden Kommentare pro Seite. Default 25.
	/// * `sort` - Ändert die Sortierung der Liste. Setze Wert "rating" um nach meisten Empfehlungen zu sortieren,
	/// ansonsten Sortierung nach Neueste zuerst.
	pub fn get_comments(&self, p_id: u64, p_page: Option<u64>, p_limit: Option<u64>, p_sort: Option<Sort>)
	-> Result<Vec<Group>, Error>
	{
		let url = url!("info", "comments");
		let body = param_build!("api_key" => Some(&self.proxer.api_key),
			"id" => Some(p_id),
			"p" => p_page,
			"limit" => p_limit,
			"sort" => p_sort);
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<Group>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert alle Verbindungen eines Entrys.
	///
	/// # Arguments
	///
	/// * `id` - Die ID des gewünschten Animes/Mangas.
	pub fn get_relations(&self, p_id: u64)
	-> Result<Vec<Relation>, Error>
	{
		let url = url!("info", "relations");
		let body = param_build!("api_key" => Some(&self.proxer.api_key), "id" => Some(p_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<Relation>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert alle Tags eines Entrys.
	///
	/// # Arguments
	///
	/// * `id` - Die ID des gewünschten Animes/Mangas.
	pub fn get_entrytag(&self, p_id: u64)
	-> Result<Vec<EntryTag>, Error>
	{
		let url = url!("info", "entrytag");
		let body = param_build!("api_key" => Some(&self.proxer.api_key), "id" => Some(p_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<EntryTag>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert alle Daten zu einer Sub/Scanlation Gruppe anhand ihrer ID.
	///
	/// # Arguments
	///
	/// * `id` - Die ID der gewünschten Gruppe.
	pub fn get_translatorgroup(&self, p_id: u64)
	-> Result<TranslatorGroup, Error>
	{
		let url = url!("info", "translatorgroup");
		let body = param_build!("api_key" => Some(&self.proxer.api_key), "id" => Some(p_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<TranslatorGroup> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert alle Daten zu einer Firma anhand ihrer ID.
	///
	/// # Arguments
	///
	/// * `id` - Die ID der gewünschten Gruppe.
	pub fn get_industry(&self, p_id: u64)
	-> Result<Industry, Error>
	{
		let url = url!("info", "industry");
		let body = param_build!("api_key" => Some(&self.proxer.api_key), "id" => Some(p_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Industry> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion setzt einen per ID spezifizierten Anime/Manga auf eine der Listen des Users, abhängig vom Paramter "type".
	///
	/// # Arguments
	///
	/// * `id` - Die ID des gewünschten Animes/Mangas.
	/// * `type` - Die Liste, zu der der Anime hinzugefügt werden soll.
	/// Erlaubt: "note" (Wird noch geschaut), "favor" (Favoriten), "finish" (Abgeschlossen)
	pub fn set_userinfo(&self, p_id: u64, p_type: WatchType)
	-> Result<(), Error>
	{
		let url = url!("info", "setuserinfo");
		let body = param_build!("api_key" => Some(&self.proxer.api_key), "id" => Some(p_id), "type" => Some(p_type));
		let response = self.proxer.connect(&url, &body)?;
		let data: EmptyResponse = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		Ok(())
	}
}
