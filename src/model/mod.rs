use serde::{Deserialize, Serialize};
pub mod courses;
pub mod user_profiles;
pub mod invitations;
pub mod registrations;

/// Possible modes of assigning coursework/announcements.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, PartialOrd)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::enum_variant_names)]
pub enum AssigneeMode {
    /// No mode specified. This is never returned.
    AssigneeModeUnspecified,
    /// All students can see the item. This is the default state.
    AllStudents,
    /// A subset of the students can see the item.
    IndividiualStudents,
}

/// Assignee details about a coursework/announcement. This field is set if and only if [``AssigneeMode``] is [``AssigneeMode::IndividiualStudents``].
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, PartialOrd)]
#[serde(rename_all = "camelCase")]
pub struct IndividualStudentsOptions {
    pub student_ids: Vec<String>,
}

/// Possible types of work
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, PartialOrd)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(clippy::enum_variant_names)]
pub enum CourseWorkType {
    /// No work type specified. This is never returned.
    CourseWorkTypeUnspecified,
    /// An assignment.
    Assignment,
    /// A short answer question.
    ShortAnswerQuestion,
    /// A multiple-choice question.
    MultipleChoiceQuestion,
}

/// Representation of a Google Drive file.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DriveFile {
    /// Drive API resource ID.
    pub id: String,
    /// Title of the Drive item.
    pub title: String,
    /// URL that can be used to access the Drive item.
    pub alternate_link: String,
    /// URL of a thumbnail image of the Drive item.
    pub thumbnail_url: String,
}

/// Representation of a Google Drive folder.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DriveFolder {
    /// Drive API resource ID.
    pub id: String,
    /// Title of the Drive folder.
    pub title: String,
    /// URL that can be used to access the Drive folder.
    pub alternate_link: String,
}

/// Google Forms item.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Form {
    /// URL of the form.
    pub form_url: String,
    /// URL of the form responses document. Only set if respsonses have been recorded and only when the requesting user is an editor of the form.
    pub response_url: String,
    /// Title of the Form.
    pub thumbnail_url: String,
    /// URL of a thumbnail image of the Form.
    pub title: String,
}

/// Details for a grade category in a course.
///
/// Coursework may have zero or one grade category, and the category may be used in computing the overall grade. See the [help center article](https://support.google.com/edu/classroom/answer/9184995) for details.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GradeCategory {
    /// ID of the grade category.
    pub id: String,
    /// Name of the grade category.
    pub name: String,
    /// The weight of the category average as part of overall average. A weight of 12.34% is represented as 123400 (100% is 1,000,000). The last two digits should always be zero since we use two decimal precision. Only applicable when grade calculation type is WEIGHTED_CATEGORIES.
    pub weight: String,
    /// Default value of denominator. Only applicable when grade calculation type is TOTAL_POINTS.
    pub default_grade_denominator: String,
}

/// URL item.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    /// URL to link to. This must be a valid UTF-8 string containing between 1 and 2024 characters.
    pub url: String,
    /// Title of the target of the URL.
    pub title: String,
    /// URL of a thumbnail image of the target URL.
    pub thumbnail_url: String,
}

/// Contains fields to add or remove students from a course work or announcement where the [``AssigneeMode``] is set to [``AssigneeMode::IndividiualStudents``]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ModifyIndividualStudentsOptions {
    /// IDs of students to be added as having access to this coursework/announcement.
    pub add_student_ids: Vec<String>,
    /// IDs of students to be removed from having access to this coursework/announcement.
    pub remove_student_ids: Vec<String>,
}

/// ``YouTube`` video item.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct YouTubeVideo {
    /// YouTube API resource ID.
    pub id: String,
    /// Title of the YouTube video.
    pub title: String,
    /// URL that can be used to view the YouTube video.
    pub alternate_link: String,
    /// URL of a thumbnail image of the YouTube video.
    pub thumbnail_url: String,
}

/// Material attached to course work.
///
/// When creating attachments, setting the form field is not supported.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Material {
    DriveFile(DriveFile),
    YoutubeVideo(YouTubeVideo),
    Link(Link),
    Form(Form),
}

/// Drive file that is used as material for course work.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SharedDriveFile {
    /// Inner drive file
    pub drive_file: DriveFile,
    /// Mechanism by which students access the Drive item.
    pub share_mode: DriveFileShareMode,
}

/// Possible sharing options. Defaults to VIEW if left unspecified, and other values may only be specified within a course work object of type ASSIGNMENT.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DriveFileShareMode {
    /// No sharing mode specified. This should never be returned.
    UnknownShareMode,
    /// Students can view the shared file.
    View,
    /// Students can edit the shared file.
    Edit,
    /// Students have a personal copy of the shared file.
    StudentCopy,
}
