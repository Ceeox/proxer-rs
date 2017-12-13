use serde_json;

use ::error::*;
use ::Proxer;
use ::models::*;

/// Mit dieser Schnittstelle kann ein User mithilfe eines Passwortes und eines Usernamen eingeloggt werden
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Login
{
    /// Die ID des eingeloggten Users.
    pub uid: u64,
    /// Der Avatar des eingeloggten Users.
    pub avatar: String,
    /// Ein Login-Token. Möchte man das gewöhnliche Cookie-basierte Login System nicht verwenden, so kann man stattdessen bei jeder Anfrage die einen Login erfordert dieses Token senden.
    pub token: String,
}

/// Mit dieser Schnittstelle kann ein User ausgeloggt werden
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Logout
{
    pub error: u8,
    pub message: String,
}

/// Mit dieser Schnittstelle können die öffentlichen Daten jedes Users per ID oder Username abgefragt werden.
/// Sind ID und Username gegeben, so wird ausschließlich die ID verwendet.
/// Ist weder ID noch Username gegeben, so werden die Daten des eingeloggten Users abgerufen.
/// Ist auch dies nicht gegeben, so wird eine Fehlermeldung ausgegeben.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct UserInfo
{
    ///  Die ID des abgefragten Users
    pub uid: u64,
    /// Der Username des abgefragten Users
    pub username: String,
    /// Der Avatar des abgefragten Users
    pub avatar: String,
    /// Der momentane Status des abgefragten Users
    pub status: String,
    /// Der Zeitpunkt der letzten Änderung des Status des abgefragten Users (Als Unix-Timestamp in Sekunden, false wenn nicht gesetzt)
    pub status_time: i64,
    /// Die Uploadpunkte des abgefragten Users
    pub points_uploads: u64,
    /// Die Animepunkte des abgefragten Users
    pub points_anime: u64,
    /// Die Mangapunkte des abgefragten Users
    pub points_manga: u64,
    /// Die Infopunkte des abgefragten Users
    pub points_info: u64,
    /// Die Forums-Punkte des abgefragten Users
    pub points_forum: u64,
    /// Die sonstigen Punkte des abgefragten Users
    pub points_misc: u64,
}

/// Mit dieser Schnittstelle können die Topten jedes Users per ID oder Username abgefragt werden.
/// Sind ID und Username gegeben, so wird ausschließlich die ID verwendet.
/// Ist weder ID noch Username gegeben, so wird eine Fehlermeldung ausgegeben.
/// Diese Schnittstelle respektiert die Sichtbarkeitseinstellungen der User.
/// Hat ein User etwa die Sichtbarkeit nur "Für Freunde",
/// so wird geprüft ob der momentan eingeloggte Nutzer mit dem User Befreundet ist.
/// Es ist daher ratsam, vor der Verwendung dieser Schnittstelle einen User einzuloggen (oder ein token zu verwenden).
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct TopTen
{
    ///  Die ID des Entrys
    pub eid: u64,
    ///  Der Name des Entrys
    pub name: String,
    /// Die Kategorie des Entrys (anime oder manga)
    pub kat: Kategorie,
    /// Das Medium des Entrys (animeseries,movie,ova,hentai,mangaseries,oneshot,doujin,hmanga)
    pub medium: Medium,
}

/// Mit dieser Schnittstelle können die Entrylisten jedes Users per ID oder Username abgefragt werden.
/// Sind ID und Username gegeben, so wird ausschließlich die ID verwendet.
/// Ist weder ID noch Username gegeben, so wird eine Fehlermeldung ausgegeben.
/// Diese Schnittstelle respektiert die Sichtbarkeitseinstellungen der User.
/// Hat ein User etwa die Sichtbarkeit nur "Für Freunde",
/// so wird geprüft ob der momentan eingeloggte Nutzer mit dem User Befreundet ist.
/// Es ist daher ratsam, vor der Verwendung dieser Schnittstelle einen User einzuloggen (oder ein token zu verwenden).
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
    pub medium: String,
    /// Der Status des Entrys
    pub estate: u64,
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
    pub rating: i8,
    /// Der Änderungszeitpunkt des Kommentars (Unix-Timestamp als Integer)
    pub timestamp: i64,
}

