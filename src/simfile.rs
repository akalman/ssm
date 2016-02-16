use std::collections::LinkedList;

pub struct Simfile<'a> {
    pub title: &'static str,
    pub title_translit: &'static str,

    pub subtitle: &'static str,
    pub subtitle_translit: &'static str,

    pub artist: &'static str,
    pub artist_translit: &'static str,

    pub genre: &'static str,
    pub credit: &'static str,

    pub banner: &'static str,
    pub background: &'static str,
    pub music: &'static str,
    pub offset: &'static str,

    pub notes: &'a [SimfileNoteset<'a>]
}

pub struct SimfileNoteset<'a> {
    noteset_type: &'static str,
    description: &'static str,
    difficulty: &'static str,
    feet: &'static str,
    groove_radar: &'static str,
    measures: LinkedList<SimfileMeasure<'a>>
}

pub struct SimfileMeasure<'a> {
    subdivision: &'static str,
    notes: &'a [SimfileBeat]
}

pub struct SimfileBeat {
    left_track: SimfileNote,
    up_track: SimfileNote,
    down_track: SimfileNote,
    right_track: SimfileNote
}

pub struct SimfileNote {
    note_type: &'static str
}
