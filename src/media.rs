use serde_json;

use ::error::*;
use ::Proxer;
use ::models::*;

/// Diese Funktion liefert einen zufälligen Header.
/// Bildpfad: //cdn.proxer.me/gallery/originals/<catpath>/<imgfilename>
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct RandomHeader
{
    /// Die ID des Headers in der Gallerie
    pub gid: u64,
    /// Der Pfad zum Bild
    pub catpath: String,
    /// Der Name des Bildes
    pub imgfilename: String,
}

impl RandomHeader
{
    pub fn get_picture_link(&self)
    -> String
    {
        format!("https://cdn.proxer.me/gallery/originals/{}/{}", self.catpath, self.imgfilename)
    }
}

/// Diese Funktion liefert eine Liste aller aktuellen Header.
/// Bildpfad: //cdn.proxer.me/gallery/originals/<catpath>/<imgfilename>
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct HeaderList
{
    /// Die ID des Headers in der Gallerie
    pub gid: u64,
    /// Der Pfad zum Bild
    pub catpath: String,
    /// Der Name des Bildes
    pub imgfilename: String,
}

impl HeaderList
{
    pub fn get_picture_link(&self)
    -> String
    {
        format!("https://cdn.proxer.me/gallery/originals/{}/{}", self.catpath, self.imgfilename)
    }
}

/// Diese Klasse dient dazu, verschiedene Medien von Proxer zu erhalten.
/// Dabei sind Animes und Mangas explizit ausgeschlossen, diese werden in eigenen Klassen behandelt.
#[derive(Debug)]
pub struct Media<'media>
{
    proxer: &'media Proxer,
}

impl<'media> Media<'media>
{
    #[doc(hidden)]
    pub fn new(p_proxer: &'media Proxer)
    -> Media<'media>
    {
        Media
        {
            proxer: p_proxer,
        }
    }

    /// Diese Funktion liefert einen zufälligen Header.
    /// Bildpfad: //cdn.proxer.me/gallery/originals/<catpath>/<imgfilename>
    pub fn get_randomheader(&self, p_style: Option<String>)
    -> Result<Vec<RandomHeader>>
    {
        let url = url!("media", "randomheader");
        let body = param_build!("style" => p_style);
        let response = self.proxer.connect(&url, &body)?;
        let data: Response<Vec<RandomHeader>> = serde_json::from_reader(response)?;
        check_error!(data.error, data.code.unwrap_or_default(), data.message);
        check_data!(data.data)
    }

    /// Diese Funktion liefert eine Liste aller aktuellen Header.
    /// Bildpfad: //cdn.proxer.me/gallery/originals/<catpath>/<imgfilename>
    pub fn get_headerlist(&self)
    -> Result<Vec<HeaderList>>
    {
        let url = url!("media", "headerlist");
        let body = String::new();
        let response = self.proxer.connect(&url, &body)?;
        let data: Response<Vec<HeaderList>> = serde_json::from_reader(response)?;
        check_error!(data.error, data.code.unwrap_or_default(), data.message);
        check_data!(data.data)
    }
}
