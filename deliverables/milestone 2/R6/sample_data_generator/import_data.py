from env import COURSE_LIMIT
from utils import (
    write_json_to_file, 
    write_string_to_file,
    write_strings_to_file
)
from courses import (
    get_raw_courses_for_term,
    map_raw_courses_to_insert_query,
)
from course_offerings import (
    get_course_offerings_for_active_raw_courses,
    map_raw_course_offerings_to_insert_query,
)
from class_schedules import (
    map_schedules_to_schedule_query
)
from env import *
import time

raw_courses = get_raw_courses_for_term(CURRENT_TERM, limit=COURSE_LIMIT)
formatted_course_query = map_raw_courses_to_insert_query(raw_courses)

# Save raw courses for current_term.
write_json_to_file(
    f"raw_courses_{CURRENT_TERM}.json",
    json_object=raw_courses,
)

# Write the insert course query for the courses in current term.
write_string_to_file(
    file_name=f"course_queries_{CURRENT_TERM}.sql",
    text=formatted_course_query,
)

start_offering = time.time()
# For raw courses that are active in this term, get schedule data for them.
course_offering_entries = get_course_offerings_for_active_raw_courses(CURRENT_TERM, raw_courses)
write_json_to_file(f"course_offerings.json", json_object=course_offering_entries)
end_offering = time.time()
elapsed_time = end_offering - start_offering
print(f"Elapsed time: {round(elapsed_time, 2)} seconds to get offerings data for: {len(raw_courses)} courses.")

# write second insert query.
offerings_query = map_raw_course_offerings_to_insert_query(course_offering_entries)
write_strings_to_file("course_offerings.sql", string_list=offerings_query)

# write schedules query.
schedules_query = map_schedules_to_schedule_query(course_offering_entries)
write_strings_to_file("class_schedule.sql", string_list=schedules_query)