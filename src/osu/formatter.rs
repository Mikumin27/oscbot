use rosu_v2::prelude::{self as rosu, GameModIntermode};

pub fn map_title(map: &rosu::BeatmapExtended) -> String {
    let mapset = map.mapset.as_deref().expect("missing mapset");
    format!("{} - {} [{}]", mapset.artist, mapset.title, map.version)
}

pub fn game_mode_name(mode: rosu::GameMode) -> String {
    match mode {
        rosu::GameMode::Osu => "osu!standard".to_string(),
        rosu::GameMode::Mania => "osu!mania".to_string(),
        rosu::GameMode::Taiko => "osu!taiko".to_string(),
        rosu::GameMode::Catch => "osu!catch".to_string(),
    }
}

pub fn osu_hits(score_statistics: &rosu::ScoreStatistics, game_mode: &rosu::GameMode) -> String {
    match game_mode {
        rosu::GameMode::Osu => format!("{}/{}/{}/{}", score_statistics.great, score_statistics.ok, score_statistics.meh, score_statistics.miss),
        rosu::GameMode::Mania => format!("{}/{}/{}/{}/{}/{}", score_statistics.perfect, score_statistics.great, score_statistics.good, score_statistics.ok, score_statistics.meh, score_statistics.miss),
        rosu::GameMode::Taiko => format!("{}/{}/{}", score_statistics.great, score_statistics.ok, score_statistics.miss),
        rosu::GameMode::Catch => format!("{}/{}", score_statistics.great, score_statistics.miss),
    }
}

pub fn score_url(score_id: &u64) -> String {
    format!("https://osu.ppy.sh/scores/{}", score_id.to_string())
}

pub fn mods_string(mods: &rosu::GameMods) -> String {
    mods.iter().map(|map: &rosu::GameMod| map.acronym().to_string()).collect::<Vec<_>>().join("")
}

pub fn convert_osu_db_to_mod_array(mods: osu_db::ModSet) -> Vec<String> {
    let mut x = mods.bits();
    let mut mod_array: Vec<String> = Vec::new();
    while x != 0 {
        let bit = x & x.wrapping_neg();
        let intermode = GameModIntermode::try_from_bits(bit).unwrap();
        mod_array.push(intermode.acronym().as_str().to_string());
        x &= x - 1;
    };
    mod_array
}

pub fn calculate_grade_from_accuracy(accuracy: f32, has_miss: bool, hidden: bool) -> rosu::Grade {
    if accuracy == 100.0 {
        return if hidden {rosu::Grade::XH} else {rosu::Grade::X};
    }

    let true_accuracy = if has_miss {accuracy} else {accuracy + 10.0};

    if 70.0 > true_accuracy {
        return rosu::Grade::D;
    }
    else if 80.0 > true_accuracy {
        return rosu::Grade::C
    }

    else if 90.0 > true_accuracy {
        return rosu::Grade::B;
    }

    else if accuracy > 90.0 && !has_miss {
        return if hidden {rosu::Grade::SH} else {rosu::Grade::S};
    }

    return rosu::Grade::A;
}