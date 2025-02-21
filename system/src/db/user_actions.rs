use bson::doc;
use mongodb::{bson::Document, error::Error};
use crate::db::connector::MongoRepo;

use super::models::user::User;

impl MongoRepo {
    pub fn count_user_documents(&self, filter: Document) -> Result<i64, Error> {
        let count_result = self.col_users.count_documents(filter, None)?;
        Ok(count_result as i64)
    }

    pub fn create_user(&self, new_user: User) -> Result<User, Error> {
        let new_doc = doc!{
            "name": new_user.name,
            "pref_name": new_user.pref_name,
            "email": new_user.email,
            "password": new_user.password,
            "avatar": new_user.avatar,
            "permissions": new_user.permissions,  // This will automatically convert `UserPermission` enum to Bson
            "role": new_user.role,  // `UserRole` should also be serialized properly as a string
            "enrolled_courses": new_user.enrolled_courses,
            "status": new_user.status, // `UserStatus` should be serialized as a string
            "last_login": new_user.last_login,
            "created_at": new_user.created_at,
            "updated_at": new_user.updated_at
        };
    
        let user = self.col_users.insert_one(new_doc, None)?;
        let new_filter = doc!{"_id": user.inserted_id};
        let user_data = self.col_users.find_one(new_filter, None)?.unwrap();
        Ok(User::from_document(user_data)?)
    }

    pub fn get_user_information(&self, filter: Document) -> Result<User, Error> {
        let user_data = self.col_users.find_one(filter, None)?.unwrap();
        Ok(User::from_document(user_data)?)
    }
    
    
}
