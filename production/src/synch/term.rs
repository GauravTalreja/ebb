use openapi::apis::configuration::Configuration;
use openapi::models;
use openapi::apis;
use openapi::apis::terms_api;
use openapi::apis::courses_api;
use log::error;

// #[allow(unused)]
// pub async fn get_next_term_data(config: &Configuration) 
// -> Result<models::Term, String> {
//     // The Winter X term is considered to be part of associated academic year X.
//     // But the Spring/Fall X term is considered to be part of associated academic year X + 1.

//     let curr_term_name: String = get_term_name_for_current_term(config).await;

//     if curr_term_name.contains("ERROR") {
//         Err(String::from("Could not get term data"))
//     } else {

//     }

// }

#[allow(unused)]
pub async fn get_term_name_for_current_term(config: &Configuration) -> String {
    match get_current_term_data(config).await {
        Ok(term) => term.name.unwrap().unwrap(),
        Err(err) => format!("ERROR retrieving current term: {:?}", err)
    }
}

pub async fn get_term_code_for_current_term(config: &Configuration) -> String {
    match get_current_term_data(config).await {
        Ok(term) => term.term_code.unwrap().unwrap(),
        Err(err) => format!("ERROR retrieving current term: {:?}", err)
    }
}

pub async fn get_current_term_data(config: &Configuration) 
-> Result<models::Term, apis::Error<terms_api::V3TermsCurrentGetError>> {
    terms_api::v3_terms_current_get(config).await
}

pub async fn get_active_courses_for_term(term_code: &str, config: &Configuration) -> Vec<models::Course> {
    let courses: Result<Vec<models::Course>, apis::Error<courses_api::V3CoursesTermCodeGetError>> = 
        courses_api::v3_courses_term_code_get(config, term_code).await;

    match courses {
        Ok(courses) => {
            courses
        }, 
        Err(err) => {
            error!("ERROR! Could not retrieve courses for term \"{}\". Details: {:?}", term_code, err);
            Vec::<models::Course>::new()
        }
    }

}