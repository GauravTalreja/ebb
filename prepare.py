#!/usr/bin/env python3

import json
import glob
import os
import shutil
import subprocess

shutil.rmtree("target/sqlx", ignore_errors=True)

os.environ["SQLX_OFFLINE"] = "false"
subprocess.run(["cargo", "clean"])
subprocess.run(["cargo", "check", "--workspace"])

data   = [json.load(open(path, "r")) for path in glob.glob("target/sqlx/*/query-*.json")]
merged = { "db": "PostgreSQL", **{ v["hash"]: v for v in data } }

json.dump(merged, open("sqlx-data.json", "w"), indent=4)
