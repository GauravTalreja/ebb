#import "template.typ": *

// Take a look at the file `template.typ` in the file panel
// to customize this template and discover how it works.
#show: project.with(
  title: "CS 348 - Milestone #0",
  authors: (
    "GT",
    "Sally Hong",
    "Saumya Khati",
    "Shake Patel",
    "Justin Kwan",
  ),
  date: "May 16, 2023",
  repo: "https://github.com/GauravTalreja/ebb",
  website: "https://ebb.csclub.cloud"
)

= Application Description

The vision is to build the ultimate search tool for courses offered by the University of Waterloo that combines #link("https://uwflow.com/")[UWFlow] and Quest's course search with a clean user interface. The primary improvement beyond the combination of Flow and Quest are *smart queries* that are built, but not directly constructed, from user input. For example, users could see all ENGL courses they can take with satisfied prereequisites alongside CS 348.

There is a multi-step plan to achieve the vision for smart queries.
+ Automate the data creation process from the #link("https://uwaterloo.ca/api/")[UWaterloo Open Data API]  and construct a database of courses for at least the current term and the next term.
+ Replicate the course search functionality of Quest and UWFlow (search by name, subject, faculty, time, location, instructor, class size, prerequisites etc.) with a clean user interface.
+ Implement the algorithms necessary for building smart queries.
+ Integrate a language learning model that builds smart queries from natural language.

Some other features that can be implemented anytime after step 2 are:
- User accounts with niceties such as prerequisites met by a user parsed from a transcript and stored.
- Integrate the data set for reviews and ratings from Flow.

This is a starry-eyed vision of the app. It is possible to satisfy the requirements of this project with just steps 1 and 2.

The target audience is University of Waterloo students. The administrators of the DBMS would be the team maintaining and deploying the app (currently this CS 348 group.)

= Platform and User Interface

The application is a website written entirely in #link("https://www.rust-lang.org/")[Rust]. The frontend web development framework is #link("https://github.com/framesurge/perseus")[Perseus] which provides reactivity with #link("https://github.com/sycamore-rs/sycamore")[Sycamore] and is styled using #link("https://tailwindcss.com/")[TailwindCSS] and #link("https://daisyui.com/")[DaisyUI]. The backend framework is #link("https://github.com/tokio-rs/axum")[Axum] and uses #link("https://github.com/launchbadge/sqlx")[sqlx] with a local or remote MySQL instance. The app is hosted by the #link("https://csclub.uwaterloo.ca/resources/services/cloud-accounts/")[Computer Science Club].

As of today, there are some issues with Perseus and Sycamore in interacting with the database server, hence they are not finalized. Nevertheless, the current API is able to generate a hello world page. 

= Members and Contributions
== Gaurav Talreja

- Pitched application features and tech stack.
- Incrementally added Perseus, Tailwind, DaisyUI, Axum and assisted with the integration of sqlx.
- Setup a cloud instance for deploying the server and database.
- Enabled GitHub Continuous Integration.
- Wrote the README and refined the report.

== Saumya Khati

- Contributed to the brainstorm session for potential features and queries the app can support. 
- Wrote the initial draft for the Milestone 0 Application Description section.- Prepared the draft report.

== Sally Hong

- Brainstormed app functionalities.
- Adjusted the interface of the index page. 

== Shake Patel

- Helped brainstorm functionalities
- Helped investigate tech stack

== Justin Kwan
- Pull Request: https://github.com/GauravTalreja/ebb/pull/2
- Established a connection pool to PostgreSQL database server running on localhost for higher throughput across concurrent clients on backend server
- Created a simple SQL table containing course entries, each containing a primary key id, course name, and course department columns with mock row entries
- Created SQL query to fuzzy search a list of courses matching a provided course name (select query)
- Exposed list of found courses through rest API given a course name url parameter
- Added sqlx.
- Implemented an API endpoint for the milestone 0 hello world.
- Modified the search bar component to use the API endpoint.