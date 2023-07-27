use openapi::apis::configuration::Configuration;
use openapi::models;
use openapi::apis;
use openapi::apis::terms_api;
use openapi::apis::courses_api;

pub async fn get_term_name_for_current_term(config: &Configuration) -> Result<String, String> {
    match get_current_term_data(config).await {
        Ok(term) => Ok(term.name.unwrap().unwrap()),
        Err(err) => Err(format!("ERROR retrieving current term: {:?}", err))
    }
}

pub async fn get_term_code_for_current_term(config: &Configuration) -> Result<String, String> {
    match get_current_term_data(config).await {
        Ok(term) => Ok(term.term_code.unwrap().unwrap()),
        Err(err) => Err(format!("ERROR retrieving current term: {:?}", err))
    }
}

pub async fn get_current_term_data(config: &Configuration) 
-> Result<models::Term, apis::Error<terms_api::V3TermsCurrentGetError>> {
    terms_api::v3_terms_current_get(config).await
}

pub async fn get_active_courses_for_term(term_code: &str, config: &Configuration) -> Result<Vec<models::Course>, String> {
    let courses: Result<Vec<models::Course>, apis::Error<courses_api::V3CoursesTermCodeGetError>> = 
        courses_api::v3_courses_term_code_get(config, term_code).await;

    match courses {
        Ok(courses) => Ok(courses),
        Err(err) => {
            let err_string: String = format!(
                "ERROR! Could not retrieve courses for term: {}. Details: {:?}", 
                term_code, 
                err
            );
            Err(err_string)
        }
    }

}