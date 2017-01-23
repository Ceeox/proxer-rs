use std::io::Error as IoError;
use hyper::error::Error as HyperError;
use serde_json::error as serde;

#[derive(Debug)]
pub enum Error
{
	Hyper(HyperError),
	SerdeError(serde::Error),
	Io(IoError),
	ProxerError(u16, String),
	Other(String),
}

impl From<HyperError> for Error
{
	fn from(err: HyperError)
	-> Error
	{
		error!("HyperError with: {:?}", err);
		Error::Hyper(err)
	}
}

impl From<serde::Error> for Error
{
	fn from(err: serde::Error)
	-> Error
	{
		error!("SerdeError with: {:?}", err);
		Error::SerdeError(err)
	}
}

impl From<IoError> for Error
{
	fn from(err: IoError)
	-> Error
	{
		error!("IoError with: {:?}", err);
		Error::Io(err)
	}
}

impl From<(u16, String)> for Error
{
	fn from((code, message): (u16, String))
	-> Error
	{
		error!("ProxerError with: message:{:?}, code:{}({:?})",
			message, code, DecodeErrorCode::decode(code));
		Error::ProxerError(code, message)
	}
}

impl From<String> for Error
{
	fn from(err: String)
	-> Error
	{
		error!("OtherError with {:?}", err);
		Error::Other(err)
	}
}

pub struct DecodeErrorCode;

impl DecodeErrorCode
{
	pub fn decode(p_code: u16)
	-> String
	{
		match p_code
		{
			1000 =>	format!("API-Version existiert nicht."),
			1001 =>	format!("API-Version wurde entfernt."),
			1002 =>	format!("API-Klasse existiert nicht."),
			1003 =>	format!("API-Funktion existiert nicht."),
			1004 =>	format!("Der API-Schlüssel besitzt nicht ausreichend Rechte um diese Aktion durchzuführen."),
			1005 =>	format!("Es wurde ein ungültiges Login-Token verwendet."),
			1006 =>	format!("Die aufgerufene Funktion wurde gesperrt."),

			2000 =>	format!("IP von Firewall geblockt."),
			2001 =>	format!("News: Fehler bei der Abfrage der News."),

			3000 =>	format!("Login: Fehlende Login-Daten."),
			3001 =>	format!("Login: Ungültige Login-Daten."),
			3002 =>	format!("Notifications: User nicht eingeloggt."),
			3003 =>	format!("Userinfo: Userid existiert nicht."),
			3004 =>	format!("Ucp: User nicht eingeloggt."),
			3005 =>	format!("Ucp: Kategorie existiert nicht."),
			3006 =>	format!("Ucp: Ungültige ID."),
			3007 =>	format!("Info: Ungültige ID."),
			3008 =>	format!("Info: setUserInfo: Ungültiger Typ."),
			3009 =>	format!("Info: setUserInfo: User nicht eingeloggt."),
			3010 =>	format!("Info: setUserInfo: Werk bereits in Liste enthalten."),
			3011 =>	format!("Info: setUserInfo: Anzahl zulässiger Favoriten überschritten."),
			3012 =>	format!("Login: Der User ist bereits eingeloggt."),
			3013 =>	format!("Login: Ein anderer User ist bereits eingeloggt."),
			3014 =>	format!("User: Der Zugriff auf die gesuchte Information wurde verweigert (möglicherweise sollte ein User eingeloggt werden)."),
			3015 =>	format!("List: Kategorie existiert nicht."),
			3016 =>	format!("List: Medium existiert nicht."),
			3017 =>	format!("Media: Stil existiert nicht."),
			3018 =>	format!("Media: Eintrag existiert nicht."),
			3019 =>	format!("Manga: Kapitel existiert nicht (nicht hochgeladen)."),
			3020 =>	format!("Anime: Episode existiert nicht (keine Streams)."),
			3021 =>	format!("Anime: Stream existiert nicht."),
			3022 =>	format!("Ucp: Episode existiert nicht."),
			3023 =>	format!("Messages: Der User ist nicht eingeloggt."),
			3024 =>	format!("Messages: Ungültige Konferenz (fehlende Berechtigung oder fehlerhafte Konferenz-ID)."),
			3025 =>	format!("Messages: Ungültige/Fehlende Eingabe bei Meldegrund."),
			3026 =>	format!("Messages: Ungültige/Fehlende Nachricht."),
			3027 =>	format!("Messages: Ungültiger Benutzer."),
			3028 =>	format!("Messages: Die maximale Anzahl an Usern wurde erreicht."),
			3029 =>	format!("Messages: Ungültiges/Fehlendes Thema."),
			3030 =>	format!("Messages: Es muss mindestens ein Benutzer in einer Konferenz hinzugefügt werden."),
			3031 =>	format!("Chat: Ungültiger Raum."),
			3032 =>	format!("Chat: Keine Berechtigungen."),
			3033 =>	format!("Chat: Ungültige Nachricht."),
			3034 =>	format!("Chat: Nicht eingeloggt."),
			3035 =>	format!("List: Ungültige Sprache."),
			3036 =>	format!("List: Ungültiger Typ."),
			3037 =>	format!("List: Ungültige ID."),

			_ => format!("Unknown Code")
		}
	}
}
