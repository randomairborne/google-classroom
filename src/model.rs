use serde::{Serialize, Deserialize};

/// Possible modes of assigning coursework/announcements.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub enum AssigneeMode {
    /// No mode specified. This is never returned.
    #[serde(rename = "ASSIGNEE_MODE_UNSPECIFIED")]
    Unspecified,
    /// All students can see the item. This is the default state.
    #[serde(rename = "ALL_STUDENTS")]
    AllStudents, 
    /// A subset of the students can see the item.
    #[serde(rename = "INDIVIDUAL_STUDENTS")]
    IndividiualStudents
}

/// Assignee details about a coursework/announcement. This field is set if and only if [AssigneeMode] is INDIVIDUAL_STUDENTS.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct IndividualStudentsOptions {
    #[serde(rename = "studentIds")]
    student_ids: Vec<String>
}

/// Possible types of work
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub enum CourseWorkType {
    /// No work type specified. This is never returned.
    #[serde(rename = "COURSE_WORK_TYPE_UNSPECIFIED")]
    Unspecified,
    /// An assignment.
    #[serde(rename = "ASSIGNMENT")]
    Assignment, 
    /// A short answer question.
    #[serde(rename = "SHORT_ANSWER_QUESTION")]
    ShortAnswer,
    /// A multiple-choice question.
    #[serde(rename = "MULTIPLE_CHOICE_QUESTION")]
    MultipleChoice
}

/// Representation of a Google Drive file.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct DriveFile {
    /// Drive API resource ID.
    id: String,
    /// Title of the Drive item.
    title: String,
    /// URL that can be used to access the Drive item.
    #[serde(rename = "alternateLink")]
    link: String,
    /// URL of a thumbnail image of the Drive item.
    #[serde(rename = "thumbnailUrl")]
    thumbnail_url: String
}

/// Representation of a Google Drive folder.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct DriveFolder {
    /// Drive API resource ID.
    id: String,
    /// Title of the Drive folder.
    title: String,
    /// URL that can be used to access the Drive folder.
    #[serde(rename = "alternateLink")]
    link: String,
}

/// Google Forms item.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Form {
    /// URL of the form.
    #[serde(rename = "formUrl")]
    form_url: String,
    /// URL of the form responses document. Only set if respsonses have been recorded and only when the requesting user is an editor of the form.
    #[serde(rename = "responseUrl")]
    response_url: String,
    /// Title of the Form.
    #[serde(rename = "thumbnailUrl")]
    thumbnail_url: String,
    /// URL of a thumbnail image of the Form.
    title: String,
}

/// Details for a grade category in a course.
///
/// Coursework may have zero or one grade category, and the category may be used in computing the overall grade. See the [help center article](https://support.google.com/edu/classroom/answer/9184995) for details.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct GradeCategory {
    /// ID of the grade category.
    id: String,
    /// Name of the grade category.
    name: String,
    /// The weight of the category average as part of overall average. A weight of 12.34% is represented as 123400 (100% is 1,000,000). The last two digits should always be zero since we use two decimal precision. Only applicable when grade calculation type is WEIGHTED_CATEGORIES.
    weight: String,
    /// Default value of denominator. Only applicable when grade calculation type is TOTAL_POINTS.
    #[serde(rename = "defaultGradeDenominator")]
    default_grade_denominator: String,
}

/// URL item.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct Link {
    /// URL to link to. This must be a valid UTF-8 string containing between 1 and 2024 characters.
    url: String,
    /// Title of the target of the URL.
    title: String,
    /// URL of a thumbnail image of the target URL.
    #[serde(rename = "thumbnailUrl")]
    thumbnail_url: String
}

/// Contains fields to add or remove students from a course work or announcement where the [AssigneeMode] is set to [AssigneeMode::IndividualStudents]
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct ModifyIndividualStudentsOptions {
     /// IDs of students to be added as having access to this coursework/announcement.
     #[serde(rename = "addStudentIds")]
    add_student_ids: Vec<String>,
     /// IDs of students to be removed from having access to this coursework/announcement.
     #[serde(rename = "removeStudentIds")]
    remove_student_ids: Vec<String>,
}

/// YouTube video item.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct YouTubeVideo {
    /// YouTube API resource ID.
    id: String,
    /// Title of the YouTube video.
    title: String,
    /// URL that can be used to view the YouTube video.
    #[serde(rename = "alternateLink")]
    link: String,
    /// URL of a thumbnail image of the YouTube video.
    #[serde(rename = "thumbnailUrl")]
    thumbnail_url: String,
}

/// Material attached to course work.
/// 
/// When creating attachments, setting the form field is not supported.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum Material {
    #[serde(rename = "driveFile")]
    DriveFile(DriveFile),
    #[serde(rename = "youtubeVideo")]
    YouTubeVideo(YouTubeVideo),
    #[serde(rename = "link")]
    Link(Link),
    #[serde(rename = "form")]
    Form(Form)
}

/// Drive file that is used as material for course work.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub struct SharedDriveFile {
    #[serde(rename = "driveFile")]
    drive_file: DriveFile,
    #[serde(rename = "shareMode")]
    share_mode: DriveFileShareMode,
}

/// Possible sharing options. Defaults to VIEW if left unspecified, and other values may only be specified within a course work object of type ASSIGNMENT.
#[derive(Serialize, Deserialize, Debug, Hash, PartialEq, Eq)]
pub enum DriveFileShareMode {
    /// No sharing mode specified. This should never be returned.
    #[serde(rename = "UNKNOWN_SHARE_MODE")]
    Unknown,
    /// Students can view the shared file.
    #[serde(rename = "VIEW")]
    View,
    /// Students can edit the shared file.
    #[serde(rename = "EDIT")]
    Edit,
    /// Students have a personal copy of the shared file.
    #[serde(rename = "STUDENT_COPY")]
    Copy
}