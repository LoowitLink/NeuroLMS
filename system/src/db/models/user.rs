use bson::Bson;
use serde::{Serialize, Deserialize, Deserializer};
use mongodb::bson::{self, doc, oid::ObjectId, Document};
use mongodb::Collection;

use serde::de::{self, Visitor};
use std::fmt;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,  // MongoDB automatically assigns an ObjectId if not provided

    pub name: String,
    pub pref_name: String,
    pub email: String,
    pub password: String,
    pub avatar: Option<String>,
    pub permissions: Vec<UserPermission>,
    pub role: UserRole,  // Enum to define role (Admin, Student, Instructor)
    pub enrolled_courses: Vec<String>,  // List of course IDs the user is enrolled in
    pub status: UserStatus,
    pub last_login: Option<i64>,  // Timestamp for the last time the user logged in
    pub created_at: Option<i64>,  // Timestamp for when the user was created
    pub updated_at: Option<i64>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")] // Ensures the enum variants are serialized as snake_case strings
pub enum UserRole {
    Admin,
    Student,
    Instructor,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")] // Ensures the enum variants are serialized as snake_case strings
pub enum UserStatus {
    Active,
    Inactive,
    Suspended,
    Pending,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "snake_case")] // Serialize variants in snake_case
pub enum UserPermission {
    ViewCourses,
    EditCourses,
    ManageCourseUsers,
    ManageCourseMaterials,
    ManageCourseAnnouncements,
    ManageCourseGrades,
    ManageCourseDiscussions,
    ManageCourseAssignments,
    ManageUsers,
    ManageTeachers,
    ManageCourses,
    ViewAllGrades,
    AdministratorAccess,
}

impl<'de> Deserialize<'de> for UserPermission {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct PermissionVisitor;

        impl<'de> Visitor<'de> for PermissionVisitor {
            type Value = UserPermission;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid UserPermission")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value.to_lowercase().as_str() {
                    "view_courses" => Ok(UserPermission::ViewCourses),
                    "edit_courses" => Ok(UserPermission::EditCourses),
                    "manage_course_users" => Ok(UserPermission::ManageCourseUsers),
                    "manage_course_materials" => Ok(UserPermission::ManageCourseMaterials),
                    "manage_course_announcements" => Ok(UserPermission::ManageCourseAnnouncements),
                    "manage_course_grades" => Ok(UserPermission::ManageCourseGrades),
                    "manage_course_discussions" => Ok(UserPermission::ManageCourseDiscussions),
                    "manage_course_assignments" => Ok(UserPermission::ManageCourseAssignments),
                    "manage_users" => Ok(UserPermission::ManageUsers),
                    "manage_teachers" => Ok(UserPermission::ManageTeachers),
                    "manage_courses" => Ok(UserPermission::ManageCourses),
                    "view_all_grades" => Ok(UserPermission::ViewAllGrades),
                    "administrator_access" | "administratoraccess" => Ok(UserPermission::AdministratorAccess), // Handle both cases
                    _ => Err(de::Error::unknown_variant(value, &["view_courses", "edit_courses", "admin_access", "administratoraccess"])),
                }
            }
        }

        deserializer.deserialize_str(PermissionVisitor)
    }
}


impl From<UserPermission> for Bson {
    fn from(permission: UserPermission) -> Self {
        Bson::String(format!("{:?}", permission).to_lowercase())  // Convert to lowercase string
    }
}

impl From<UserRole> for Bson {
    fn from(role: UserRole) -> Self {
        Bson::String(format!("{:?}", role).to_lowercase())  // Convert to lowercase string
    }
}

impl From<UserStatus> for Bson {
    fn from(status: UserStatus) -> Self {
        Bson::String(format!("{:?}", status).to_lowercase())  // Convert to lowercase string
    }
}

impl User {
    pub fn from_document(doc: Document) -> Result<Self, mongodb::error::Error> {
        bson::from_document(doc).map_err(Into::into)
    }
}