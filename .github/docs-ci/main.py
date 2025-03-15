import os
import re
import signal
import subprocess
from dataclasses import dataclass
from typing import Dict, List, Tuple

# current file location
REPO_ROOT = subprocess.check_output(["git", "rev-parse", "--show-toplevel"]).decode("utf-8").strip()
print(f"Root dir: {REPO_ROOT}")

# Store PIDs of background processes for later cleanup
background_processes = []

def main():
    paths = config.iterate_paths()
    file_path = next(paths)

    config.run_pre_cmds()

    try:
        values = parse_markdown_code_blocks(file_path)
        for value in values:
            if value.ignored: continue

            # value.print()
            value.run_commands(additional_env=config.env_var)
    finally:
        config.run_cleanup_cmds()
        cleanup_background_processes()

class Config:
    paths: List[str] = [] # this will be loaded with the prefix of the absolute path of the repo
    env_var: Dict[str, str] = {}
    pre_cmds: List[str] = []
    cleanup_cmds: List[str] = []
    only_run_bash: bool = True
    ignore_commands: List[str] = ['gh repo create'] # TODO: example
    # debugging?

    def __init__(self, paths: List[str], env_var: Dict[str, str], cleanup_cmds: List[str], pre_cmds: List[str] = []):
        self.paths = paths
        self.env_var = env_var
        self.cleanup_cmds = cleanup_cmds
        self.pre_cmds = pre_cmds

    def iterate_paths(self):
        for path in self.paths:
            yield os.path.join(REPO_ROOT, path)

    def __run_cmd(self, cmd: str):
        print(f"Running command: {cmd}")
        subprocess.run(cmd, shell=True, cwd=REPO_ROOT)

    def run_pre_cmds(self):
        for cmd in self.pre_cmds:
            self.__run_cmd(cmd)

    def run_cleanup_cmds(self):
        for cmd in self.cleanup_cmds:
            self.__run_cmd(cmd)

config = Config(
    # paths=["README.md"],
    paths=["README_WAVS.md"],
    env_var={"WAVS_IN_BACKGROUND": "true"},
    pre_cmds=["docker compose rm --stop --force --volumes", "docker rm -f wavs wavs-deploy-service-manager wavs-deploy-eigenlayer"],
    cleanup_cmds=["killall anvil"],
)

@dataclass
class DocsValue:
    language: str # bash, python, rust, etc
    tags: List[str] # (e.g., 'docs-ci-ignore') in the ```bash tag1, tag2```
    content: str # unmodified content
    ignored: bool
    commands: List[str]
    background: bool = False # if the command should run in the background i.e. it is blocking

    def run_commands(
        self,
        additional_env: Dict[str, str] = {},
        background_exclude_commands: List[str] = ["cp", "export", "cd", "mkdir", "echo", "cat"],
    ) -> bool: # success / failure returned
        '''
        Runs the commands
        '''

        env = os.environ.copy()
        env.update(additional_env)

        success = True

        for command in self.commands:
            if command in config.ignore_commands:
                continue

            # Determine if this specific command should run in background
            cmd_background = self.background
            if self.background:
                # Check if command starts with any excluded prefix
                first_word = command.strip().split()[0]
                if first_word in background_exclude_commands:
                    cmd_background = False

            # Add & if running in background and not already there
            if cmd_background and not command.strip().endswith('&'):
                command = f"{command} &"

            print(f"Running command: {command}" + (" (& added for background)" if cmd_background else ""))

            # Use the merged environment when running the command
            process = subprocess.Popen(
                command,
                shell=True,
                env=env,
                cwd=REPO_ROOT
            )

            if cmd_background:
                # Try to get PID of the actual background process
                try:
                    # Simple approach: the last word in the output of echo $! is the PID
                    pid_cmd = f"bash -c '{command} echo $!'"
                    pid_output = subprocess.check_output(pid_cmd, shell=True, text=True)
                    pid = int(pid_output.strip().split()[-1])
                    background_processes.append(pid)
                    print(f"Started background process with PID: {pid}")
                except Exception as e:
                    print(f"Warning: Could not track background process: {e}")
                    # If we can't get the specific PID, save the shell's PID as a fallback
                    if process.pid:
                        background_processes.append(process.pid)
            else:
                # For regular processes, wait and check return code
                process.wait()

                if process.returncode != 0:
                    print(f"Error running command: {command}")
                    success = False
                    break

                print(f"Command finished: {command}")

        return success


    def __str__(self):
        # content={self.content.replace('\n', '\\n')}
        return f"DocsValue(language={self.language}, tags={self.tags}, ignored={self.ignored}, commands={self.commands})"

    def print(self):
        print(self.__str__())

def cleanup_background_processes():
    """Kill all saved background processes"""
    if not background_processes:
        return

    print(f"Cleaning up {len(background_processes)} background processes...")
    for pid in background_processes:
        try:
            os.kill(pid, signal.SIGTERM)
            print(f"Terminated process with PID: {pid}")
        except OSError as e:
            print(f"Error terminating process {pid}: {e}")

    # Clear the list
    background_processes.clear()

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
        background = 'docs-ci-background' in tags

        content = str(block_content).strip()

        value = DocsValue(
            language=language,
            tags=tags,
            content=content,
            ignored=ignored,
            background=background,
            commands=[]
        )

        # using regex, remove any sections of code that start with a comment '#' and end with a new line '\n', this info is not needed.
        # an example is '# Install packages (npm & submodules)\nmake setup\n\n# Build the contracts\nforge build\n\n# Run the solidity tests\nforge test'
        # this should just be `make setup\nforge build\nforge test`
        # TODO: other comment types need to also be supported?
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
