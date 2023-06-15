# Class

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**course_id** | Option<**String**> | Course identifier number, not unique | [optional]
**course_offer_number** | Option<**i32**> | Course offer number identifier for this class | [optional]
**session_code** | Option<**String**> | Session code for this class | [optional]
**class_section** | Option<**i32**> | Number identifying the section of this class | [optional]
**term_code** | Option<**String**> | Waterloo academic term code | [optional]
**class_number** | Option<**i32**> | Class number identifier for this class | [optional]
**course_component** | Option<**String**> | Course component code for this class | [optional]
**associated_class_code** | Option<**i32**> | Associated class code for this class | [optional]
**max_enrollment_capacity** | Option<**i32**> | Indicates the maximum number of students that can enroll in this class | [optional]
**enrolled_students** | Option<**i32**> | Indicates the current number of students enrolled in this class | [optional]
**enroll_consent_code** | Option<**String**> | Code describing whether No, Instructor, or Department consent to enroll is required. Overrides Course level information if different. | [optional]
**enroll_consent_description** | Option<**String**> | Description of the enroll requirement. Overrides Course level information if different. | [optional][readonly]
**drop_consent_code** | Option<**String**> | Code describing whether No, Instructor, or Department consent to drop is required. Overrides Course level information if different. | [optional]
**drop_consent_description** | Option<**String**> | Description of the drop requirement. Overrides Course level information if different. | [optional][readonly]
**schedule_data** | Option<[**Vec<crate::models::ClassSchedule>**](ClassSchedule.md)> | Schedule data for this class | [optional]
**instructor_data** | Option<[**Vec<crate::models::ClassInstructor>**](ClassInstructor.md)> | Instructor data for this class | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


