use serde::{Deserialize, Serialize};

use super::{GradeCategory, DriveFolder};

/// A Course in Classroom.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CourseCreate {
    /// Identifier for this course assigned by Classroom.
    /// When creating a course, you may optionally set this identifier to an alias string in the request to create a corresponding alias. The id is still assigned by Classroom and cannot be updated after the course is created.
    id: Option<String>,
    name: String,
    section: String,
    description_heading: String,
    description: String,
    room: String,
    owner_id: String,
    creation_time: String,
    update_time: String,
    enrollment_code: String,
    course_state: CourseState,
    alternate_link: String,
    teacher_group_email: String,
    course_group_email: String,
    teacher_folder: DriveFolder,
    guardians_enabled: bool,
    calendar_id: String,
    gradebook_settings: GradebookSettings,
}

/// A Course in Classroom.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CourseModify {
    name: String,
    section: String,
    description_heading: String,
    description: String,
    room: String,
    owner_id: String,
    creation_time: String,
    update_time: String,
    enrollment_code: String,
    course_state: CourseState,
    alternate_link: String,
    teacher_group_email: String,
    course_group_email: String,
    teacher_folder: DriveFolder,
    guardians_enabled: bool,
    calendar_id: String,
    gradebook_settings: GradebookSettings,
}

/// A Course in Classroom.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Course {
    /// Identifier for this course assigned by Classroom.
    /// When creating a course, you may optionally set this identifier to an alias string in the request to create a corresponding alias. The id is still assigned by Classroom and cannot be updated after the course is created.
    id: Option<String>,
    name: String,
    section: String,
    description_heading: String,
    description: String,
    room: String,
    owner_id: String,
    creation_time: String,
    update_time: String,
    enrollment_code: String,
    course_state: CourseState,
    alternate_link: String,
    teacher_group_email: String,
    course_group_email: String,
    teacher_folder: DriveFolder,
    guardians_enabled: bool,
    calendar_id: String,
    gradebook_settings: GradebookSettings,
}

/// Possible states a course can be in.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CourseState {
    CourseStateUnspecified,
    Active,
    Archived,
    Provisioned,
    Declined,
    Suspended,
}

/// The gradebook settings for a course. See the [help center article](https://support.google.com/edu/classroom/answer/9184995) for details.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GradebookSettings {
    calculation_type: CalculationType,
    display_setting: DisplaySetting,
    grade_categories: Vec<GradeCategory>
}

/// Possible methods of overall grade calculation.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CalculationType {
    CalculationTypeUnspecified,
    TotalPoints,
    WeightedCategories
}

/// Possible methods of overall grade calculation.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DisplaySetting {
    DisplaySettingUnspecified,
    ShowOverallGrade,
    HideOverallGrade,
    ShowTeachersOnly
}

