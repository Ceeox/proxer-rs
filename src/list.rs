use serde_json;

use ::error::Error;
use ::Proxer;
use ::models::*;

/// Diese Funktion erfüllt die Aufgabe der erweiterten Suche
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntrySearch
{
	/// Die ID des Entrys
	pub id: u64,
	/// Der Name des Entrys (Der selbe Entry kann mit unterschiedlichen Namen mehrfach auftreten)
	pub name: String,
	/// Die Genre des Entrys (Ein durch Leerzeichen getrennter String, jedes Wort ist ein Genre)
	pub genre: Vec<String>,
	/// Das Medium des Entrys (animeseries,movie,ova,hentai,mangaseries,oneshot,doujin,hmanga)
	pub medium: Medium,
	/// Die Anzahl der Kapitel/Episoden (Müssen nicht hochgeladen sein!)
	pub count: u64,
	/// Ein Integer-Wert, der den Status des Entrys beschreibt
	/// 0: Nicht Erschienen (Pre-Airing)
	/// 1: Abgeschlossen
	/// 2: Airing/Am Laufen
	/// 3: Abgebrochen
	/// 4: Abgeschlossen/Nicht fertiggesubbt
	pub state: u8,
	/// Die Summe aller Bewertungen
	pub rate_sum: u64,
	/// Die Anzahl der Bewertungen
	pub rate_count: u64,
	/// Die Sprachen, in denen der Entry verfügbar ist, als Komma-separierter String
	pub language: Vec<String>,
}
/// Diese Funktion liefert eine Liste aller Entrys einer Kategorie mit bestimmten Einschränkungsmöglichkeiten.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EntryList
{
	/// Die ID des Entrys
	pub id: u64,
	/// Der Name des Entrys (Der selbe Entry kann mit unterschiedlichen Namen mehrfach auftreten)
	pub name: String,
	/// Die Genre des Entrys (Ein durch Leerzeichen getrennter String, jedes Wort ist ein Genre)
	pub genre: Vec<String>,
	/// Das Medium des Entrys (animeseries,movie,ova,hentai,mangaseries,oneshot,doujin,hmanga)
	pub medium: Medium,
	/// Die Anzahl der Kapitel/Episoden (Müssen nicht hochgeladen sein!)
	pub count: u64,
	/// Ein Integer-Wert, der den Status des Entrys beschreibt
	/// 0: Nicht Erschienen (Pre-Airing)
	/// 1: Abgeschlossen
	/// 2: Airing/Am Laufen
	/// 3: Abgebrochen
	/// 4: Abgeschlossen/Nicht fertiggesubbt
	pub state: u64,
	/// Die Summe aller Bewertungen
	pub rate_sum: u64,
	/// Die Anzahl der Bewertungen
	pub rate_count: u64,
	/// Die Sprachen, in denen der Entry verfügbar ist, als Komma-separierter String
	pub language: Vec<String>,
}

/// Diese Funktion zieht aus einem String die IDs aller darin vorkommenden Tags und gibt sie zurück.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagIDs
{
	/// Ein Array, dass die IDs aller vorkommenden Tags ohne Minus ("-") enthält.
	pub tags: Vec<String>,
	/// Ein Array, dass die IDs aller vorkommenden Tags mit Minus ("-") enthält.
	pub notags: Vec<String>,
}

/// Diese Funktion liefert eine Liste aller Tags, anhand bestimmter Kriterien.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag
{
	/// Die ID des Tags
	pub id: u64,
	/// Der Typ des Tags (entry_genre,entry_tag,entry_tag_h,gallery)
	#[serde(rename="type")]
	pub tag_type: String,
	/// Der Name des Tags
	pub tag: String,
	/// Die Beschreibung des Tags
	pub description: String,
	/// Ob der Tag verwendet werden kann (Dieser Wert ist immer gleich 0, da geblacklistete Tags nicht ausgegeben werden)
	pub blacklist: u8,
	/// In welche Kategorie der Tag gehört. (Arten siehe Eingabeparameter)
	pub subtype: Kategorie,
}

