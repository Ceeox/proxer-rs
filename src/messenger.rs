use serde_json;

use ::error::*;
use ::Proxer;
use ::models::*;

/// Liefert Messengerkonstanten.
/// Bitte höchstens beim erstmaligen Start einer Anwendung durchführen.
/// Diese Werte werden sich höchstens alle paar Monate mal ändern.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Constants
{
	/// Maximalanzahl an Zeichen pro Nachricht.
	#[serde(rename="textCount")]
	pub text_count: u64,
	/// Anzahl der Elemente, die "Get Conferences" maximal pro Aufruf/pro Seite liefert.
	#[serde(rename="conferenceLimit")]
	pub conference_limit: u64,
	/// Anzahl der Elemente, die "Get Messages" maximal pro Aufruf/pro Seite liefert.
	#[serde(rename="messagesLimit")]
	pub messages_limit: u64,
	/// Maximalanzahl an Benutzern pro Gruppenkonferenz.
	#[serde(rename="userLimit")]
	pub user_limit: u64,
	/// Maximalanzahl an Zeichen für Konferenzthema.
	#[serde(rename="tropicCount")]
	pub topic_count: u64,
}

/// Liefert eine Liste der aktuellen Konferenzen.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Conference
{
	/// Die Konferenz-ID
	pub id: u64,
	/// Der Titel einer Konferenz.
	/// Dies ist entweder der Benutzername des Gesprächspartners, oder das Thema einer Gruppenkonferenz.
	pub topic: String,
	/// Der Titel, der vom Benutzer festgelegt wurde.
	pub topic_custom: String,
	/// Anzahl der Konferenzteilnehmer.
	pub count: u64,
	/// true oder false. true Falls es sich um eine Gruppenkonferenz handelt. Ansonsten false.
	pub group: bool,
	/// Zeitstempel der letzten Nachricht.
	pub timestamp_end: String,
	/// true oder false: true falls Benutzer letzte Nachricht dieser Konferenz gelesen hat.
	pub read: bool,
	/// Anzahl der ungelesenen Nachrichten.
	pub read_count: u64,
	/// Die Message-ID der letzten gelesenen Nachricht.
	pub read_mid: u64,
	/// Bild der Konferenz. Format: "Bild-Typ:Bild-ID".
	/// Typ ist meistens "avatar" und ID beispielsweise "62_yF5zd7.jpg".
	/// Avatare haben den folgenden Link: http://cdn.proxer.me/avatar/tn/[Bild-ID]
	pub image: String,
}

