/*
 * Waterloo OpenData API
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: v3
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FranchiseMenu {
    #[serde(rename = "vid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vid: Option<Option<String>>,
    #[serde(rename = "uid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uid: Option<Option<String>>,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "log", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub log: Option<Option<String>>,
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<String>>,
    #[serde(rename = "comment", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub comment: Option<Option<String>>,
    #[serde(rename = "promote", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub promote: Option<Option<String>>,
    #[serde(rename = "sticky", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sticky: Option<Option<String>>,
    #[serde(rename = "vuuid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub vuuid: Option<Option<String>>,
    #[serde(rename = "nid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub nid: Option<Option<String>>,
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<String>>,
    #[serde(rename = "language", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub language: Option<Option<String>>,
    #[serde(rename = "created", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub created: Option<Option<String>>,
    #[serde(rename = "changed", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub changed: Option<Option<String>>,
    #[serde(rename = "tnid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tnid: Option<Option<String>>,
    #[serde(rename = "translate", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub translate: Option<Option<String>>,
    #[serde(rename = "uuid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<Option<String>>,
    #[serde(rename = "revision_timestamp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub revision_timestamp: Option<Option<String>>,
    #[serde(rename = "revision_uid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub revision_uid: Option<Option<String>>,
    #[serde(rename = "field_addons", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub field_addons: Option<Option<serde_json::Value>>,
    #[serde(rename = "field_combos", skip_serializing_if = "Option::is_none")]
    pub field_combos: Option<Box<crate::models::FieldCombos>>,
    #[serde(rename = "field_franchise_logo", skip_serializing_if = "Option::is_none")]
    pub field_franchise_logo: Option<Box<crate::models::FieldFranchiseLogo>>,
    #[serde(rename = "field_individual_items", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub field_individual_items: Option<Option<serde_json::Value>>,
    #[serde(rename = "title_field", skip_serializing_if = "Option::is_none")]
    pub title_field: Option<Box<crate::models::TitleField>>,
    #[serde(rename = "metatags", skip_serializing_if = "Option::is_none")]
    pub metatags: Option<Box<crate::models::Metatags>>,
    #[serde(rename = "rdf_mapping", skip_serializing_if = "Option::is_none")]
    pub rdf_mapping: Option<Box<crate::models::RdfMapping>>,
    #[serde(rename = "path", skip_serializing_if = "Option::is_none")]
    pub path: Option<Box<crate::models::Path>>,
    #[serde(rename = "translations", skip_serializing_if = "Option::is_none")]
    pub translations: Option<Box<crate::models::Translations>>,
    #[serde(rename = "title_original", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title_original: Option<Option<String>>,
    #[serde(rename = "entity_translation_handler_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub entity_translation_handler_id: Option<Option<String>>,
    #[serde(rename = "title_language", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title_language: Option<Option<String>>,
    #[serde(rename = "cid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub cid: Option<Option<String>>,
    #[serde(rename = "last_comment_timestamp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_comment_timestamp: Option<Option<String>>,
    #[serde(rename = "last_comment_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_comment_name: Option<Option<serde_json::Value>>,
    #[serde(rename = "last_comment_uid", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub last_comment_uid: Option<Option<String>>,
    #[serde(rename = "comment_count", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub comment_count: Option<Option<String>>,
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    #[serde(rename = "picture", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub picture: Option<Option<String>>,
    #[serde(rename = "data", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub data: Option<Option<String>>,
    #[serde(rename = "uw_page_settings_node_enable", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub uw_page_settings_node_enable: Option<Option<String>>,
    #[serde(rename = "workbench_moderation", skip_serializing_if = "Option::is_none")]
    pub workbench_moderation: Option<Box<crate::models::WorkbenchModeration>>,
}

impl FranchiseMenu {
    pub fn new() -> FranchiseMenu {
        FranchiseMenu {
            vid: None,
            uid: None,
            title: None,
            log: None,
            status: None,
            comment: None,
            promote: None,
            sticky: None,
            vuuid: None,
            nid: None,
            r#type: None,
            language: None,
            created: None,
            changed: None,
            tnid: None,
            translate: None,
            uuid: None,
            revision_timestamp: None,
            revision_uid: None,
            field_addons: None,
            field_combos: None,
            field_franchise_logo: None,
            field_individual_items: None,
            title_field: None,
            metatags: None,
            rdf_mapping: None,
            path: None,
            translations: None,
            title_original: None,
            entity_translation_handler_id: None,
            title_language: None,
            cid: None,
            last_comment_timestamp: None,
            last_comment_name: None,
            last_comment_uid: None,
            comment_count: None,
            name: None,
            picture: None,
            data: None,
            uw_page_settings_node_enable: None,
            workbench_moderation: None,
        }
    }
}


