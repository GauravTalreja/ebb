from utils import write_strings_to_file

CID = 1946
UP_TO = 2045
YEAR = 2023
TERM = 'spring'
SDATE = '2023-05-08T00:00:00'
EDATE = '2023-08-01T00:00:00'

import random

NAMES = ['Alice', 'Bob', 'Charlie', 'David', 'Eve', 'Frank', 'Grace', 'Henry', 'Ivy', 'Jack', 'Katherine', 'Liam', 'Mia', 'Nathan', 'Olivia', 'Peter', 'Quinn', 'Rachel', 'Samuel', 'Taylor']
terms = ['winter', 'spring', 'fall']
tuple_val = (id, 2023, 'spring', [], 200, 0)

first_string = "INSERT INTO course_offerings (course_id, year, term, start_date, end_date, instructor_names, max_enrollment, current_enrollment)"
second_string = "VALUES"

strings_list = [first_string, second_string]

while CID <= UP_TO:
    name_list = random.choices(NAMES, k=random.randint(1, 3))
    max_enrollment = random.randint(100, 500)
    curr_enrollment = random.randint(0, max_enrollment)
    query_string = f"({CID}, {YEAR}, '{TERM}', '{SDATE}', '{EDATE}', ARRAY{name_list}, {max_enrollment}, {curr_enrollment}),"
    strings_list.append(query_string)
    CID += 1

strings_list.append("ON CONFLICT DO NOTHING;")

write_strings_to_file("insert_queries.txt", strings_list)