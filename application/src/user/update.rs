
use domain::models::User;
use shared::response_models::{Response, ResponseBody};
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use diesel::prelude::*;

pub fn update_user(user_id: i32) -> Result<User, NotFound<String>> {
    use infrastructure::schema::users::dsl::*;

    match diesel::update(users.find(user_id)).set(is_registered.eq(true)).get_result::<User>(&mut establish_connection()) {
        Ok(user) => Ok(user),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error registering user with id {} - {}", user_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }        
        }
    }
}