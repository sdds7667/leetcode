import random
from pathlib import Path
from argparse import ArgumentParser
import shutil
import os

if __name__ == "__main__":
    arguments = ArgumentParser(
        description="Randomly picks a template for the problem and initializes the proper template")

    arguments.add_argument("problem-number", type=int,
                           help="The number of the problem to create the template for")
    probNumber = arguments.parse_args().__dict__['problem-number']
    probPath = Path(f"prob{probNumber}")
    if (probPath.exists()):
        print("Already exists; Removing")
        shutil.rmtree(probPath)

    ignored_templates = ["java-intellij"]

    shutil.copytree((choice := random.choice(
        list(filter(lambda x: x.name not in ignored_templates, Path("./templates").iterdir())))), probPath)

    print(f"Chosen {choice.name}")
    os.system(
        f"git checkout -b prob/{probNumber} && git add . && git commit -am \"prob({probNumber}): Adds the problem skeleton in {choice.name}\"")
