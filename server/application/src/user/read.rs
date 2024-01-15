use diesel::prelude::*;
use domain::models::User;
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn list_user(user_id: i32) -> Result<User, NotFound<String>> {
    use infrastructure::schema::users;

    match users::table
        .find(user_id)
        .first::<User>(&mut establish_connection())
    {
        Ok(user) => Ok(user),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response {
                    body: ResponseBody::Message(format!(
                        "Error selecting user with id {} - {}",
                        user_id, err
                    )),
                };
                Err(NotFound(serde_json::to_string(&response).unwrap()))
            }
            _ => {
                panic!("Database error - {}", err);
            }
        },
    }
}

pub fn list_users() -> Vec<User> {
    use infrastructure::schema::users;

    match users::table
        .select(users::all_columns)
        .load::<User>(&mut establish_connection())
    {
        Ok(mut users) => {
            users.sort();
            users
        }
        Err(err) => {
            panic!("Database error - {}", err);
        }
    }
}