/// Informationen zu einer bestimmten Konferenz.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ConferenceInfo
{
	/// Allgemeine Infos zur Konferenz. Objekt enthält folgende Spalten:
	pub conference: ConferenceInfoDataConference,
	/// Informationen zu Konferenzteilnehmern. Array enthält folgende Spalten:
	pub users: Vec<ConferenceInfoDataUsers>,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ConferenceInfoDataConference
{
	/// Thema der Konferenz.
	pub topic: String,
	/// Anzahl der Konferenzteilnehmer.
	pub count: u64,
	/// Zeitstempel der letzten Konferenz-Nachricht.
	pub timestamp_end: i64,
	/// Die Benutzer-ID des Konferenzleiters.
	pub leader: u64,
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct ConferenceInfoDataUsers
{
	/// User-ID
	pub uid: u64,
	/// Bild-ID des Avatars.
	/// Falls kein Avatar gesetzt ist, ist dieses Feld leer. Beispiel für Bild-ID: "62_yF5zd7.jpg"
	pub avatar: String,
	/// Benutzername.
	pub username: String,
	/// Statusmitteilung des Benutzers.
	pub status: String,
}

/// Informationen zu einem bestimmten Benutzer.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct UserInfo
{
	/// Bild-ID des Avatars. Falls kein Avatar gesetzt ist, ist dieses Feld leer.
	/// Beispiel für Bild-ID: "62_yF5zd7.jpg".
	/// Avatare haben den folgenden Link: http://cdn.proxer.me/avatar/tn/[Bild-ID]
	pub avatar: String,
	/// Benutzername des Benutzers
	pub username: String,
	/// Status des Benutzers
	pub status: String,
}

/// Gibt die letzten Nachrichten einer Konferenz/eines Benutzers zurück.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Messages
{
	/// conference_id=0 und message_id=0:
	/// 	Gibt die letzten Nachrichten des Benutzers zurück.
	/// 	Die letzte Nachricht dieser Ausgabe kann verwendet werden.
	/// 	Empfohlen für den ersten Programmstart um die ID der letzten Nachricht zu erhalten.
	/// conference_id=0 und message_id!=0:
	/// 	Gibt die letzten Nachrichten ab einer bestimmten Nachrichten-ID zurück.
	/// 	Dieser Fall kann für die Abfrage der Benachrichtigungen verwendet werden.
	/// conference_id!=0 und message_id=0:
	/// 	Gibt die letzten Nachrichten einer bestimmten Konferenz zurück.
	/// 	Kann für den Fall verwendet werden, wenn ein Benutzer zum ersten Mal eine Konferenz öffnet.
	/// 	Falls read=true: Konferenz wird als gelesen markiert
	/// 	(read=true, read_count=0, read_mid=letzte Nachrichten-ID).
	/// conference_id!=0 und message_id!=0:
	/// 	Gibt die letzten Nachrichten einer bestimmten Konferenz bis zu einer bestimmten
	///		Nachricht zurück.
	/// 	Kann verwendet werden, falls ein Benutzer beispielsweise weitere Nachrichten einer
	///		Konferenz anzeigen möchte.
	/// 	Falls read=true: Konferenz wird als gelesen markiert
	///		(read=true, read_count=0, read_mid=letzte Nachrichten-ID).
	/// Das Array-Element enthalten die folgenden Spalten:
	/// Die Nachrichten-ID.
	pub message_id: u64,
	/// Die Konferenz-ID.
	pub conference_id: u64,
	/// Die User-ID des Verfassers.
	pub user_id: u64,
	/// Der Username des Verfassers.
	pub username: String,
	/// Die Nachricht.
	pub message: String,
	/// Falls es sich um ein Befehl handelt, ist hier der Befehl angegeben.
	pub action: String,
	/// Zeitstempel des Absendezeitpunktes.
	pub timestamp: i64,
	/// Das Gerät/die Anwendung aus der eine Nachricht gesendet wurde.
	pub device: String,
}

/// Der Proxer Messenger ist ein Nachrichtensystem, welches es ermöglichen soll,
/// über verschiedene Plattformen hinweg, mit anderen Proxer-Nutzern Nachrichten auszutauschen.
/// Die aktuelle Schnittstelle basiert auf polling. Es ist eine zusätzliche Schnittstelle,
/// die auf push-Nachrichten und Sockets basiert, geplant.
#[derive(Debug)]
pub struct Messenger<'messenger>
{
	proxer: &'messenger Proxer,
}

impl<'messenger> Messenger<'messenger>
{
	#[doc(hidden)]
	pub fn new(p_proxer: &'messenger Proxer)
	-> Messenger<'messenger>
	{
		Messenger
		{
			proxer: p_proxer,
		}
	}

	/// Liefert Messengerkonstanten. Bitte höchstens beim erstmaligen Start einer Anwendung durchführen.
	/// Diese Werte werden sich höchstens alle paar Monate mal ändern.
	pub fn get_constants(&self)
	-> Result<Vec<Constants>>
	{
		let url = url!("messenger", "constants");
		let body = String::new();
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<Constants>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Liefert eine Liste der aktuellen Konferenzen.
	///
	/// # Arguments
	///
	/// * `p_type`- Gibt Konferenzen an, die eine bestimmte Markierung oder einen bestimmten Typ haben.
	/// * `p_page`- Die Seite der Konferenzen. Default ist 0.
	pub fn get_conferences(&self, p_type: Option<ConferenceOption>, p_page: Option<u64>)
	-> Result<Vec<Conference>>
	{
		let url = url!("messenger", "conferences");
		let body = param_build!("type" => p_type, "p" => p_page);
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<Conference>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Informationen zu einer bestimmten Konferenz.
	///
	/// # Arguments
	///
	/// * `p_conference_id` - Die ID der Konferenz.
	pub fn get_conference_info(&self, p_conference_id: u64)
	-> Result<ConferenceInfo>
	{
		let url = url!("messenger", "conferenceinfo");
		let body = param_build!("conference_id" => Some(p_conference_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<ConferenceInfo> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Informationen zu einem bestimmten Benutzer.
	///
	/// # Arguments
	///
	/// * `p_user_id` - Die ID des betroffenen Benutzers.
	pub fn get_user_info(&self, p_user_id: u64)
	-> Result<UserInfo>
	{
		let url = url!("messenger", "userinfo");
		let body = param_build!("user_id" => Some(p_user_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<UserInfo> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Gibt die letzten Nachrichten einer Konferenz/eines Benutzers zurück.
	///
	/// # Arguments
	///
	/// * `p_conference_id` - Die Konferenz-ID.
	/// * `p_message_id` - Die Nachrichten-ID.
	/// * `p_read` - Ob eine Konferenz als gelesen markiert werden soll.
	/// Mögliche String-Werte: "true" (default) oder "false".
	pub fn get_messages(&self, p_conference_id: Option<u64>, p_message: Option<u64>, p_read: Option<bool>)
	-> Result<Vec<Messages>>
	{
		let url = url!("messenger", "messages");
		let body = param_build!("conference_id" => p_conference_id,
			"message" => p_message,
			"read" => p_read);
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<Messages>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Erstellt eine neue Unterhaltung, oder fügt eine Nachricht einer vorhandenen Unterhaltung hinzu (zwei Benutzer).
	///
	/// # Arguments
	///
	/// * `p_text` Eine Eingabenachricht. Beim Erstellen von Konferenzen werden Befehlseingaben ignoriert.
	/// * `p_username` Der Benutzername eines Proxer-Nutzers, an die eine Nachricht gesendet werden soll.
	pub fn new_conference(&self, p_text: String, p_username: String)
	-> Result<u64>
	{
		let url = url!("messenger", "newconference");
		let body = param_build!("text" => Some(p_text),
			"username" => Some(p_username));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<u64> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Erstellt eine neue Gruppen-Unterhaltung.
	///
	/// # Arguments
	///
	/// * `p_users` Ein Array aus Benutzernamen. Benutzer, die einer Konferenz hinzugefügt werden sollen.
	/// Eingeschränkt durch die Konstante userLimit.
	/// * `p_topic` Das Thema/der Name der Konferenz. Eingeschränkt durch die Konstante topicCount.
	/// * `p_text` Die erste Nachricht der neu erstellten Konferenz.
	/// Eingeschränkt durch die Konstante textCount. Beim Erstellen von Konferenzen werden Befehlseingaben ignoriert.
	pub fn new_conferencegroup(&self, p_users: String, p_tropic: String, p_text: Option<String>)
	-> Result<u64>
	{
		let url = url!("messenger", "newconferencegroup");
		let body = param_build!("users" => Some(p_users),
			"tropic" => Some(p_tropic),
			"text" => p_text);
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<u64> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Funktionalität zum Melden einer Konferenz bei der Proxer-Administration.
	///
	/// # Arguments
	///
	/// * `p_text` - Ein kurzer Grund, weshalb die Konferenz gemeldet wird.
	/// * `p_conference_id` ID der Konferenz, die gemeldet werden soll.
	pub fn report(&self, p_text: String, p_conference_id: u64)
	-> Result<()>
	{
		let url = url!("messenger", "report");
		let body = param_build!("text" => Some(p_text),
			"conference_id" => Some(p_conference_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: EmptyResponse = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		Ok(())
	}

	/// Sendet eine Nachricht an eine bestimmte Konferenz.
	///
	/// # Arguments
	///
	/// * `p_conference_id` - Die Konferenz-ID.
	/// * `p_text` - Eine Eingabenachricht. Die Nachricht ist eingeschränkt durch die Konstante textCount. Es gibt zwei Arten von Nachrichten:
	/// Normale Nachricht: Eine vom Benutzer abgesendete Nachricht.
	/// Eine Befehl: Ein Befehl fängt mit einem Schrägstrich an.
	/// Ein Beispiel für ein Befehl ist wie folgt: /addUser ProxerBot.
	/// Dieser Befehl würde den Benutzer ProxerBot zu der aktuellen Konferenz hinzufügen.
	pub fn set_message(&self, p_conference_id: u64, p_text: String)
	-> Result<String>
	{
		let url = url!("messenger", "setmessage");
		let body = param_build!("conference_id" => Some(p_conference_id),
			"text" => Some(p_text));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<String> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Markiert eine Konferenz als gelesen (read=true, read_count=0). Beachte bitte: Funktion Get Messages.
	///
	/// # Arguments
	///
	/// * `conference_id` - Die Konferenz-ID.
	pub fn set_read(&self, p_conference_id: u64)
	-> Result<()>
	{
		let url = url!("messenger", "setread");
		let body = param_build!("conference_id" => Some(p_conference_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: EmptyResponse = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		Ok(())
	}

	/// Markiert eine Konferenz als ungelesen. Beachte bitte: Funktion Get Messages.
	///
	/// # Arguments
	///
	/// * `conference_id` - Die Konferenz-ID.
	pub fn set_unread(&self, p_conference_id: u64)
	-> Result<()>
	{
		let url = url!("messenger", "setunread");
		let body = param_build!("conference_id" => Some(p_conference_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: EmptyResponse = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		Ok(())
	}

	/// Markiert eine Konferenz als blockiert.
	///
	/// # Arguments
	///
	/// * `conference_id` - Die Konferenz-ID.
	pub fn set_block(&self, p_conference_id: u64)
	-> Result<()>
	{
		let url = url!("messenger", "setblock");
		let body = param_build!("conference_id" => Some(p_conference_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: EmptyResponse = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		Ok(())
	}

	/// Hebt eine Blockierung einer Konferenz auf.
	///
	/// # Arguments
	///
	/// * `conference_id` - Die Konferenz-ID.
	pub fn set_unblock(&self, p_conference_id: u64)
	-> Result<()>
	{
		let url = url!("messenger", "setunblock");
		let body = param_build!("conference_id" => Some(p_conference_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: EmptyResponse = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		Ok(())
	}

	/// Markiert eine Konferenz als favorit.
	///
	/// # Arguments
	///
	/// * `conference_id` - Die Konferenz-ID.
	pub fn set_favour(&self, p_conference_id: u64)
	-> Result<()>
	{
		let url = url!("messenger", "setfavour");
		let body = param_build!("conference_id" => Some(p_conference_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: EmptyResponse = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		Ok(())
	}

	/// Hebt eine Favorisierung einer Konferenz auf.
	///
	/// # Arguments
	///
	/// * `conference_id` - Die Konferenz-ID.
	pub fn set_unfavour(&self, p_conference_id: u64)
	-> Result<()>
	{
		let url = url!("messenger", "setunfavour");
		let body = param_build!("conference_id" => Some(p_conference_id));
		let response = self.proxer.connect(&url, &body)?;
		let data: EmptyResponse = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		Ok(())
	}
}
