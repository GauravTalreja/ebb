from env import *
import requests
import random

def get_raw_courses_for_term(term_code: str, limit: int):
    full_url = BASE_V3_URL + COURSES + f"/{term_code}"
    resp = requests.get(full_url, headers=HEADERS)

    if resp.status_code == 200:
        l = resp.json()
        # random.seed(SEED)
        # random.shuffle(l)
        print(len(l))
        return l[:limit]
    else:
        print("ERROR:", resp.content)
        return []


def map_raw_courses_to_insert_query(raw_courses: list):
    BASE_QUERY_STR = "INSERT INTO courses (catalog_number, subject_code, external_id," \
        " academic_level, title, description, requirements, enroll_consent, drop_consent, prerequisites_id)\nVALUES\n"
    END_QUERY_STR = "\nON CONFLICT DO NOTHING;"

    for course in raw_courses:
        # Escape single quotes (') for SQL query ('').
        catalog_number = course["catalogNumber"]
        subject_code = course["subjectCode"].replace("'", "''")
        external_id = course["courseId"].replace("'", "''")
        academic_level = course["associatedAcademicCareer"]
        title = course["title"].replace("'", "''")
        description = course["description"].replace("'", "''")
        requirements = None
        enroll_consent = None
        drop_consent = course["dropConsentDescription"].replace("'", "''") if course["dropConsentDescription"] else course["dropConsentDescription"]
        
        # Handle cases for requirements description.
        if course["requirementsDescription"] is not None:
            v = course["requirementsDescription"].replace("'", "''")
            requirements = f"'{v}'"
        else:
            requirements = "NULL"

        # Handle cases for enrollConsentDescription.
        if course["enrollConsentDescription"] is not None:
            v = course["enrollConsentDescription"].replace("'", "''")
            enroll_consent = f"'{v}'"
        else:
            enroll_consent = "NULL"

        # Handle cases for dropConsentDescription.
        if course["dropConsentDescription"] is not None:
            v = course["dropConsentDescription"].replace("'", "''")
            drop_consent = f"'{v}'"
        else:
            drop_consent = "NULL"
        
        prerequisites_id = "NULL"

        # Construct insert query for a course.
        BASE_QUERY_STR += f"\t('{catalog_number}', '{subject_code}'" \
        f", '{external_id}', '{academic_level}', '{title}', '{description}'" \
        f", {requirements}, {enroll_consent}, {drop_consent}, {prerequisites_id}),\n"

    return BASE_QUERY_STR[:-2] + END_QUERY_STR