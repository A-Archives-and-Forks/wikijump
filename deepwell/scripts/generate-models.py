#!/usr/bin/env python3

from functools import cached_property
from glob import iglob
import os
from io import StringIO
import re
import shutil
import subprocess
import sys

DATABASE_URL = "postgres://wikijump:wikijump@localhost/wikijump"
MODELS_DIRECTORY = "src/models"

TIMESTAMP_ATTRIBUTE = "time::serde::rfc3339"
OPTION_TIMESTAMP_ATTRIBUTE = "time::serde::rfc3339::option"
TIMESTAMP_FIELD_REGEX = re.compile(r"( *)pub ([^:]+): TimeDateTimeWithTimeZone,\n")
OPTION_TIMESTAMP_FIELD_REGEX = re.compile(
    r"( *)pub ([^:]+): Option<TimeDateTimeWithTimeZone>,\n",
)


def chdir_to_crate_root():
    print("Changing directory to deepwell")
    this_script = os.path.abspath(sys.argv[0])
    crate_root = os.path.dirname(os.path.dirname(this_script))
    os.chdir(crate_root)


def remove_existing_models():
    if os.path.isdir(MODELS_DIRECTORY):
        print(f"Deleting existing {MODELS_DIRECTORY}")
        shutil.rmtree(MODELS_DIRECTORY)


def run_sea_orm_cli():
    print("Running sea-orm-cli generate entity")
    subprocess.check_call(
        [
            "sea-orm-cli",
            "generate",
            "entity",
            "--verbose",
            "--date-time-crate",
            "time",
            "--with-serde",
            "both",
            "--enum-extra-attributes",
            'serde(rename_all = "kebab-case")',
            "--with-copy-enums",
            "--database-url",
            DATABASE_URL,
            "--output-dir",
            MODELS_DIRECTORY,
        ]
    )



class ModelFileRewriter:
    __slots__ = ("path", "filename", "lines", "original_line_count")

    def __init__(self, path):
        self.path = path
        self.filename = os.path.basename(path)

        with open(self.path) as file:
            self.lines = file.readlines()
            self.original_line_count = len(self.lines)

    @property
    def current_line_count(self):
        return len(self.lines)

    @property
    def line_iter(self):
        return enumerate(self.lines)

    def rewrite(self):
        # We're making these separate methods for readability
        #
        # Sure it would be faster to do it all in one pass,
        # but given that this script is run on-demand as part
        # of development, it is not part of any hot path.
        self.insert_timestamp_fields()
        self.replace_enum_types()

    def save(self):
        if self.current_line_count != self.original_line_count:
            print(f"Rewriting {self.filename}")
            with open(self.path, "w") as file:
                file.writelines(self.lines)

    ## SPECIFIC REWRITE RULES ##

    def insert_timestamp_fields(self):
        def find_regex_match(line):
            PATTERNS = [
                (TIMESTAMP_FIELD_REGEX, TIMESTAMP_ATTRIBUTE),
                (OPTION_TIMESTAMP_FIELD_REGEX, OPTION_TIMESTAMP_ATTRIBUTE),
            ]

            for regex, attribute in PATTERNS:
                match = regex.match(line)
                if match is not None:
                    indent = match[1]
                    return indent, attribute

            return None

        lines_to_insert = []  # (index, line)
        for idx, line in self.line_iter:
            match find_regex_match(line):
                case indent, attribute:
                    # Insert #[serde] on the line before
                    lines_to_insert.append((idx, f'{indent}#[serde(with = "{attribute}")]\n'))

        # Insert the lines in reverse order to not mess up indices
        for idx, line in reversed(lines_to_insert):
            self.lines.insert(idx, line)

    def replace_enum_types(self):

    @cached_property
    def start_of_import_block(self):
        for idx, line in self.line_iter:
            if line.startswith("use"):
                return idx

        raise ValueError("No import block found in entity file")

    def format_use_block(self, types):
        if len(types) == 1:
            the_type = next(types)  # the one and only
            return f"use crate::types::{the_type};"
        else:
            return f"use crate::types::{{{', '.join(sorted(types))}}};"


if __name__ == "__main__":
    chdir_to_crate_root()
    remove_existing_models()
    run_sea_orm_cli()

    for path in iglob(os.path.join(MODELS_DIRECTORY, "*.rs")):
        if os.path.basename(path) == "mod.rs":
            # Not an entity file corresponding to a table
            continue

        model = ModelFileRewriter(path)
        model.rewrite()
        model.save()
