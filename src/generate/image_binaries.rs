use rosu_v2::{model::Grade, prelude::GameMod};

pub const TEMPLATE_MAIN: &[u8] = include_bytes!("./data/templates/main.png");

pub const DEFAULT_BACKGROUND: &[u8] = include_bytes!("./data/background/default.jpg");

pub const FONT_ALLER_BD: &[u8] = include_bytes!("./data/font/Aller_Bd.ttf");

pub const RANK_S: &[u8] = include_bytes!("./data/rank/S.png");

pub const RANK_SILVER_S: &[u8] = include_bytes!("./data/rank/SH.png");

pub const RANK_SS: &[u8] = include_bytes!("./data/rank/X.png");

pub const RANK_SILVER_SS: &[u8] = include_bytes!("./data/rank/XH.png");

pub const RANK_A: &[u8] = include_bytes!("./data/rank/A.png");

pub const RANK_B: &[u8] = include_bytes!("./data/rank/B.png");

pub const RANK_C: &[u8] = include_bytes!("./data/rank/C.png");

pub const RANK_D: &[u8] = include_bytes!("./data/rank/D.png");

pub const MOD_AC: &[u8] = include_bytes!("./data/mods/AC.png");

pub const MOD_AD: &[u8] = include_bytes!("./data/mods/AD.png");

pub const MOD_AL: &[u8] = include_bytes!("./data/mods/AL.png");

pub const MOD_AP: &[u8] = include_bytes!("./data/mods/AP.png");

pub const MOD_AS: &[u8] = include_bytes!("./data/mods/AS.png");

pub const MOD_AT: &[u8] = include_bytes!("./data/mods/AT.png");

pub const MOD_BL: &[u8] = include_bytes!("./data/mods/BL.png");

pub const MOD_BR: &[u8] = include_bytes!("./data/mods/BR.png");

pub const MOD_BU: &[u8] = include_bytes!("./data/mods/BU.png");

pub const MOD_CL: &[u8] = include_bytes!("./data/mods/CL.png");

pub const MOD_CN: &[u8] = include_bytes!("./data/mods/CN.png");

pub const MOD_DA: &[u8] = include_bytes!("./data/mods/DA.png");

pub const MOD_DC: &[u8] = include_bytes!("./data/mods/DC.png");

pub const MOD_DF: &[u8] = include_bytes!("./data/mods/DF.png");

pub const MOD_DP: &[u8] = include_bytes!("./data/mods/DP.png");

pub const MOD_DT: &[u8] = include_bytes!("./data/mods/DT.png");

pub const MOD_EZ: &[u8] = include_bytes!("./data/mods/EZ.png");

pub const MOD_FL: &[u8] = include_bytes!("./data/mods/FL.png");

pub const MOD_FR: &[u8] = include_bytes!("./data/mods/FR.png");

pub const MOD_GR: &[u8] = include_bytes!("./data/mods/GR.png");

pub const MOD_HD: &[u8] = include_bytes!("./data/mods/HD.png");

pub const MOD_HR: &[u8] = include_bytes!("./data/mods/HR.png");

pub const MOD_HT: &[u8] = include_bytes!("./data/mods/HT.png");

pub const MOD_MG: &[u8] = include_bytes!("./data/mods/MG.png");

pub const MOD_MR: &[u8] = include_bytes!("./data/mods/MR.png");

pub const MOD_MU: &[u8] = include_bytes!("./data/mods/MU.png");

pub const MOD_NC: &[u8] = include_bytes!("./data/mods/NC.png");

pub const MOD_NF: &[u8] = include_bytes!("./data/mods/NF.png");

pub const MOD_NS: &[u8] = include_bytes!("./data/mods/NS.png");

pub const MOD_PF: &[u8] = include_bytes!("./data/mods/PF.png");

pub const MOD_RD: &[u8] = include_bytes!("./data/mods/RD.png");

pub const MOD_RP: &[u8] = include_bytes!("./data/mods/RP.png");

