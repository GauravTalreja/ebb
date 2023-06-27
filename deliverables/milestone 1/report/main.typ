#import "template.typ": *

// Take a look at the file `template.typ` in the file panel
// to customize this template and discover how it works.
#show: project.with(
  title: "CS 348 - Milestone #1",
  authors: (
    "GT",
    "Sally",
    "Saumya",
    "Shake",
    "Justin",
  ),
  date: "May 16, 2023",
  repo: "https://github.com/GauravTalreja/ebb",
  website: "https://ebb.csclub.cloud"
)

// We generated the example code below so you can see how
// your document will look. Go ahead and replace it with
// your own content!

= Application Description
The vision is to build the ultimate search tool for courses offered by the University of Waterloo that combines #link("https://uwflow.com/")[UWFlow] and Quest's course search with a clean user interface. The primary improvement beyond the combination of Flow and Quest are *smart queries* that are built, but not directly constructed, from user input. For example, users could see all ENGL courses they can take with satisfied prerequisites alongside CS 348.

There is a multi-step plan to achieve the vision for smart queries.
+ Automate the data creation process from the #link("https://uwaterloo.ca/api/")[UWaterloo Open Data API]  and construct a database of courses for at least the current term and the next term.
+ Replicate the course search functionality of Quest and UWFlow (search by name, subject, faculty, time, location, instructor, class size, prerequisites etc.) with a clean user interface.
+ Implement the algorithms necessary for building smart queries.
+ Integrate a language learning model that builds smart queries from natural language.

Some other features that can be implemented anytime after step 2 are:
- User accounts with niceties such as prerequisites met by a user parsed from a transcript and stored.
- Integrate the data set for reviews and ratings from Flow.

This is a starry-eyed vision of the app. It is possible to satisfy the requirements of this project with just steps 1 and 2. 

The target audience is University of Waterloo students. The administrators of the DBMS would be the team maintaining and deploying the app (currently, this CS 348 group.)

= System Support
The application is a webapp written entirely in #link("https://www.rust-lang.org/")[Rust]. 

The frontend web development framework is #link("https://github.com/framesurge/perseus")[Perseus] which provides reactivity with #link("https://github.com/sycamore-rs/sycamore")[Sycamore] and is styled using #link("https://tailwindcss.com/")[TailwindCSS] and #link("https://daisyui.com/")[DaisyUI]. 

The backend framework is #link("https://github.com/tokio-rs/axum")[Axum] which uses #link("https://github.com/launchbadge/sqlx")[sqlx] to connect to a #link("https://www.postgresql.org")[Postgres] instance. 

The app is currently hosted by the #link("https://csclub.uwaterloo.ca/resources/services/cloud-accounts/")[Computer Science Club].

#pagebreak(weak: true)
= Plan for Sample Data

*Description of Sample Data*:
- The *courses* table is populated with 266 courses collected from the `/v3/Courses/{termCode}` Waterloo OpenData API endpoint. Each row in the table consists of key metadata (attributes) for the course including: course_id, subject code, catalogue number, academic level, description, and more. For sample data, we only consider Spring 2023 (1235) courses.
- The *course_offerings* table is populated with 40 entries that contain offering data corresponding to 40 entries from the _courses_ table. In particular, this table stores the course_id (FK referencing _courses_ table), year and term in which the course is offered as well as the current and max enrollment for the course.
- The *course_schedule* table contains schedule data for sections of a course offering. Thus, each entry maps back to some row in the course_offerings table through the course_offering_id (foreign key) attribute. This table stores granular data for sections of a course offering. In particular, each tuple contains the start and end times for an offering's section in addition to the days of the week when said section is active. Because it is sample data, we only store one section for each course offering, hence we have 40 entries in this table as well.
*Sample Data Generation*:
- For Milestone 1, the sample data was generated by hardcoded insert queries that were created through a mix of ad-hoc scripts that pulled course data from the OpenData API and programatically generated insert queries and through manual construction.
- But going forwards, the plan is that the data import, generation and cleaning will all be taken care of by the Periodic Synchronizer (see R6 for details). This feature will periodically pull data from the API and update the live database. For sample data, we may only hit the API once (ex. when the app is deployed).

*Using Data to populate the Database*:
The general approach for adding data to the database can be broken down into the following steps:
- We first add the general courses data to _courses_ table. The data being added corresponds to content returned by the `/v3/Courses/{termCode}` endpoint.
- For entries in the _courses_ table that are being offered in the current term, add the offering metadata to the _course_offerings_ table. This step corresponds to data being returned from the `/v3/ClassSchedules/{termCode}/{subject}/{catalogNumber}` endpoint. 
- For each offering, we then want to add the available schedules for however many sections we have for the course being offered. The data being added here is also returned by the previous endpoint within the `scheduleData` attribute of the JSON object.
- Once this is done, we will then parse the returned metadata from previous endpoint(s) to extract the prerequisite details for the courses and populate the `prerequisites, required_prerequisites` and `optional_preqrequisites` tables respectively. We may either come up with our own algorithm or look at what UWFlow did for inspiration. 


