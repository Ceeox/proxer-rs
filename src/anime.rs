use serde_json;

use ::error::*;
use ::Proxer;
use ::models::*;

/// Diese Funktion liefert die Streams einer Folge eines festgelegten Animes, ohne den Proxerstream!.
/// Um Animepunkte für das Schauen zu erhalten muss ein User angemeldet sein.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Stream
{
    /// Die ID des Streams
    pub id: u64,
    /// Auf welchem Hoster der Stream liegt
    #[serde(rename="type")]
    pub host_type: String,
    /// Der Name des Hosters (Klartext)
    pub name: String,
    /// Das Anzeigebild des Hosters
    pub img: String,
    /// Die ID des Uploaders
    pub uploader: u64,
    /// Der Name des Uploaders
    pub username: String,
    /// Der Verlinkzeitpunkt (Unix-Timestamp als Integer)
    pub timestamp: i64,
    /// Die ID der zugewiesenen Subgruppe, null wenn nicht vorhanden.
    pub tid: u64,
    /// Der Name der zugewiesenen Subgruppe, null wenn nicht vorhanden.
    pub tname: String,
    /// Der Typ des Hosters (iframe,embed,js,code,link)
    pub htype: String,
}

/// Diese Funktion liefert die Streams einer Folge eines festgelegten Animes, inklusive des Proxerstreams!.
/// Um Animepunkte für das Schauen zu erhalten muss ein User angemeldet sein.
/// Die Schnittstelle liefert auch alle weiteren Streams,
/// eine zusätzliche Abfrage der "Get Streams" Schnittstelle ist also nicht nötig.
/// Hinweis: Diese Schnittstelle ist momentan gesperrt!
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ProxerStream
{
    /// Die ID des Streams
    pub id: u64,
    /// Auf welchem Hoster der Stream liegt
    #[serde(rename="type")]
    pub host_type: String,
    /// Der Name des Hosters (Klartext)
    pub name: String,
    /// Das Anzeigebild des Hosters
    pub img: String,
    /// Die ID des Uploaders
    pub uploader: u64,
    /// Der Name des Uploaders
    pub username: String,
    /// Der Verlinkzeitpunkt (Unix-Timestamp als Integer)
    pub timestamp: i64,
    /// Die ID der zugewiesenen Subgruppe, null wenn nicht vorhanden.
    pub tid: String,
    /// Der Name der zugewiesenen Subgruppe, null wenn nicht vorhanden.
    pub tname: String,
    /// Der Typ des Hosters (iframe,embed,js,code,link)
    pub htype: String,
}

/// Diese Klasse dient dazu, Streams für Animes zu erhalten, und andere rein Anime-bezogene Aktionen durchzuführen.
/// Der Zugriff auf diese Klasse ist stark beschränkt.
#[derive(Debug)]
pub struct Anime<'a>
{
    proxer: &'a Proxer,
}

impl<'a> Anime<'a>
{
    #[doc(hidden)]
    pub fn new(proxer: &'a Proxer)
    -> Anime<'a>
    {
        Anime
        {
            proxer: proxer,
        }
    }

    /// Diese Funktion liefert die Streams einer Folge eines festgelegten Animes, ohne den Proxerstream!.
    /// Um Animepunkte für das Schauen zu erhalten muss ein User angemeldet sein.
    ///
    /// #Arguments
    ///
    /// * `id` - Die id des Entrys
    /// * `episode` - Die Episodennummer der zu ladenden Folge
    /// * `language` - Die zu ladende Sprache (gersub,gerdub,engsub,engdub)
    pub fn get_streams(&self, id: u64, episode: u64, language: Language)
    -> Result<Vec<Stream>>
    {
        let url = url!("anime", "streams");
        let body = param_build!("id" => Some(id),
            "episode" => Some(episode),
            "language" => Some(language));
        let response = self.proxer.connect(&url, &body)?;
        let data: Response<Vec<Stream>> = serde_json::from_reader(response)?;
        check_error!(data.error, data.code.unwrap_or_default(), data.message);
        check_data!(data.data)
    }

    /// Diese Funktion liefert die Streams einer Folge eines festgelegten Animes, inklusive des Proxerstreams!.
    /// Um Animepunkte für das Schauen zu erhalten muss ein User angemeldet sein.
    /// Die Schnittstelle liefert auch alle weiteren Streams,
    /// eine zusätzliche Abfrage der "Get Streams" Schnittstelle ist also nicht nötig.
    /// Hinweis: Diese Schnittstelle ist momentan gesperrt!
    ///
    /// #Arguments
    ///
    /// * `id` - Die id des Entrys
    /// * `episode` - Die Episodennummer der zu ladenden Folge
    /// * `language` - Die zu ladende Sprache (gersub,gerdub,engsub,engdub)
    pub fn get_proxerstreams(&self, id: u64, episode: u64, language: Language)
    -> Result<Vec<ProxerStream>>
    {
        let url = url!("anime", "proxerstreams");
        let body = param_build!("id" => Some(id),
            "episode" => Some(episode),
            "language" => Some(language));
        let response = self.proxer.connect(&url, &body)?;
        let data: Response<Vec<ProxerStream>> = serde_json::from_reader(response)?;
        check_error!(data.error, data.code.unwrap_or_default(), data.message);
        check_data!(data.data)
    }

    /// Diese Funktion dient dazu, den Link zu einem einzelnen Stream zu erhalten.
    /// Dazu muss die ID des gewählten Streams angegeben werden.
    /// Dieser kann erhalten werden durch die Get Streams oder Get Proxerstreams Funktionen.
    /// Diese Funktion erhöht euch den Viewcount des Streams.
    ///
    /// #Arguments
    ///
    /// * `id` - Die id des Entrys
    pub fn get_link(&self, id: u64)
    -> Result<String>
    {
        let url = url!("anime", "link");
        let body = param_build!("id" => Some(id));
        let response = self.proxer.connect(&url, &body)?;
        let data: Response<String> = serde_json::from_reader(response)?;
        check_error!(data.error, data.code.unwrap_or_default(), data.message);
        check_data!(data.data)
    }
}
