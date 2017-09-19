use serde_json;

use ::error::*;
use ::Proxer;
use ::models::*;

/// Diese Funktion liefert die Liste aller Animes/Mangas, zu denen der User einen Eintrag im UCP hat.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct List
{
	/// Die ID des Entrys
	pub id: u64,
	/// Der Name des Entrys
	pub name: String,
	/// Anzahl der Folgen/Kapitel (müssen nicht hochgeladen sein!)
	pub count: u64,
	/// Das Medium des Entrys (animeseries,movie,ova,hentai,mangaseries,oneshot,doujin,hmanga)
	pub medium: Medium,
	/// Der Status des Entrys
	pub estate: String,
	/// Die ID des zugehörigen Kommentars
	pub cid: u64,
	/// Der Text des zugehörigen Kommentars
	pub comment: String,
	/// Der Status des zugehörigen Kommentars
	pub state: String,
	/// Die Episodenzahl des zugehörigen Kommentars (bis wohin der Entry gesehen/gelesen wurde)
	pub episode: u64,
	/// Die Daten des zugehörigen Kommentars
	pub data: String,
	/// Die Bewertung des Entrys durch den User (0 bis 10)
	pub rating: f32,
}

/// Diese Funktion liefert die Chronik des Users.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct History
{
	/// Die ID des Entrys
	pub eid: u64,
	/// Der Name des Entrys
	pub name: String,
	/// Die Sprache des Entrys
	pub language: String,
	/// Das Medium des Entrys (animeseries,movie,ova,hentai,mangaseries,oneshot,doujin,hmanga)
	pub medium: Medium,
	/// Die Kategorie des Entrys (anime,manga)
	pub kat: Kategorie,
	/// Die Episoden/Kapitelnummer
	pub episode: u64,
	/// Der Zeitpunkt des Aufrufs (Format: 'YYYY-MM-DD hh:mm:ss')
	pub timestamp: i64,
}

/// Diese Funktion liefert die Kommentarvotes des Users.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Vote
{
	/// Die ID des Kommentarvotes
	pub id: u64,
	/// Der Name des gevoteten Entrys
	pub name: String,
	/// Die User-ID des Erstellers des Kommentars
	pub uid: u64,
	/// Der Username des Erstellers des Kommentars
	pub username: String,
	/// Die ID des Kommentars
	pub kid: u64,
	/// Der Inhalt des Kommentars
	pub comment: String,
	/// Die Bewertung des Kommentars
	pub rating: String,
	/// Der Typ des Votes
	#[serde(rename="type")]
	pub vote_type: String,
}

/// Diese Funktion liefert eine Liste aller Lesezeichen des Users.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Reminder
{
	/// Die ID des Entrys
	pub eid: u64,
	/// Die Kategorie des Entrys (anime,manga)
	pub kat: Kategorie,
	/// Der Name des Entrys
	pub name: String,
	/// Die Folgen/Kapitelnummer des Lesezeichens
	pub episode: u64,
	/// Die Sprache des Lesezeichens
	pub language: String,
	/// Das Medium des Entrys (animeseries,movie,ova,hentai,mangaseries,oneshot,doujin,hmanga)
	pub medium: Medium,
	/// Die ID des Lesezeichens
	pub id: u64,
	/// Der Status des Entrys
	pub state: String,
}

/// Diese Klasse dient der Verwaltung sämtlicher Daten, die normalerweise über das UCP Abrufbar/Veränderbar sind.
/// Logischerweise erfordern alle diese Funktionen, dass der User eingeloggt ist.
#[derive(Debug)]
pub struct Ucp<'ucp>
{
	proxer: &'ucp Proxer,
}

