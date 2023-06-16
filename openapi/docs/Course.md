# Course

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**course_id** | Option<**String**> | Course Id that identifies this Course, not unique across terms | [optional]
**course_offer_number** | Option<**i32**> | Course Offer Number identifies cross-listed and similar Courses that shared a Course Id in a Term | [optional]
**term_code** | Option<**String**> | Waterloo Term code for which this Course data applies | [optional]
**term_name** | Option<**String**> | Waterloo Term name for which this Course data applies | [optional]
**associated_academic_career** | Option<**String**> | Academic Career code associated with Course | [optional]
**associated_academic_group_code** | Option<**String**> | The Academic Group code that is assocaited to this Course | [optional]
**associated_academic_org_code** | Option<**String**> | The Academic Organization code that is associated to this Course | [optional]
**subject_code** | Option<**String**> | The Subject code for this Course | [optional]
**catalog_number** | Option<**String**> | The Catalog Number for this Course | [optional]
**title** | Option<**String**> | Course title, full name of course | [optional]
**description_abbreviated** | Option<**String**> | Short description of the course, often an abbreviation of the title | [optional]
**description** | Option<**String**> | Description of the Course content and topics | [optional]
**grading_basis** | Option<**String**> | Code to describe the grading basis for this course, can be overriden at Class level | [optional]
**course_component_code** | Option<**String**> | Course Component Code that describes if the course is a lecture, tutorial, etc. | [optional]
**enroll_consent_code** | Option<**String**> | Code describing whether No, Instructor, or Department consent to enroll is required. Can be overwridden at Class level. | [optional]
**enroll_consent_description** | Option<**String**> | Description of the enroll requirement. Can be overwridden at Class level. | [optional][readonly]
**drop_consent_code** | Option<**String**> | Code describing whether No, Instructor, or Department consent to drop is required. Can be overwridden at Class level. | [optional]
**drop_consent_description** | Option<**String**> | Description of the drop requirement. Can be overwridden at Class level. | [optional][readonly]
**requirements_description** | Option<**String**> | Description of the Course requirements, such as pre-requisites, anti-requisites, and co-requisites | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


