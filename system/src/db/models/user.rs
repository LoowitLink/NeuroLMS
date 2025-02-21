use serde::{Serialize, Deserialize};
use mongodb::bson::{self, doc, oid::ObjectId, Document};
use mongodb::Collection;

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
    pub enrolled_courses: Vec<ObjectId>,  // List of course IDs the user is enrolled in
    pub status: UserStatus,
    pub last_login: Option<i64>,  // Timestamp for the last time the user logged in
    pub created_at: Option<i64>,  // Timestamp for when the user was created
    pub updated_at: Option<i64>
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserRole {
    #[serde(rename = "Admin")]
    Admin,
    #[serde(rename = "Student")]
    Student,
    #[serde(rename = "Instructor")]
    Instructor,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserStatus {
    #[serde(rename = "Active")]
    Active,
    #[serde(rename = "Inactive")]
    Inactive,
    #[serde(rename = "Suspended")]
    Suspended,
    #[serde(rename = "Pending")]
    Pending,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserPermission {
    #[serde(rename = "ViewCourses")]
    ViewCourses,
    #[serde(rename = "EditCourses")]
    EditCourses,
    #[serde(rename = "ManageCourseUsers")]
    ManageCourseUsers,
    #[serde(rename = "ManageCourseMaterials")]
    ManageCourseMaterials,
    #[serde(rename = "ManageCourseAnnouncements")]
    ManageCourseAnnouncements,
    #[serde(rename = "ManageCourseGrades")]
    ManageCourseGrades,
    #[serde(rename = "ManageCourseDiscussions")]
    ManageCourseDiscussions,
    #[serde(rename = "ManageCourseAssignments")]
    ManageCourseAssignments,
    #[serde(rename = "ManageUsers")]
    ManageUsers,
    #[serde(rename = "ManageTeachers")]
    ManageTeachers,
    #[serde(rename = "ManageCourses")]
    ManageCourses,
    #[serde(rename = "ViewAllGrades")]
    ViewAllGrades,
    #[serde(rename = "AdminAccess")]
    AdminAccess,
    // Add other permissions as needed
}

impl User {
    pub fn from_document(doc: Document) -> Result<Self, mongodb::error::Error> {
        bson::from_document(doc).map_err(Into::into)
    }
}