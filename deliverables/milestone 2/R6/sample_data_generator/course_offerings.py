from env import *
import requests


def get_course_offerings_for_active_raw_courses(current_term: str, raw_courses: list):
    url = BASE_V3_URL + CLASS_SCHEDULES + f"/{current_term}"
    course_offering_courses = []
    for idx, course in enumerate(raw_courses, start=1):
        subject_code = course["subjectCode"]
        catalog_num = course["catalogNumber"]
        full_url = url + f"/{subject_code}/{catalog_num}"
        resp = requests.get(full_url, headers=HEADERS)

        if resp.status_code == 200:
            sched = resp.json()
            sched[0]["_COURSE_DB_ID"] = idx
            course_offering_courses.append(sched)
    return course_offering_courses


def map_raw_course_offerings_to_insert_query(course_offerings: list):
    BASE_QUERY = "INSERT INTO course_offerings (course_id, year, term)"
    BASE_QUERY += "\nVALUES"
    YEAR = 2023
    TERM = "Spring"

    query_str_list = [BASE_QUERY]

    for offering_list in course_offerings:
        first_section = offering_list[0]
        course_id = first_section["_COURSE_DB_ID"]
        # max_enrollment = first_section["maxEnrollmentCapacity"]
        # current_enrollment = first_section["enrolledStudents"]
        curr_query = f"\t({course_id}, {YEAR}, '{TERM}'),"
        query_str_list.append(curr_query)

    query_str_list[len(query_str_list) - 1] = query_str_list[len(query_str_list) - 1][:-1]
    query_str_list.append("ON CONFLICT DO NOTHING;")
    return query_str_list