/// Diese Funktion liefert eine Liste aller Sub/Scanlation Gruppen, anhand bestimmter Kriterien.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranslatorGroup
{
	/// Die id der Gruppe
	pub id: u64,
	/// Der Name der Gruppe
	pub name: String,
	/// Die Sprache der Gruppe (misc, de, en)
	pub country: String,
	/// Ein Bildlink für ein Bild der Gruppe (kann leer sein)
	pub image: Option<String>,
}

/// Diese Funktion liefert eine Liste aller Firmen, anhand bestimmter Kriterien.
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
	/// Folgender Link enthält das Coverbild einer Firma: ///cdn.proxer.me/industry/<id>.jpg
	pub link: String,
}

/// Diese Funktion liefert eine Liste aller Projekte (=Entrys) einer Gruppe anhand ihrer ID.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TranslatorGroupProject
{
	/// Die id des Entrys
	pub id: u64,
	/// Der Name des Entrys (Originalname)
	pub name: String,
	/// Die Genre des Entrys (Ein durch Leerzeichen getrennter String, jedes Wort ist ein Genre)
	pub genre: String,
	/// Die Gefahrensymbole des Entrys (Ein durch Leerzeichen getrennter String, jedes Wort ist ein Gefahrensymbol)
	pub fsk: String,
	/// Das Medium des Entrys (animeseries,movie,ova,hentai,mangaseries,oneshot,doujin,hmanga)
	pub medium: Medium,
	/// Der Status der Subgruppe (Werte siehe Parameter)
	#[serde(rename="type")]
	pub trans_group_type: String,
	/// Ein Integer-Wert, der den Status des Entrys beschreibt
	/// 0: Nicht Erschienen (Pre-Airing)
	/// 1: Abgeschlossen
	/// 2: Airing/Am Laufen
	/// 3: Abgebrochen
	/// 4: Abgeschlossen/Nicht fertiggesubbt
	pub state: u8,
	/// Die Summe aller Bewertungen
	pub rate_sum: u64,
	/// Die Anzahl der Bewertungen
	pub rate_count: u64,
}

/// Diese Funktion liefert eine Liste aller Projekte (=Entrys) einer Firma anhand ihrer ID.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndustrysProject
{
	/// Die id des Entrys
	pub id: u64,
	/// Der Name des Entrys (Originalname)
	pub name: String,
	/// Die Genre des Entrys (Ein durch Leerzeichen getrennter String, jedes Wort ist ein Genre)
	pub genre: String,
	/// Die Gefahrensymbole des Entrys (Ein durch Leerzeichen getrennter String, jedes Wort ist ein Gefahrensymbol)
	pub fsk: String,
	/// Das Medium des Entrys (animeseries,movie,ova,hentai,mangaseries,oneshot,doujin,hmanga)
	pub medium: Medium,
	/// Der Typ der Firma (Werte siehe Parameter)
	#[serde(rename="type")]
	pub industy_type: String,
	/// Ein Integer-Wert, der den Status des Entrys beschreibt
	/// 0: Nicht Erschienen (Pre-Airing)
	/// 1: Abgeschlossen
	/// 2: Airing/Am Laufen
	/// 3: Abgebrochen
	/// 4: Abgeschlossen/Nicht fertiggesubbt
	pub state: u8,
	/// Die Summe aller Bewertungen
	pub rate_sum: u64,
	/// Die Anzahl der Bewertungen
	pub rate_count: u64,
}

/// Diese Klasse dient als Hauptmethode um die Daten von Entrys zu erhalten,
/// insbesondere der ID (die für jede spezielle Entry Schnittstelle nötig ist).
/// Sie enthält zwei Arten von Schnittstellen: Eine "Search" Schnittstelle,
/// die wie die Erweiterte Suche auf Proxer funktioniert, sowie Auflistungs-Schnittstellen,
/// die Animelisten nach bestimmten Schemata erstellen.
#[derive(Debug)]
pub struct List<'list>
{
	proxer: &'list Proxer,
}

