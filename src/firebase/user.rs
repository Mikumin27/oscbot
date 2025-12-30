use crate::firebase::get_firebase_instance;

pub async fn get_user_skin(discord_user_id: &String) -> Option<String> {
    get_firebase_instance().at("users").at(discord_user_id).at("skin").get::<Option<String>>().await.unwrap()
}

pub async fn save_skin(discord_user_id: &String, skin: &String) {
    get_firebase_instance().at("users").at(discord_user_id).set_with_key("skin", skin).await.unwrap();
}