impl<'ucp> Ucp<'ucp>
{
	#[doc(hidden)]
	pub fn new(p_proxer: &'ucp Proxer)
	-> Ucp<'ucp>
	{
		Ucp
		{
			proxer: p_proxer,
		}
	}

	/// Diese Funktion liefert die Liste aller Animes/Mangas, zu denen der User einen Eintrag im UCP hat.
	///
	/// # Arguments
	///
	/// * `p_kat` - Die Kategorie, die geladen werden soll. Mögliche Werte: anime, manga. Default: anime.
	/// * `p_page` - Dieser Parameter gibt an, welche Seite geladen werden soll. Default Wert 0. Start bei 0.
	/// * `p_limt` - Dieser Parameter gibt an, wie viele Einträge eine Seite enthalten soll. Default Wert 100.
	/// * `p_search` - Durch die Angabe dieses Parameters werden nur Entrys angezeigt,
	/// die den angegeben Wert als Substring ihres Namens haben. Dabei ist die Position im Namen egal.
	/// * `p_search_start` - Durch die Angabe dieses Parameters werden nur Entrys angezeigt,
	/// die den angegeben Wert als Substring zu Beginn ihres Namens haben.
	/// * `p_sort` - Dieser Parameter gibt an, wie die Liste sortiert werden soll,
	/// erlaubte Eingaben (Fehlerhafte Eingaben werden auf den Default-Wert gezwungen)
	pub fn get_list(&self,
		p_kat: Option<Kategorie>,
		p_page: Option<u64>,
		p_limit: Option<u64>,
		p_search: Option<String>,
		p_search_start: Option<String>,
		p_sort: Option<Sort>)
	-> Result<Vec<List>>
	{
		let url = url!("ucp", "list");
		let body = param_build!("kat" => p_kat,
			"p" => p_page,
			"limit" => p_limit,
			"search" => p_search,
			"search_start" => p_search_start,
			"sort" => p_sort);
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<List>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}
	/// Diese Funktion liefert die Summe der Episoden/Kapitel, die der User bisher gesehen hat.
	///
	/// # Arguments
	///
	/// * `p_kat` - Die Kategorie, die geladen werden soll. Mögliche Werte: anime, manga. Default: anime.
	pub fn get_listsum(&self, p_kat: Option<Kategorie>)
	-> Result<String>
	{
		let url = url!("ucp", "listsum");
		let body = param_build!("kat" => p_kat);
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<String> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert die Top-Ten des Users. (Anime und Manga)
	pub fn get_topten(&self)
	-> Result<String>
	{
		let url = url!("ucp", "topten");
		let body = String::new();
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<String> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert die Chronik des Users.
	///
	/// # Arguments
	///
	/// * `p_limit` - Dieser Parameter gibt an, wie viele Einträge eine Seite der Chronik haben soll. Default Wert 50.
	/// * `p_page` - Dieser Parameter gibt an, welche Seite der Chronik geladen werden soll. Start bei 0, Default Wert 0.
	pub fn get_history(&self, p_limit: Option<u64>, p_page: Option<u64>)
	-> Result<Vec<History>>
	{
		let url = url!("ucp", "history");
		let body = param_build!("limit" => p_limit, "p" => p_page);
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<History>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert die Kommentarvotes des Users.
	pub fn get_votes(&self)
	-> Result<Vec<Vote>>
	{
		let url = url!("ucp", "votes");
		let body = String::new();
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<Vote>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert eine Liste aller Lesezeichen des Users.
	///
	/// # Arguments
	///
	/// * `p_kat` - Dieser Parameter gibt an, welche Kategorie geladen werden soll.
	/// Wenn weggelassen werden beide Kategorien geladen. Erlaubte Werte: "anime","manga"
	/// * `p_page` - Die zu ladende Seite, Start bei 0. Default 0
	/// * `p_limit` - Die zu ladenden Einträge pro Seite. Default 100
	pub fn get_reminder(&self, p_kat: Option<Kategorie>, p_page: Option<u64>, p_limit: Option<u64>)
	-> Result<Vec<Reminder>>
	{
		let url = url!("ucp", "reminder");
		let body = param_build!("kat" => p_kat, "p" => p_page, "limit" => p_limit);
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<Reminder>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion löscht ein Lesezeichen.
	///
	/// # Arguments
	///
	/// * `p_id` - Die ID des zu löschenden Lesezeichens (erhältlich über die "Reminder" Funktion)
	pub fn delete_reminder(&self, p_id: u64)
	-> Result<()>
	{
		let url = url!("ucp", "deletereminder");
		let body = param_build!("id" => Some(p_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: EmptyResponse = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		Ok(())
	}

	/// Diese Funktion löscht einen Eintrag der Top-ten.
	///
	/// # Arguments
	///
	/// * `p_id` - Die ID des zu löschenden Eintrags (erhältlich über die "Favorite" Funktion)
	pub fn delte_favorite(&self, p_id: u64)
	-> Result<()>
	{
		let url = url!("ucp", "deletefavorite");
		let body = param_build!("id" => Some(p_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: EmptyResponse = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		Ok(())
	}

	/// Diese Funktion löscht einen Kommentarvote.
	///
	/// # Arguments
	///
	/// * `p_id` - Die ID des zu löschenden Eintrags (erhältlich über die "Vote" Funktion)
	pub fn delte_vote(&self, p_id: u64)
	-> Result<()>
	{
		let url = url!("ucp", "deletevote");
		let body = param_build!("id" => Some(p_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: EmptyResponse = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		Ok(())
	}

	/// Diese Funktion setzt die Zahl der bereits gesehenen/gelesenen Folgen/Kapitel
	/// für einen Anime/Manga aus der Liste des Users.
	/// Wird dieser Wert auf oder über die Zahl der vorhandenen Folgen/Kapitel gesetzt,
	/// so wird zudem der Status des Animes/Mangas auf "Abgeschlossen" gesetzt.
	///
	/// # Arguments
	///
	/// * `p_id` - Die ID des zu bearbeitenden Eintrags (erhältlich über die "List" Funktion)
	/// * `p_value` - Der zu setzende Wert
	pub fn set_commentstate(&self, p_id: u64, p_value: u64)
	-> Result<()>
	{
		let url = url!("ucp", "setcommentstate");
		let body = param_build!("id" => Some(p_id), "value" => Some(p_value));
		let response = self.proxer.connect(&url, &body)?;
		let data: EmptyResponse = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		Ok(())
	}

	/// Diese Funktion setzt eine Episode auf die Watch/Readlist eines eingeloggten Users.
	///
	/// # Arguments
	///
	/// * `p_id` - Die id des Entrys
	/// * `p_episode` - Die Episodennummer, auf die das Lesezeichen gesetzt werden soll.
	/// * `p_language` - Die zu ladende Sprache. (Für Animes: gersub,gerdub,engsub,engdub; Für Mangas: de,en)
	/// * `p_kat` - Die Kategorie des Entrys (manga oder anime)
	pub fn set_reminder(&self, p_id: u64, p_value: u64)
	-> Result<()>
	{
		let url = url!("ucp", "setreminder");
		let body = param_build!("id" => Some(p_id), "value" => Some(p_value));
		let response = self.proxer.connect(&url, &body)?;
		let data: EmptyResponse = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		Ok(())
	}
}
