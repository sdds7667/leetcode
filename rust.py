"""Utility for creating a rust project for a specific problem"""
import argparse
import os


def parse_arguments():
    """Parses the arguments passed to the script. 
    :return: a tuple containing the problem number
    """
    arguments = argparse.ArgumentParser()
    arguments.add_argument("problem")
    return arguments.parse_args()


if __name__ == "__main__":
    problem, = parse_arguments()
    os.system(f"cargo new prob{problem}")
    os.system(f"git checkout -b prob/{problem}")
    os.system(f"code prob{problem}")
