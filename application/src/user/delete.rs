use diesel::prelude::*;
use domain::models::User;
use infrastructure::establish_connection;
use rocket::response::status::NotFound;
use shared::response_models::{Response, ResponseBody};

pub fn delete_user(user_id: i32) -> Result<Vec<User>, NotFound<String>> {
    use infrastructure::schema::users;
    use infrastructure::schema::users::dsl::*;

    let response: Response;

    let num_deleted =
        match diesel::delete(users.filter(id.eq(user_id))).execute(&mut establish_connection()) {
            Ok(count) => count,
            Err(err) => match err {
                diesel::result::Error::NotFound => {
                    let response = Response {
                        body: ResponseBody::Message(format!(
                            "Error deleting user with id {} - {}",
                            user_id, err
                        )),
                    };
                    return Err(NotFound(serde_json::to_string(&response).unwrap()));
                }
                _ => {
                    panic!("Database error - {}", err);
                }
            },
        };

    if num_deleted > 0 {
        match users::table
            .select(users::all_columns)
            .load::<User>(&mut establish_connection())
        {
            Ok(mut users_) => {
                users_.sort();
                Ok(users_)
            }
            Err(err) => {
                panic!("Database error - {}", err);
            }
        }
    } else {
        response = Response {
            body: ResponseBody::Message(format!("Error - no user with id {}", user_id)),
        };
        Err(NotFound(serde_json::to_string(&response).unwrap()))
    }
}