= Plan for Production Data
This is not required for Milestone 1.

#pagebreak(weak: true)
= Database Schema
- A list of assumptions that you are making about the data being modelled. 
- An E/R diagram for your database design. 
- A list of database tables with keys declared (relational data model). 

#image("image.png")


Course (\
#h(1cm)    #underline[id] serial Primary key, \
#h(1cm)     catalog_number int, \
#h(1cm)     subject_code varchar, \
#h(1cm)     external_id varchar, \
#h(1cm)     academic_level varchar, \
#h(1cm)     title text, \
#h(1cm)     description text, \
#h(1cm)     prerequisites_id int\  
)

#parbreak()
        
course_offerings (\
#h(1cm)   #underline[id] serial Primary key, \
#h(1cm)   course_id integer, \
#h(1cm)   year int, \
#h(1cm)   term varchar, \
#h(1cm)   max_enrollment int,\ 
#h(1cm)   current_enrollment int)\
FK: course_id references courses (id) \

#parbreak()


course_schedule ( \
#h(1cm)    #underline[id] serial Primary key, \
#h(1cm)    section varchar, \
#h(1cm)    start_time time, \
#h(1cm)    end_time time, \
#h(1cm)    meeting_days varchar, \
#h(1cm)    instructor_name varchar, \
#h(1cm)    room_name varchar, \
#h(1cm)    campus_name varchar, \
#h(1cm)    course_offering_id integer, \
#h(1cm)    max_enrollment int, \
#h(1cm)    current_enrollment in) \
FK: course_offering_id references course_offerings(id)

prerequisite( \
#h(1cm) id Primary key
)

#parbreak()

required_prerequisites ( \
#h(1cm)    prerequisite_id varchar, \
#h(1cm)    course_id integer) \
PK: [prerequisite_id, course_id]
FK: course_offering_id references prerequisite(id)
FK: course_id references course(id)

#parbreak()
    
optional_prerequisites ( \
#h(1cm)    prerequisite_id varchar, \
#h(1cm)    course_id integer) \
PK: [prerequisite_id, course_id]
FK: course_offering_id references prerequisite(id)
FK: course_id references course(id)




#pagebreak(weak: true)
#feature[Periodic synchronization of the DB with UWaterloo API
][ The Synchronizer will periodically retrieve data from the #link("https://openapi.data.uwaterloo.ca/api-docs/index.html")[UWaterloo Open Data API] for courses, their offerings and schedules (if available), instructors as well as valid terms and update the live database. The motivation behind this feature is to relieve group members of the burden of having to create custom scripts and queries in order to populate the database—regardless of whether it's sample or production data. This is an internal feature so it will only be of concern to group members and thus we will be its only users. Even from an internal standpoint, the feature is not something that members will interact with directly. Once deployed, it is inteded to be an automated, background job that runs once for sample data and will run periodically for production data (ex. once per day) to hit the API endpoint and consistently update the prod database. This will be especially important for course attributes that demand up-to-date live counts such as `current_enrollment` for course section(s).
][

As an example, the following query represents an `insert` that will populate the `course_offerings` table with corresponding data for a subset of the courses present in the `courses` table, assuming that `courses` was populated by the first insert query outlined in #link("https://github.com/GauravTalreja/ebb/blob/main/deliverables/milestone%201/R5/test-sample.sql")[R5/test-sample.sql]. In this case, the expected output is Postgres output is for a successful insert job. It just depicts: `INSERT (old_rows, new_rows)` for the table. It has values (0, 40), which is expected since we insert 40 rows into a fresh table (and database).

*Note*: The queries that populate the `courses` and `course_schedule` tables were skipped due to size concerns. But they can be found on GitHub:
- The insert query for populating the #link("https://github.com/GauravTalreja/ebb/blob/main/sample/queries/insert_courses.sql")[courses] table.
- The insert query for populating the #link("https://github.com/GauravTalreja/ebb/blob/main/sample/queries/insert_schedules.sql")[course_schedule] table.

*Insert Query*

][```sql
INSERT INTO course_offerings (course_id, year, term, max_enrollment, current_enrollment)
VALUES
    (2, 2023, 'spring', 75, 56),
    (5, 2023, 'spring', 2, 2),
    (6, 2023, 'spring', 123, 118),
    (10, 2023, 'spring', 24, 0),
    (15, 2023, 'spring', 60, 22),
    (18, 2023, 'spring', 40, 17),
    (24, 2023, 'spring', 116, 0),
    (31, 2023, 'spring', 100, 94),
    (32, 2023, 'spring', 30, 12),
    (34, 2023, 'spring', 1, 0),
    (38, 2023, 'spring', 5, 5),
    (40, 2023, 'spring', 60, 39),
    (42, 2023, 'spring', 55, 38),
    (62, 2023, 'spring', 50, 17),
    (64, 2023, 'spring', 35, 10),
    (69, 2023, 'spring', 200, 148),
    (83, 2023, 'spring', 500, 5),
    (89, 2023, 'spring', 40, 17),
    (98, 2023, 'spring', 42, 42),
    (102, 2023, 'spring', 35, 1),
    (121, 2023, 'spring', 40, 38),
    (131, 2023, 'spring', 1, 0),
    (132, 2023, 'spring', 60, 35),
    (135, 2023, 'spring', 90, 89),
    (140, 2023, 'spring', 157, 157),
    (148, 2023, 'spring', 5, 4),
    (151, 2023, 'spring', 130, 109),
    (154, 2023, 'spring', 100, 83),
    (165, 2023, 'spring', 5, 0),
    (166, 2023, 'spring', 3500, 319),
    (179, 2023, 'spring', 120, 21),
    (180, 2023, 'spring', 40, 38),
    (183, 2023, 'spring', 25, 11),
    (184, 2023, 'spring', 25, 0),
    (185, 2023, 'spring', 15, 11),
    (191, 2023, 'spring', 100, 83),
    (193, 2023, 'spring', 60, 28),
    (194, 2023, 'spring', 15, 4),
    (200, 2023, 'spring', 42, 36),
    (201, 2023, 'spring', 2, 1)
ON CONFLICT DO NOTHING;
```][
  *Sample Output*
  
  Insert 0 40
]

#feature[Display a list of filtered courses
][
  The user should be able to filter courses by name, term, time, location, etc.  
][
  This query implements two filters for sample data, by year and course code. It also returns course offerings as JSON.
][```sql
SELECT
    c.id,
    c.catalog_number,
    c.subject_code,
    c.title,
    c.external_id,
(
        SELECT
            COALESCE(NULLIF(jsonb_agg(DISTINCT co.*), '[null]'), '[]'::jsonb)
        FROM
            course_offerings co
        WHERE
            c.id = co.course_id
            AND co.year = 2023) AS "offerings!: _"
FROM
    courses c
    LEFT JOIN course_offerings co ON c.id = co.course_id
WHERE
    c.subject_code || c.catalog_number::VARCHAR ILIKE 'cs%'
GROUP BY
    c.id,
    c.subject_code
ORDER BY
    c.catalog_number;
```][
  #image("Screenshot 2023-06-22 at 11.42.55 PM.png")
]

#feature[Course Prerequisites
][
  On the course details page, 
  show paths that the current course leads to.
][
  Naive approach for sample data
][```sql

```][
  Sample Output
]

#feature[Smart Queries
][
  Search for courses with satsified prerequisites that have no conflicts with selected courses.
][
  Naive approach for sample data
][```sql
SELECT a FROM r WHERE b = 0
```][
  Sample Output
]


