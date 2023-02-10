use serde::{de::Visitor, Deserialize, Serialize};

use super::{DriveFolder, GradeCategory};

pub mod aliases;
pub mod announcements;
pub mod course_work;
pub mod students;
pub mod teachers;
pub mod topics;

/// Modify a Classroom course.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub struct CourseCreate {
    /// When creating a course, you may optionally set this identifier to an alias string in the request to create a corresponding alias. The id is still assigned by Classroom and cannot be updated after the course is created.
    pub id: Option<String>,
    /// Name of the course. For example, "10th Grade Biology". The name is required. It must be between 1 and 750 characters and a valid UTF-8 string.
    pub name: String,
    /// Section of the course. For example, "Period 2". If set, this field must be a valid UTF-8 string and no longer than 2800 characters.
    pub section: Option<String>,
    /// Optional heading for the description. For example, "Welcome to 10th Grade Biology." If set, this field must be a valid UTF-8 string and no longer than 3600 characters.
    pub description_heading: Option<String>,
    /// Optional description. For example, "We'll be learning about the structure of living creatures from a combination of textbooks, guest lectures, and lab work. Expect to be excited!" If set, this field must be a valid UTF-8 string and no longer than 30,000 characters.
    pub description: Option<String>,
    /// Optional room location. For example, "301". If set, this field must be a valid UTF-8 string and no longer than 650 characters.
    pub room: Option<String>,
    /// The identifier of the owner of a course.
    pub owner_id: OwnerId,
    /// State of the course. If unspecified, the default state is [`CourseState::Provisioned`].
    pub course_state: Option<CourseState>,
}

/// Modify a Classroom course.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
#[allow(clippy::module_name_repetitions)]
pub struct CourseModify {
    /// Name of the course. For example, "10th Grade Biology". The name is required. It must be between 1 and 750 characters and a valid UTF-8 string.
    pub name: Option<String>,
    /// Section of the course. For example, "Period 2". If set, this field must be a valid UTF-8 string and no longer than 2800 characters.
    pub section: Option<String>,
    /// Optional heading for the description. For example, "Welcome to 10th Grade Biology." If set, this field must be a valid UTF-8 string and no longer than 3600 characters.
    pub description_heading: Option<String>,
    /// Optional description. For example, "We'll be learning about the structure of living creatures from a combination of textbooks, guest lectures, and lab work. Expect to be excited!" If set, this field must be a valid UTF-8 string and no longer than 30,000 characters.
    pub description: Option<String>,
    /// Optional room location. For example, "301". If set, this field must be a valid UTF-8 string and no longer than 650 characters.
    pub room: Option<String>,
    /// The identifier of the owner of a course.
    pub owner_id: Option<OwnerId>,
    /// State of the course. If unspecified, the default state is [`CourseState::Provisioned`].
    pub course_state: Option<CourseState>,
}

/// A Course in Classroom.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Course {
    /// Identifier for this course assigned by Classroom.
    pub id: String,
    /// Name of the course. For example, "10th Grade Biology". The name is required. It must be between 1 and 750 characters and a valid UTF-8 string.
    pub name: String,
    /// Section of the course. For example, "Period 2". If set, this field must be a valid UTF-8 string and no longer than 2800 characters.
    pub section: Option<String>,
    /// Optional heading for the description. For example, "Welcome to 10th Grade Biology." If set, this field must be a valid UTF-8 string and no longer than 3600 characters.
    pub description_heading: Option<String>,
    /// Optional description. For example, "We'll be learning about the structure of living creatures from a combination of textbooks, guest lectures, and lab work. Expect to be excited!" If set, this field must be a valid UTF-8 string and no longer than 30,000 characters.
    pub description: Option<String>,
    /// Optional room location. For example, "301". If set, this field must be a valid UTF-8 string and no longer than 650 characters.
    pub room: Option<String>,
    /// Identifier for owner of a course. Can be a google identifier, an email, or `me`
    pub owner_id: OwnerId,
    /// When this course was created
    #[cfg(feature = "chrono")]
    pub creation_time: chrono::DateTime<chrono::Utc>,
    /// Time of the most recent update to this course. Specifying this field in a course update mask results in an error.
    #[cfg(feature = "chrono")]
    pub update_time: chrono::DateTime<chrono::Utc>,
    /// Enrollment code to use when joining this course. Specifying this field in a course update mask results in an error.
    pub enrollment_code: String,
    /// State of the course. If unspecified, the default state is [`CourseState::Provisioned`].
    pub course_state: Option<CourseState>,
    /// Absolute link to this course in the Classroom web UI.
    pub alternate_link: String,
    /// The email address of a Google group containing all teachers of the course. This group does not accept email and can only be used for permissions.
    pub teacher_group_email: String,
    /// The email address of a Google group containing all members of the course. This group does not accept email and can only be used for permissions.
    pub course_group_email: String,
    /// Information about a Drive Folder that is shared with all teachers of the course.
    /// This field will only be set for teachers of the course and domain administrators.
    pub teacher_folder: Option<DriveFolder>,
    /// Whether or not guardian notifications are enabled for this course.
    pub guardians_enabled: bool,
    /// The Calendar ID for a calendar that all course members can see, to which Classroom adds events for course work and announcements in the course.
    pub calendar_id: String,
    /// The gradebook settings that specify how a student's overall grade for the course will be calculated and who it will be displayed to.
    pub gradebook_settings: GradebookSettings,
}

/// Possible states a course can be in.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::module_name_repetitions, clippy::enum_variant_names)]
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
    grade_categories: Vec<GradeCategory>,
}

/// Possible methods of overall grade calculation.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::enum_variant_names)]
pub enum CalculationType {
    CalculationTypeUnspecified,
    TotalPoints,
    WeightedCategories,
}

/// Possible methods of overall grade calculation.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::enum_variant_names)]
pub enum DisplaySetting {
    DisplaySettingUnspecified,
    ShowOverallGrade,
    HideOverallGrade,
    ShowTeachersOnly,
}

/// This is a simple wrapper type, no guarentees are made.
#[derive(Debug, Hash, PartialEq, Eq)]
pub enum OwnerId {
    /// Email (unchecked)
    Email(String),
    /// Unique numeric ID (numeric only checked in deserialization)
    Id(String),
    /// Specifies the current user
    Me,
}

impl Serialize for OwnerId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            Self::Email(email) => email,
            Self::Id(id) => id,
            Self::Me => "me",
        };
        serializer.serialize_str(value)
    }
}

struct OwnerIdVisitor;

impl<'v> Visitor<'v> for OwnerIdVisitor {
    type Value = OwnerId;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Expected `str`")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == "me" {
            return Ok(OwnerId::Me);
        };
        if v.chars().all(|c| c.is_ascii_digit()) {
            return Ok(OwnerId::Id(v));
        }
        Ok(OwnerId::Email(v))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v == "me" {
            return Ok(OwnerId::Me);
        };
        if v.chars().all(|c| c.is_ascii_digit()) {
            return Ok(OwnerId::Id(v.to_string()));
        }
        Ok(OwnerId::Email(v.to_string()))
    }
}

impl<'de> Deserialize<'de> for OwnerId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_string(OwnerIdVisitor)
    }
}
