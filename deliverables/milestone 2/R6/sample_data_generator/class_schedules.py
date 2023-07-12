def map_schedules_to_schedule_query(course_offerings: list):
    BASE_QUERY = "INSERT INTO class_schedule (class_section, class_number, component, start_time"\
        ", end_time, monday, tuesday, wednesday, thursday, friday, saturday, sunday" \
        ", instructor_name, location, course_offering_id, max_enrollment, current_enrollment)"
    BASE_QUERY += "\nVALUES"

    query_str_list = [BASE_QUERY]

    for course_offering_id, offering_list in enumerate(course_offerings, start=1):
        for offering in offering_list:
            schedule = offering["scheduleData"][0]
            instructorData = offering["instructorData"]
            class_section = offering["classSection"] # int
            class_number = offering["classNumber"] # int
            component = offering["courseComponent"] # string
            start_time = schedule["classMeetingStartTime"].split("T")[1] # timestamp as string
            end_time = schedule["classMeetingEndTime"].split("T")[1] # timestamp as string
            days_code = schedule["classMeetingWeekPatternCode"]
            location = None # string

            if schedule["locationName"] is not None:
                location = schedule["locationName"].replace("'", "''")
                location = f"'{location}'"
            else:
                location = "NULL"

            max_enrollment = offering["maxEnrollmentCapacity"] # int
            current_enrollment = offering["enrolledStudents"] # int
            monday = "false" # "bool"
            tuesday = "false" # "bool"
            wednesday = "false" # "bool"
            thursday = "false" # "bool"
            friday = "false" # "bool"
            saturday = "false" # "bool"
            sunday = "false" # "bool"
            instructor_name = None # string

            if days_code != "":
                monday = "true" if days_code[0] == "Y" else "false"
                tuesday = "true" if days_code[1] == "Y" else "false"
                wednesday = "true" if days_code[2] == "Y" else "false"
                thursday = "true" if days_code[3] == "Y" else "false"
                friday = "true" if days_code[4] == "Y" else "false"
                saturday = "true" if days_code[5] == "Y" else "false"
                sunday = "true" if days_code[6] == "Y" else "false"

            if instructorData is None:
                instructor_name = "NULL"
            else:
                name_list = []
                for instructor in instructorData:
                    first_name = instructor["instructorFirstName"]
                    last_name = instructor["instructorLastName"]
                    name_list.append(first_name + " " + last_name)
                name_list = set(name_list)
                instructor_name = ", ".join(name_list).replace("'", "''")
                instructor_name = f"'{instructor_name}'"
            
            curr_query = f"\t({class_section}, {class_number}, '{component}', '{start_time}'" \
                f", '{end_time}', {monday}, {tuesday}, {wednesday}, {thursday}, {friday}" \
                f", {saturday}, {sunday}, {instructor_name}, {location}, {course_offering_id}" \
                f", {max_enrollment}, {current_enrollment}),"
            
            query_str_list.append(curr_query)

    query_str_list[len(query_str_list) - 1] = query_str_list[len(query_str_list) - 1][:-1]
    query_str_list.append("ON CONFLICT DO NOTHING;")
    return query_str_list