/// Mit dieser Schnittstelle können die letzten Kommentare jedes Users per ID oder Username abgefragt werden.
/// Sind ID und Username gegeben, so wird ausschließlich die ID verwendet.
/// Ist weder ID noch Username gegeben, so wird eine Fehlermeldung ausgegeben.
/// Diese Schnittstelle respektiert die Sichtbarkeitseinstellungen der User.
/// Hat ein User etwa die Sichtbarkeit nur "Für Freunde",
/// so wird geprüft ob der momentan eingeloggte Nutzer mit dem User Befreundet ist.
/// Es ist daher ratsam, vor der Verwendung dieser Schnittstelle einen User einzuloggen (oder ein token zu verwenden).
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct LatestComment
{
    /// Die ID des Kommentars
    pub id: u64,
    /// Die ID des Entrys
    pub tid: u64,
    /// Der beim Kommentar angegebene Status (0 = geschaut, 1 = am schauen, 2 = wird geschaut, 3 = abgebrochen)
    pub state: i8,
    /// Zusätzliche informationen, die als json-String abgespeichert werden. Es kann sich hierbei beispielsweise um die "Genre"-Bewertung handeln.
    pub data: String,
    /// Der Kommentar-Text
    pub comment: String,
    /// Die Bewertung des Entrys
    pub rating: i8,
    /// Die Episode, bis zu der der Kommentarschreiber geschaut hat
    pub episode: u64,
    /// Wie viele Personen den Kommentar hilfreich finden.
    pub positive: u64,
    /// Der Zeitpunkt der Erstellung des Kommentars (Unix-Timestamp in Sekunden)
    pub timestamp: i64,
    /// Der Username des Erstellers des Kommentars
    pub username: String,
    /// Die User-ID des Erstellers des Kommentars
    pub uid:  u64,
    /// Das Profilbild des Erstellers des Kommentars
    pub avatar: String,
}

/// Diese Klasse beinhaltet die grundlegenden Schnittstellen zur Verwaltung von Userdaten,
/// vornehmlich Login und Logout, aber auch die Möglichkeit,
/// einen neuen User zu registrieren sowie die öffentlichen Daten eines jeden Users per ID oder Username abzufragen.
#[derive(Debug)]
pub struct User<'user>
{
    /// Die ID des eingeloggten Users.
    pub uid: u64,
    /// Der Avatar des eingeloggten Users.
    pub avatar: String,
    /// Ein Login-Token.
    /// Möchte man das gewöhnliche Cookie-basierte Login System nicht verwenden,
    /// so kann man stattdessen bei jeder Anfrage die einen Login erfordert dieses Token senden.
    pub token: String,

    proxer: &'user Proxer,
}

