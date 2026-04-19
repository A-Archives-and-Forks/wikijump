#!/usr/bin/env python3

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


def model_files():
    for path in iglob(os.path.join(MODELS_DIRECTORY, "*.rs")):
        filename = os.path.basename(path)
        yield path, filename


def add_datetime_format(path, filename):
    with open(path) as file:
        lines = file.readlines()
        original_len = file.tell()

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

    buffer = StringIO()
    for line in lines:
        match find_regex_match(line):
            case indent, attribute:
                # Write #[serde] line, then field
                buffer.write(f'{indent}#[serde(with = "{attribute}")]\n')
                buffer.write(line)

            case None:
                # Write line as-is
                buffer.write(line)

    modified_len = buffer.tell()
    if original_len != modified_len:
        print(f"Inserting serde attributes for datetime fields in {filename}")
        with open(path, "w") as file:
            file.write(buffer.getvalue())


def apply_patches(model_path, filename):
    patch_path = f"{model_path}.patch"
    if not os.path.isfile(patch_path):
        # No patch to process
        return

    print(f"Applying patch {patch_path}")
    # TODO
    # See https://github.com/SeaQL/sea-orm/issues/2358


if __name__ == "__main__":
    chdir_to_crate_root()
    remove_existing_models()
    run_sea_orm_cli()
    for path, filename in model_files():
        add_datetime_format(path, filename)
        apply_patches(path, filename)
