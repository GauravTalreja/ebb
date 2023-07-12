import json
import os

def write_json_to_file(file_name, json_object):
    # Get the directory path of the script
    script_directory = os.path.dirname(os.path.abspath(__file__))

    # Build the full file path
    file_path = os.path.join(script_directory, file_name)

    # Open the file in write mode
    with open(file_path, "w") as file:
        # Write the pretty-printed JSON to the file
        json.dump(json_object, file, indent=4)


def write_strings_to_file(file_name, string_list):
    with open(file_name, 'w') as file:
        for text in string_list:
            file.write(text + '\n')

def write_string_to_file(file_name, text):
    with open(file_name, 'w') as file:
        file.write(text + '\n')