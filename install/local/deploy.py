#!/usr/bin/env python3

import argparse
import os


def chdir_wikijump():
    # Determine where this script is
    full_path = __file__
    if not os.path.abspath(full_path):
        full_path = os.path.join(os.getcwd(), full_path)

    # The directory it's located in should be install/local/
    # So we should change to that
    install_local_dir = os.path.dirname(full_path)
    os.chdir(install_local_dir)


if __name__ == "__main__":
    # Parse arguments
    argparser = argparse.ArgumentParser()
    argparser.add_argument(
        "-s",
        "--sudo",
        action="store_true",
        help="Runs the docker-compose command using sudo",
    )
    argparser.add_argument(
        "-d",
        "--no-dev",
        action="store_true",
        help="Do not use the docker-compose.dev.yaml convenience bindings",
    )
    argparser.add_argument(
        "-x",
        "--no-chdir",
        action="store_true",
        help="Do not change directory before attempting to run docker-compose",
    )
    argparser.add_argument("command", nargs=argparse.REMAINDER)
    args = argparser.parse_args()

    # Get into the right context
    if not args.no_chdir:
        chdir_wikijump()

    # Build command-line argument list
    cmdline = [
        "docker",
        "compose",
        "-p",
        "wikijump",
        "-f",
        "docker-compose.yaml",
    ]

    if args.sudo:
        cmdline.insert(0, "sudo")

    if not args.no_dev:
        cmdline.extend(("-f", "docker-compose.dev.yaml"))

    cmdline.extend(args.command)

    # We use exec instead of subprocess since this is just a launch script,
    # it doesn't need to do any work after starting docker-compose.
    print(" ".join(cmdline))
    os.execvp(cmdline[0], cmdline)
