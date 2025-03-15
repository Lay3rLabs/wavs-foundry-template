import os
import re
import subprocess
import time
from dataclasses import dataclass
from typing import Dict, List, Tuple

import requests

# current file location
REPO_ROOT = subprocess.check_output(["git", "rev-parse", "--show-toplevel"]).decode("utf-8").strip()
print(f"Root dir: {REPO_ROOT}")

# Commands which block the process
BLOCKING_START_COMMANDS = ["local-ic start", "make testnet", "make sh-testnet"]

DOCS_TO_CI = ["README.md"]

START_PID = -1
DEBUGGING = False

def main():
    paths = config.iterate_paths()
    file_path = next(paths)

    values = parse_markdown_code_blocks(file_path)
    for value in values:
        if value.ignored: continue

        value.run_commands()

class Config:
    paths: List[str] = [] # this will be loaded with the prefix of the absolute path of the repo
    env_var: Dict[str, str] = {}
    cleanup_cmds: List[str] = []
    only_run_bash: bool = True
    ignore_commands: List[str] = ['gh repo create'] # TODO: example

    def __init__(self, paths: List[str], env_var: Dict[str, str], cleanup_cmds: List[str]):
        self.paths = paths
        self.env_var = env_var
        self.cleanup_cmds = cleanup_cmds

    def iterate_paths(self):
        for path in self.paths:
            yield os.path.join(REPO_ROOT, path)

config = Config(
    paths=DOCS_TO_CI,
    env_var={"WAVS_IN_BACKGROUND": "true"},
    cleanup_cmds=["killall anvil", "docker compose rm"]
)

@dataclass
class DocsValue:
    language: str # bash, python, rust, etc
    tags: List[str] # (e.g., 'docs-ci-ignore') in the ```bash tag1, tag2```
    content: str # unmodified content
    ignored: bool
    commands: List[str]

    def run_commands(self, additional_env: Dict[str, str] = {}):
        # Start with the current environment
        env = os.environ.copy()

        # Add any additional environment variables
        env.update(additional_env)

        for command in self.commands:
            if command in config.ignore_commands:
                continue

            print(f"Running command: {command}")

            # Use the merged environment when running the command
            process = subprocess.Popen(
                command,
                shell=True,
                env=env,
                cwd=REPO_ROOT
            )
            process.wait()

            if process.returncode != 0:
                print(f"Error running command: {command}")
                break

            print(f"Command finished: {command}")

    def __str__(self):
        # content={self.content.replace('\n', '\\n')}
        return f"DocsValue(language={self.language}, tags={self.tags}, ignored={self.ignored}, commands={self.commands})"

    def print(self):
        print(self.__str__())

def parse_markdown_code_blocks(file_path: str) -> List[DocsValue]:
    """
    Parse a markdown file and extract all code blocks with their language and content.

    Args:
        file_path: Path to the markdown file to parse

    Returns:
        A list of dictionaries, each containing:
        - 'language': The language of the code block
        - 'content': The content of the code block
        - 'should_run': Boolean indicating if this block should be executed (True for bash blocks without docs-ci-ignore)
    """
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    # Regex pattern to match code blocks: ```language ... ```
    # Capturing groups:
    # 1. Language and any additional markers (e.g., 'bash docs-ci-ignore')
    # 2. Content of the code block
    pattern = r'```(.*?)\n(.*?)```'

    # Find all matches with re.DOTALL to include newlines
    matches = re.findall(pattern, content, re.DOTALL)

    results: List[DocsValue] = []
    for language_info, block_content in matches:
        language_info = language_info.strip()
        language_parts = language_info.split()

        # Get the primary language
        language = language_parts[0] if language_parts else ''
        tags = language_parts[1:] if len(language_parts) > 1 else []

        ignored = 'docs-ci-ignore' in tags  # TODO: only run bash?

        content = str(block_content).strip()

        value = DocsValue(
            language=language,
            tags=tags,
            content=content,
            ignored=ignored,
            commands=[]
        )

        # using regex, remove any sections of code that start with a comment '#' and end with a new line '\n', this info is not needed.
        # an example is '# Install packages (npm & submodules)\nmake setup\n\n# Build the contracts\nforge build\n\n# Run the solidity tests\nforge test'
        # this should just be `make setup\nforge build\nforge test`
        content = re.sub(r'^#.*\n', '', content, flags=re.MULTILINE)

        # if there is a # comment with no further \n after it, remove it
        content = re.sub(r'#.*$', '', content, flags=re.MULTILINE).strip()

        # ensure only 1 \n is present, not ever \n\n or more
        content = re.sub(r'\n+', '\n', content)

        # split by the \n to get a list of commands
        commands = content.split('\n')

        value.commands = commands
        results.append(value)

    return results

if __name__ == "__main__":
    main()