impl<'list> List<'list>
{
	#[doc(hidden)]
	pub fn new(p_proxer: &'list Proxer)
	-> List<'list>
	{
		List
		{
			proxer:		p_proxer,
		}
	}

	/// Diese Funktion erfüllt die Aufgabe der erweiterten Suche
	///
	/// # Arguments
	///
	/// * `p_name` - Ein zu suchender Entryname. Die Schnittstelle sucht sowohl nach exakten Matches
	/// als auch nach Teil-Matches im Titel des Entrys (oder einem seiner Synonyme)
	/// * `p_language` - Die zu suchende Sprache. Erlaubte Werte: "de","en". Default: Beide.
	/// * `p_type` - Der zu suchende Typ. Erlaubte Werte:
	/// 'p_animeseries', 'movie', 'ova', 'mangaseries', 'oneshot', 'doujin', 'hentai', 'hmanga',
	/// 'p_all-anime'(kein H), 'all-manga'(kein H), 'all' (Default, kein H), 'all18'(H)
	/// * `p_genre` - Genre, die der Entry enthalten soll. Als Leerzeichen (oder Plus "+")
	/// separierter String, Genre-Namen wie in Suche. (Also ausgeschrieben, z.B. "Action") (Kein Effekt wenn leer)
	/// * `p_nogenre` - Genre, die der Entry nicht enthalten darf. Angabe wie genre. (Kein Effekt wenn leer)
	/// * `p_fsk` - Die zu suchenden Gefahrensymbole/Altersbeschränkungen. Erlaubte Werte:
	/// "fsk0", "fsk6", "fsk12", "fsk16", "fsk18", "bad_language", "violence", "fear", "sex".
	/// Anzugeben als Leerzeichen oder Plus ("+") separierter String. (Kein Effekt wenn leer)
	/// * `p_sort` - Wie die Ergebnisse sortiert werden sollen. Erlaubte Werte:
	/// "relevance" (Qualität des Name-Matches, Default), "clicks", "rating"
	/// (Sortierung erst nach Anzahl und dann Wertung der Stimmen), "count" (Anzahl der Kapitel/Episoden),
	/// "name" (Alphabetisch)
	/// * `p_length` - Die Anzahl der Kapitel/Episoden (Müssen nicht hochgeladen sein!),
	/// die ein Entry mindestens/höchstens haben darf. Zwischen 0 und 400 (exklusive). (Kein Effekt wenn leer)
	/// * `p_length-limit` - Ob der Parameter "length" als Minimal- oder Maximalwert verwendet werden soll.
	/// Erlaubte Werte: "up" (größer gleich), "down" (kleiner gleich, Default).
	/// Hat keinen Effekt wenn "length" nicht korrekt gesetzt ist.
	/// * `p_tags` - Tags, die der Entry enthalten soll. Als Leerzeichen (oder Plus "+")
	/// separierter String von Tag-Ids. (Kein Effekt wenn leer)
	/// * `p_notags` - Tags, die der Entry nicht enthalten darf.
	/// Als Leerzeichen (oder Plus "+") separierter String von Tag-Ids. (Kein Effekt wenn leer)
	/// * `p_tagratefilter` - Welche Art Tags berücksichtigt werden soll (negativ als auch positiv),
	/// "rate_1" für nur eingetragene Tags (Default), "rate_10" für zusätzlich Unbestimmte Tags.
	/// * `p_tagspoilerfilter` - Inwieweit Spoiler-Tags berücksichtigt werden sollen (negativ als auch positiv),
	/// "spoiler_0" für keine Spoiler (Default), "spoiler_10" für Spoiler und Nicht-Spoiler, "spoiler_1" für nur Spoiler.
	/// * `p_page` - Die zu ladende Seite, Beginn bei 0, Default 0.
	/// * `p_limit` - Wie viele Einträge eine Seite enthalten soll. Default 100.
	pub fn entry_search(&self,
		p_name: Option<String>,
		p_language: Option<String>,
		p_type: Option<Medium>,
		p_genre: Option<String>,
		p_nogenre: Option<String>,
		p_fsk: Option<String>,
		p_sort: Option<SearchSort>,
		p_length: Option<u64>,
		p_lengthlimit: Option<LengthLimit>,
		p_tags: Option<String>,
		p_notags: Option<String>,
		p_tagratefilter: Option<String>,
		p_tagspoilerfilter: Option<String>,
		p_page: Option<u64>,
		p_limit: Option<u64>)
	-> Result<Vec<EntrySearch>, Error>
	{
		let url = url!("list", "entrysearch");
		let body = param_build!("api_key" => Some(&self.proxer.api_key),
			"name" => p_name,
			"language" => p_language,
			"type" => p_type,
			"genre" => p_genre,
			"nogenre" => p_nogenre,
			"fsk" => p_fsk,
			"sort" => p_sort,
			"length" => p_length,
			"length-limit" => p_lengthlimit,
			"tags" => p_tags,
			"notags" => p_notags,
			"tagratefilter" => p_tagratefilter,
			"tagspoilerfilter" => p_tagspoilerfilter,
			"p" => p_page,
			"limit" => p_limit);
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<EntrySearch>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}
	/// Diese Funktion liefert eine Liste aller Entrys einer Kategorie mit bestimmten Einschränkungsmöglichkeiten.
	///
	/// # Arguments
	///
	/// * `p_kat` - Die Kategorie des Entrys. Erlaubte Werte: "anime","manga". Default: "anime"
	/// * `p_medium` - Welche Art medium geladen werden soll.
	/// Erlaubte Werte: "animeseries", "movie", "ova", "hentai", "mangaseries", "oneshot", "doujin", "hmanga".
	/// (Kein Effekt wenn leer)
	/// * `p_is_h` - Ob die Liste Hentais enthalten soll. "true" für Hentai-Liste, sonst non-Hentai Liste (default).
	/// Dieser Parameter hat keinen Effekt wenn "medium" gesetzt ist
	/// * `p_start` - Mit welchem String der Name der Entrys beginnen soll.
	/// Nützlich, um z.B. nach Anfangsbuchstaben zu filtern.
	/// Um nach nicht-alphabetischen Anfängen (Erstes Zeichen) zu filtern, 'nonAlpha' angeben. (Kein Effekt wenn leer)
	/// * `p_page` - Die zu ladende Seite, Beginn bei 0, Default 0.
	/// * `p_limit` - Wie viele Einträge eine Seite enthalten soll. Default 100.
	pub fn get_entrylist(&self,
		p_kat: Option<Kategorie>,
		p_medium: Option<Medium>,
		p_is_h: Option<bool>,
		p_start: Option<String>,
		p_page: Option<u64>,
		p_limit: Option<u64>)
	-> Result<Vec<EntryList>, Error>
	{
		let url = url!("list", "entrylist");
		let body = param_build!("api_key" => Some(&self.proxer.api_key),
			"kat" => p_kat,
			"medium" => p_medium,
			"isH" => p_is_h,
			"start" => p_start,
			"p" => p_page,
			"limit" => p_limit);
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<EntryList>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion zieht aus einem String die IDs aller darin vorkommenden Tags und gibt sie zurück.
	///
	/// # Arguments
	///
	/// * `search` - Ein beliebiger String. Sollte sinnvollerweise Tags enthalten,
	/// einzelne Tags durch Leerzeichen getrennt (also keine Kommata zur Trennung oder dergleichen verwenden).
	/// Es können nur Tags erkannt werden, vor und nach denen ein Leerzeichen bzw der Anfang/das Ende des Strings sind.
	/// Zudem darf vor einem Tag (Also nach dem Leerzeichen/Beginn des Strings vor dem Tag) ein Minus ("-") stehen.
	/// Solcherart gekennzeichnete Tags werden gesondert ausgegeben.
	pub fn get_tag_ids(&self, p_search: String)
	-> Result<TagIDs, Error>
	{
		let url = url!("list", "tagids");
		let body = param_build!("api_key" => Some(&self.proxer.api_key), "search" => Some(p_search));
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<TagIDs> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert eine Liste aller Tags, anhand bestimmter Kriterien.
	///
	/// # Arguments
	///
	/// * `p_search` - Ein beliebiger String. Nur Tags,
	/// deren Name oder Beschreibung diesen String als Substring haben oder gleich ihm sind sind werden ausgegeben.
	/// * `p_type` - Welcher Typ Tag angezeigt werden soll.
	/// Mögliche Werte: entry_genre (Genres, momentan nicht in Benutzung), entry_tag (normale Tags),
	/// entry_tag_h (H-tags), gallery (Gallerie Tags, momentan nicht in Benutzung). Default: Alles außer H-Tags.
	/// * `p_sort` - Nach welchem Ausgabeparameter die Liste sortiert werden soll.
	/// Erlaubte Werte: id, type, tag, description, blacklist, subtype. Default: tag
	/// * `p_sort_type` - In welcher Reihenfolge die Sortierung ist. 'ASC' für aufsteigend,
	/// 'DESC' für absteigend (Jeder andere Wert wird zu DESC). Default: ASC
	/// * `p_subtype` - Die Kategorie des Tags
	pub fn get_tags(&self, p_search: Option<String>,
		p_type: Option<String>,
		p_sort: Option<String>,
		p_sort_type: Option<String>,
		p_sub_type: Option<SubType>)
	-> Result<Vec<Tag>, Error>
	{
		let url = url!("list", "tags");
		let body = param_build!("api_key" => Some(&self.proxer.api_key),
			"search" => p_search,
			"type" => p_type,
			"sort" => p_sort,
			"p_sort_type" => p_sort_type,
			"p_sub_type" => p_sub_type);
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<Tag>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert eine Liste aller Sub/Scanlation Gruppen, anhand bestimmter Kriterien.
	///
	/// # Arguments
	///
	/// * `p_start` - Mit welchem String der Name der Gruppen beginnen soll.
	/// Nützlich, um z.B. nach Anfangsbuchstaben zu filtern.
	/// Um nach nicht-alphabetischen Anfängen (Erstes Zeichen) zu filtern, 'nonAlpha' angeben. (Kein Effekt wenn leer)
	/// * `p_contains` - Nur Gruppen, die diesen String als Substring ihres Namens haben, werden ausgegeben.
	/// (Kein Effekt wenn leer)
	/// country (optional): Ermöglich, Gruppen nach Sprache zu filtern,
	/// erlaubte Werte: "de", "en", "misc". Default: Alle.
	/// * `p_page` - Die zu ladende Seite, Beginn bei 0, Default 0.
	/// * `p_limit` - Wie viele Einträge eine Seite enthalten soll. Default 100.
	pub fn get_translatorgroups(&self, p_start: Option<String>,
		p_contains: Option<String>,
		p_page: Option<u64>,
		p_limit: Option<u64>)
	-> Result<Vec<TranslatorGroup>, Error>
	{
		let url = url!("list", "translatorgroups");
		let body = param_build!("api_key" => Some(&self.proxer.api_key),
			"start" => p_start,
			"contains" => p_contains,
			"p" => p_page,
			"limit" => p_limit);
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<TranslatorGroup>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert eine Liste aller Firmen, anhand bestimmter Kriterien.
	///
	/// # Arguments
	///
	/// * `p_start` - Mit welchem String der Name der Firma beginnen soll.
	/// Nützlich, um z.B. nach Anfangsbuchstaben zu filtern.
	/// Um nach nicht-alphabetischen Anfängen (Erstes Zeichen) zu filtern, 'nonAlpha' angeben.(Kein Effekt wenn leer)
	/// * `p_contains` - Nur Firmen, die diesen String als Substring ihres Namens haben,
	/// werden ausgegeben. (Kein Effekt wenn leer)
	/// * `p_country` - Ermöglich, Firmen nach Sprache zu filtern,
	/// erlaubte Werte: "de", "us", "jp", "misc". Default: Alle.
	/// * `p_type` - Ermöglicht, Firmen nach Typ zu filtern,
	///  erlaubte Werte: 'publisher','studio','producer','record_label','talent_agent','streaming'. Default: Alle.
	/// * `p_page` - Die zu ladende Seite, Beginn bei 0, Default 0.
	/// * `p_limit` - Wie viele Einträge eine Seite enthalten soll. Default 100.
	pub fn get_industrys(&self,
		p_start: Option<String>,
		p_contains: Option<String>,
		p_country: Option<String>,
		p_type: Option<Firma>,
		p_page: Option<u64>,
		p_limit: Option<u64>,)
	-> Result<Vec<Industry>, Error>
	{
		let url = url!("list", "industrys");
		let body = param_build!("api_key" => Some(&self.proxer.api_key),
			"start" => p_start,
			"contains" => p_contains,
			"country" => p_country,
			"type" => p_type,
			"p" => p_page,
			"limit" => p_limit);
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<Industry>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert eine Liste aller Projekte (=Entrys) einer Gruppe anhand ihrer ID.
	///
	/// # Arguments
	///
	/// * `p_id` - Die ID der gewünschten Gruppe.
	/// * `p_type` - Ein Integer. Wenn gesetzt werden nur Entrys des gegebenen Übersezungs-Status ausgegeben
	/// (0=undefined, 1=abgeschlossen, 2=am laufen, 3=Geplant, 4=abgebrochen, 5=lizenziert). Default: Alle.
	/// * `p_is_h` - Ein Integer. Steuert die Ausgabe von H-Inhalten. Werte:
	/// -1 (kein H, Default), 0 (beides), 1 (nur H)
	/// * `p_page` - Die zu ladende Seite, Beginn bei 0, Default 0.
	/// * `p_limit` - Wie viele Einträge eine Seite enthalten soll. Default 100.
	pub fn get_translatorgroups_projects(&self,
		p_id: u64,
		p_type: Option<TranslationStatus>,
		p_is_h: Option<i8>,
		p_page: Option<u64>,
		p_limit: Option<u64>,)
	-> Result<Vec<TranslatorGroupProject>, Error>
	{
		let url = url!("list", "translatorgroupprojects");
		let body = param_build!("api_key" => Some(&self.proxer.api_key),
			"id" => Some(p_id),
			"type" => p_type,
			"isH" => p_is_h,
			"p" => p_page,
			"limit" => p_limit);
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<TranslatorGroupProject>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}

	/// Diese Funktion liefert eine Liste aller Projekte (=Entrys) einer Firma anhand ihrer ID.
	///
	/// # Arguments
	///
	/// * `p_id` - Die ID der gewünschten Firma.
	/// * `p_type` - Ermöglicht, Entrys nach Typ der Firma zu filtern
	/// (eine Firma kann als verschiedene Typen auftreten),
	/// erlaubte Werte: 'publisher','studio','producer','record_label','talent_agent','streaming'. Default: Alle.
	/// * `p_is_h` - Ein Integer. Steuert die Ausgabe von H-Inhalten.
	/// Werte: -1 (kein H, Default), 0 (beides), 1 (nur H)
	/// * `p_page` - Die zu ladende Seite, Beginn bei 0, Default 0.
	/// * `p_limit` - Wie viele Einträge eine Seite enthalten soll. Default 100.
	pub fn get_industry_projects(&self,
		p_id: u64,
		p_type: Option<Firma>,
		p_is_h: Option<i8>,
		p_page: Option<u64>,
		p_limit: Option<u64>)
	-> Result<Vec<IndustrysProject>, Error>
	{
		let url = url!("list", "industryprojects");
		let body = param_build!("api_key" => Some(&self.proxer.api_key),
			"id" => Some(p_id),
			"type" => p_type,
			"isH" => p_is_h,
			"p" => p_page,
			"limit" => p_limit);
		let response = self.proxer.connect(&url, &body)?;
		let data: Response<Vec<IndustrysProject>> = serde_json::from_reader(response)?;
		check_error!(data.error, data.code.unwrap_or_default(), data.message);
		check_data!(data.data)
	}
}