pub const MOD_RX: &[u8] = include_bytes!("./data/mods/RX.png");

pub const MOD_SD: &[u8] = include_bytes!("./data/mods/SD.png");

pub const MOD_SG: &[u8] = include_bytes!("./data/mods/SG.png");

pub const MOD_SI: &[u8] = include_bytes!("./data/mods/SI.png");

pub const MOD_SO: &[u8] = include_bytes!("./data/mods/SO.png");

pub const MOD_ST: &[u8] = include_bytes!("./data/mods/ST.png");

pub const MOD_SY: &[u8] = include_bytes!("./data/mods/SY.png");

pub const MOD_TC: &[u8] = include_bytes!("./data/mods/TC.png");

pub const MOD_TD: &[u8] = include_bytes!("./data/mods/TD.png");

pub const MOD_TP: &[u8] = include_bytes!("./data/mods/TP.png");

pub const MOD_TR: &[u8] = include_bytes!("./data/mods/TR.png");

pub const MOD_SV2: &[u8] = include_bytes!("./data/mods/V2.png");

pub const MOD_WD: &[u8] = include_bytes!("./data/mods/WD.png");

pub const MOD_WG: &[u8] = include_bytes!("./data/mods/WG.png");

pub const MOD_WU: &[u8] = include_bytes!("./data/mods/WU.png");

static MOD_TABLE: &[(&str, &[u8])] = &[
    ("AC", MOD_AC),
    ("AD", MOD_AD),
    ("AL", MOD_AL),
    ("AP", MOD_AP),
    ("AS", MOD_AS),
    ("AT", MOD_AT),
    ("BL", MOD_BL),
    ("BR", MOD_BR),
    ("BU", MOD_BU),
    ("CL", MOD_CL),
    ("CN", MOD_CN),
    ("DA", MOD_DA),
    ("DC", MOD_DC),
    ("DF", MOD_DF),
    ("DP", MOD_DP),
    ("DT", MOD_DT),
    ("EZ", MOD_EZ),
    ("FL", MOD_FL),
    ("FR", MOD_FR),
    ("GR", MOD_GR),
    ("HD", MOD_HD),
    ("HR", MOD_HR),
    ("HT", MOD_HT),
    ("MG", MOD_MG),
    ("MR", MOD_MR),
    ("MU", MOD_MU),
    ("NC", MOD_NC),
    ("NF", MOD_NF),
    ("NS", MOD_NS),
    ("PF", MOD_PF),
    ("RD", MOD_RD),
    ("RP", MOD_RP),
    ("RX", MOD_RX),
    ("SD", MOD_SD),
    ("SG", MOD_SG),
    ("SI", MOD_SI),
    ("SO", MOD_SO),
    ("ST", MOD_ST),
    ("SY", MOD_SY),
    ("TC", MOD_TC),
    ("TD", MOD_TD),
    ("TP", MOD_TP),
    ("TR", MOD_TR),
    ("SV2", MOD_SV2),
    ("WD", MOD_WD),
    ("WG", MOD_WG),
    ("WU", MOD_WU),
];

static RANK_TABLE: &[(&Grade, &[u8])] = &[
    (&Grade::S, RANK_S),
    (&Grade::SH, RANK_SILVER_S),
    (&Grade::X, RANK_SS),
    (&Grade::XH, RANK_SILVER_SS),
    (&Grade::A, RANK_A),
    (&Grade::B, RANK_B),
    (&Grade::C, RANK_C),
    (&Grade::D, RANK_D),
];

pub fn get_mod_bytes(m: &GameMod) -> &'static [u8] {
    let binding = m.acronym();
    let key = binding.as_str();
    MOD_TABLE
        .iter()
        .find(|(k, _)| *k == key)
        .map(|(_, v)| *v)
        .expect("Mod must have file")
}

pub fn get_rank_bytes(grade: &Grade) -> &'static [u8] {
    RANK_TABLE
        .iter()
        .find(|(k, _)| *k == grade)
        .map(|(_, v)| *v)
        .expect("Grade must have file")
}