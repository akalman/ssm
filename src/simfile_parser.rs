use simfile;

pub fn parse_simfile<'a>(path: String) -> simfile::Simfile<'a> {
    return simfile::Simfile {
        title: "",
        title_translit: "",

        subtitle: "",
        subtitle_translit: "",

        artist: "",
        artist_translit: "",

        genre: "",
        credit: "",

        banner: "",
        background: "",
        music: "",
        offset: "",

        notes: &[]
    };
}