#set heading(numbering: none)
= R17. Members
== Gaurav Talreja
- Reorganized the workspace with separate sample and production modules that implemment traits dynamically dispatched by the backend, allowing the storage module to cleanly run in either mode without affecting the rest of the app.
- Reorganized the workspace to allow model sharing across the full stack.
- Created a Full Page Layout component required by all webpages.
- Added responsiveness to the front-end.
- Implemented themes (a fancy feature.)
- Generated Rust bindings for the OpenData API.
- Prepared report template, and wrote R1, R2, R7.
- Significantly reworked Justin's code to work with build time guarantees with for generic data types.

== Justin Kwan
- Contributed to feature planning and architecture of the application
- Implemented first two features, including data access layer in Rust application that executes SQL select queries to retrieve the list of courses and aggregated offerings and API endpoints to serve JSON data
- Modelled courses, offerings, schedules, and recursive prerequisites relationship tables and created ER diagram of the schema
- Wrote and tested two SQL queries to select a list of courses and offerings, along with selecting a single course, offerings for the same year, and details
- Created indices on tables for heavily accessed columns used for join selection and selection criteria of courses, planned performance benchmarking roadmap (using EXPLAIN ANALYZE to understand E2E query and per operator cost + latency)

== Sally Hong
- Assisted building of front end page layout.
- Created front end component for search page and course description page, including course table, filter, section table.
- Implemented responsive design for course search and description page.
- Populated course search page with back end qeury data.
- Assisted setting up front end page style and aesthetics.




== Saumya Khati
- Added the insert queries to populate the `courses`, `course_offerings` and `course_schedule` tables. 
- Worked with Justin on schema design for some of the tables.
- Discovered and documented schema issues as part of database population. Listed in #link("https://github.com/GauravTalreja/ebb/issues/32")[no. 32].
- Wrote R3 and R6 of this report.

== Shake Patel

#lorem(40)
