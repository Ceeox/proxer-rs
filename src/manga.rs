use serde_json;

use ::error::Error;
use ::Proxer;
use ::models::*;

/// Diese Funktion liefert ein Kapitel eines festgelegten Mangas.
/// Um Mangapunkte für das Lesen zu erhalten muss ein User angemeldet sein.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chapter
{
	/// Die ID des Kapitels
	pub cid:					u64,
	/// Die ID des Entrys
	pub eid:					u64,
	/// Der Titel des Kapitels
	pub title:					String,
	/// Die ID des Uploaders
	pub uploader:				u64,
	/// Der Name des Uploaders
	pub username:				String,
	/// Der Hochladezeitpunkt (Unix-Timestamp als Integer)
	pub timestamp:				i64,
	/// Die ID der zugewiesenen Scangruppe, null wenn nicht vorhanden.
	pub tid:					u64,
	/// Der Name der zugewiesenen Scangruppe, null wenn nicht vorhanden.
	pub tname:					String,
	/// Notwendig zur Anzeige der Seiten, siehe "pages"
	pub server:					u32,
	/// Ein Array aus Arrays, die die Seiten des Kapitels in geordneter Reihenfolge enthalten.
	/// Jedes Unter-Array enthält dabei folgende Daten:
	/// 0 => Name der Seite (Dateiname),
	/// 1 => Höhe der Seite,
	/// 2 => Breite der Seite.
	/// Um eine einzelne Seite anzuzeigen, ist folgender Link nötig: //manga<server>.proxer.me/f/<eid>/<cid>/<page[0]>
	/// (Unter der Annahme, dass page = pages[i])
	pub pages:					Vec<Vec<(i32, i32, i32)>>,
}

impl Chapter
{
	pub fn get_chapter_link(&self)
	-> String
	{
		// format!("https://manga{}.proxer.me/f/{}/{}/{}", self.server, self.eid, self.cid, self.)
		format!("TODO")
	}
}

/// Diese Klasse dient dazu, Kapitel für Mangas zu erhalten,
/// und andere rein Manga-bezogene Aktionen durchzuführen.
/// Der Zugriff auf diese Klasse ist stark beschränkt.
#[derive(Debug)]
pub struct Manga<'manga>
{
	proxer:			&'manga Proxer,
}

impl<'manga> Manga<'manga>
{
	#[doc(hidden)]
	pub fn new(p_proxer: &'manga Proxer)
	-> Manga<'manga>
	{
		Manga
		{
			proxer:		p_proxer,
		}
	}

	/// Diese Funktion liefert ein Kapitel eines festgelegten Mangas.
	/// Um Mangapunkte für das Lesen zu erhalten muss ein User angemeldet sein.
	///
	///	# Arguments
	///
	/// * `id` - Die id des Entrys
	/// * `episode` - Die Episodennummer des zu ladenden Kapitels
	/// * `language` - Die zu ladende Sprache (de,en)
	pub fn get_chapter(&self, p_id: u64, p_episode: u64, p_language: &str)
	-> Result<Vec<Chapter>, Error>
	{
		let url = url!("manga", "chapter");
		let body = param_build!("api_key" => Some(&self.proxer.api_key),
			"id" => Some(p_id),
			"episode" => Some(p_episode),
			"language" => Some(p_language));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<Chapter>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}
}
