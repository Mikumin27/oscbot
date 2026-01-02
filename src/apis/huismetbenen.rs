use rosu_v2::prelude as rosu;
use osu_db::Replay;

use reqwest::Client;
use serde::Serialize;
use serde_json::Value;

use crate::osu::formatter::convert_osu_db_to_mod_array;

#[derive(Serialize)]
struct Payload {
    map_id: u32,
    great: u32,
    ok: u32,
    meh: u32,
    miss: u32,
    large_tick_misses: u32,
    slider_tail_misses: u32,
    combo: u32,
    mods: Vec<String>,
    rework: String
}

pub struct CalculateScoreResponse {
    pub accuracy: f32,
    pub pp: f32,
    pub star_rating: f32,
}

pub async fn calculate_score_by_score(score: &rosu::Score) -> CalculateScoreResponse {
    let mods: Vec<String> = score.mods.iter().map(|beatmap| beatmap.acronym().to_string()).collect();
    
    let payload = Payload {
        map_id: score.map_id,
        great: score.statistics.great,
        ok: score.statistics.ok,
        meh: score.statistics.meh,
        miss: score.statistics.miss,
        large_tick_misses: score.statistics.large_tick_miss,
        slider_tail_misses: 0,
        combo: score.max_combo,
        mods: mods,
        rework: "live".to_string()
    };
    calculate_score(payload).await
}

pub async fn calculate_score_by_replay(replay: &Replay, map: &rosu::BeatmapExtended) -> CalculateScoreResponse {
    let mods = convert_osu_db_to_mod_array(replay.mods);

    let payload = Payload {
        map_id: map.map_id,
        great: replay.count_300 as u32,
        ok: replay.count_100 as u32,
        meh: replay.count_50 as u32,
        miss: replay.count_miss as u32,
        large_tick_misses: 0,
        slider_tail_misses: 0,
        combo: replay.max_combo as u32,
        mods: mods,
        rework: "live".to_string()
    };
    calculate_score(payload).await
}

async fn calculate_score(payload: Payload) -> CalculateScoreResponse {
    let client = Client::new();
    let url = "https://api.pp.huismetbenen.nl/calculate-score";


     let res = client
        .post(url)
        .json(&payload)
        .send()
        .await.unwrap()
        .error_for_status()
        .unwrap();


    let result: Value = res.json().await.expect("response to have JSON");
    let score = result.get("score").unwrap();
    let difficulty_attributes = result.get("difficulty_attributes").unwrap();
    CalculateScoreResponse {
        accuracy: score.get("accuracy").expect("accuracy to exist").as_f64().unwrap() as f32,
        pp: result.get("live_pp").expect("live_pp to exist").as_f64().unwrap() as f32,
        star_rating: difficulty_attributes.get("star_rating").expect("star_rating to exist").as_f64().unwrap() as f32,
    }
}