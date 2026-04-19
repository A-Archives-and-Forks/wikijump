#!/usr/bin/env python3

import os
import subprocess
import shutil
import sys

DATABASE_URL = "postgres://wikijump:wikijump@localhost/wikijump"
MODELS_DIRECTORY = "src/models"


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


if __name__ == "__main__":
    chdir_to_crate_root()
    remove_existing_models()
    run_sea_orm_cli()