impl<'user> User<'user>
{
    #[doc(hidden)]
    pub fn new(p_proxer: &'user Proxer, p_username: &str, p_password: &str)
    -> Result<User<'user>>
    {
        let user: Login = User::login(&p_proxer, p_username, p_password)?;
        Ok(User
        {
            uid: user.uid,
            avatar: user.avatar,
            token: user.token,
            proxer: p_proxer,
        })
    }

    /// Mit dieser Schnittstelle kann ein User mithilfe eines Passwortes und eines Usernamen eingeloggt werden
    ///
    /// # Arguments
    ///
    /// * `p_username` - Der Benutzername des zu einloggenden Benutzers.
    /// * `p_password` - Das Passwort des zu einloggenden Benutzers
    /// * `p_api_key` - API_KEY.
    pub fn login(p_proxer: &Proxer, p_username: &str, p_password: &str)
    -> Result<Login>
    {
        let url = url!("user", "login");
        let body = param_build!("username" => Some(p_username),
            "password" => Some(p_password));
        let response = p_proxer.connect(&url, &body)?;
        let data: Response<Login> = serde_json::from_reader(response)?;
        check_data!(data.data)
    }

    /// Mit dieser Schnittstelle kann ein User ausgeloggt werden
    pub fn logout(self)
    -> Result<()>
    {
        let url = url!("user", "logout");
        let response = self.proxer.connect(&url, "")?;
        let data: Logout = serde_json::from_reader(response)?;
        check_error!(data.error, 0, data.message);
        Ok(())
    }

    /// Mit dieser Schnittstelle können die öffentlichen Daten jedes Users per ID oder Username abgefragt werden.
    /// Sind ID und Username gegeben, so wird ausschließlich die ID verwendet.
    /// Ist weder ID noch Username gegeben, so werden die Daten des eingeloggten Users abgerufen.
    /// Ist auch dies nicht gegeben, so wird eine Fehlermeldung ausgegeben.
    ///
    /// # Arguments
    ///
    /// * `&self` - User-ID, deren Daten abgefragt werden sollen
    pub fn get_userinfo(&self)
    -> Result<UserInfo>
    {
        let url = url!("user", "userinfo");
        let body = param_build!("uid" => Some(self.uid));
        let response = self.proxer.connect(&url, &body)?;
        let data: Response<UserInfo> = serde_json::from_reader(response)?;
        check_error!(data.error, data.code.unwrap_or_default(), data.message);
        check_data!(data.data)
    }

    /// Mit dieser Schnittstelle können die Topten jedes Users per ID oder Username abgefragt werden.
    /// Sind ID und Username gegeben, so wird ausschließlich die ID verwendet.
    /// Ist weder ID noch Username gegeben, so wird eine Fehlermeldung ausgegeben.
    /// Diese Schnittstelle respektiert die Sichtbarkeitseinstellungen der User.
    /// Hat ein User etwa die Sichtbarkeit nur "Für Freunde",
    /// so wird geprüft ob der momentan eingeloggte Nutzer mit dem User Befreundet ist.
    /// Es ist daher ratsam, vor der Verwendung dieser Schnittstelle einen User einzuloggen (oder ein token zu verwenden).
    ///
    /// # Arguments
    ///
    /// * `p_uid` - User-ID, deren Daten abgefragt werden sollen
    /// * `p_kat` - Die Kategorie, die geladen werden soll. Mögliche Werte: anime, manga. Default: anime.
    pub fn get_topten(&self, p_uid: u64, p_kat: Option<Kategorie>)
    -> Result<Vec<TopTen>>
    {
        let url = url!("user", "topten");
        let body = param_build!("uid" => Some(p_uid), "kat" => p_kat);
        let response = self.proxer.connect(&url, &body)?;
        let data: Response<Vec<TopTen>> = serde_json::from_reader(response)?;
        check_error!(data.error, data.code.unwrap_or_default(), data.message);
        check_data!(data.data)
    }

    /// Mit dieser Schnittstelle können die Entrylisten jedes Users per ID oder Username abgefragt werden.
    /// Sind ID und Username gegeben, so wird ausschließlich die ID verwendet.
    /// Ist weder ID noch Username gegeben, so wird eine Fehlermeldung ausgegeben.
    /// Diese Schnittstelle respektiert die Sichtbarkeitseinstellungen der User.
    /// Hat ein User etwa die Sichtbarkeit nur "Für Freunde",
    /// so wird geprüft ob der momentan eingeloggte Nutzer mit dem User Befreundet ist.
    /// Es ist daher ratsam, vor der Verwendung dieser Schnittstelle einen User einzuloggen (oder ein token zu verwenden).
    ///
    /// # Arguments
    ///
    /// * `p_uid` - User-ID, deren Daten abgefragt werden sollen
    /// * `p_kat` - Die Kategorie, die geladen werden soll. Mögliche Werte: anime, manga. Default: anime.
    /// * `p_page` - Dieser Parameter gibt an, welche Seite geladen werden soll. Default Wert 0. Start bei 0.
    /// * `p_limt` - Dieser Parameter gibt an, wie viele Einträge eine Seite enthalten soll. Default Wert 100.
    /// * `p_search` - Durch die Angabe dieses Parameters werden nur Entrys angezeigt, die den angegeben Wert als Substring ihres Namens haben. Dabei ist die Position im Namen egal.
    /// * `p_search_start` - Durch die Angabe dieses Parameters werden nur Entrys angezeigt, die den angegeben Wert als Substring zu Beginn ihres Namens haben.
    /// * `p_sort` - Dieser Parameter gibt an, wie die Liste sortiert werden soll, erlaubte Eingaben (Fehlerhafte Eingaben werden auf den Default-Wert gezwungen)
    pub fn get_list(&self,
        p_uid:	u64,
        p_kat: Option<Kategorie>,
        p_page: Option<u64>,
        p_limit: Option<u64>,
        p_search: Option<String>,
        p_search_start: Option<String>,
        p_sort: Option<Sort>)
    -> Result<Vec<List>>
    {
        let url = url!("user", "list");
        let body = param_build!("uid" => Some(p_uid),
            "kat" => p_kat,
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

    /// Mit dieser Schnittstelle können die letzten Kommentare jedes Users per ID oder Username abgefragt werden.
    /// Sind ID und Username gegeben, so wird ausschließlich die ID verwendet.
    /// Ist weder ID noch Username gegeben, so wird eine Fehlermeldung ausgegeben.
    /// Diese Schnittstelle respektiert die Sichtbarkeitseinstellungen der User.
    /// Hat ein User etwa die Sichtbarkeit nur "Für Freunde",
    /// so wird geprüft ob der momentan eingeloggte Nutzer mit dem User Befreundet ist.
    /// Es ist daher ratsam, vor der Verwendung dieser Schnittstelle einen User einzuloggen (oder ein token zu verwenden).
    ///
    /// * `&self` - User-ID, deren Daten abgefragt werden solle.
    /// * `p_kat` - Die Kategorie, die geladen werden soll. Mögliche Werte: anime, manga. Default: anime.
    /// * `p_page` - Dieser Parameter gibt an, welche Seite geladen werden soll. Default Wert 0. Start bei 0.
    /// * `p_limit` - Dieser Parameter gibt an, wie viele Einträge eine Seite enthalten soll. Default Wert 25.
    /// * `p_length` -  Dieser Parameter gibt die minimale Anzahl an Zeichen an, ab der ein Kommentar angezeigt werden soll. Default 300.
    pub fn get_latestcomments(&self,
        p_kat: Option<Kategorie>,
        p_page: Option<u64>,
        p_limit: Option<u64>,
        p_length: Option<u64>)
    -> Result<Vec<LatestComment>>
    {
        let url = url!("user", "comments");
        let body = param_build!("uid" => Some(self.uid),
            "kat" => p_kat,
            "p" => p_page,
            "limit" => p_limit,
            "length" => p_length);
        let response = self.proxer.connect(&url, &body)?;
        let data: Response<Vec<LatestComment>> = serde_json::from_reader(response)?;
        check_error!(data.error, data.code.unwrap_or_default(), data.message);
        check_data!(data.data)
    }
}
