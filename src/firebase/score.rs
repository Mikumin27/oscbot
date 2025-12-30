use crate::firebase::get_firebase_instance;

pub async fn score_already_saved(identifier: &String) -> bool {
    match get_firebase_instance().at("checked_scores").at(identifier).get::<Option<bool>>().await {
        Ok(Some(true)) => true,
        Err(_) => false,
        _ => false,
    }
}

pub async fn insert_score(identifier: &String) {
    get_firebase_instance().at("checked_scores").set_with_key(identifier, &true).await.unwrap();
}