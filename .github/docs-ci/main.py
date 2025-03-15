import os
import re
import signal
import subprocess
import time
from dataclasses import dataclass, field
from typing import Dict, List, Tuple

# current file location
REPO_ROOT = subprocess.check_output(["git", "rev-parse", "--show-toplevel"]).decode("utf-8").strip()

# Store PIDs of background processes for later cleanup
background_processes = []

def main():
    paths = config.iterate_paths()
    file_path = next(paths)

    config.run_pre_cmds(hide_output=True)

    try:
        values = parse_markdown_code_blocks(file_path)
        for value in values:
            if value.ignored: continue

            value.print()
            # value.run_commands(additional_env=config.env_var)
    finally:
        config.run_cleanup_cmds(hide_output=True)
        cleanup_background_processes()

class Config:
    paths: List[str] = [] # this will be loaded with the prefix of the absolute path of the repo
    env_var: Dict[str, str] = {}
    pre_cmds: List[str] = []
    cleanup_cmds: List[str] = []
    only_run_bash: bool = True
    ignore_commands: List[str] = ['gh repo create'] # TODO: example

    def __init__(self, paths: List[str], env_var: Dict[str, str], cleanup_cmds: List[str], pre_cmds: List[str] = []):
        self.paths = paths
        self.env_var = env_var
        self.cleanup_cmds = cleanup_cmds
        self.pre_cmds = pre_cmds

    def iterate_paths(self):
        for path in self.paths:
            yield os.path.join(REPO_ROOT, path)

    def __run_cmd(self, cmd: str, hide_output: bool):
        subprocess.run(cmd, shell=True, cwd=REPO_ROOT, stdout=subprocess.DEVNULL if hide_output else None, stderr=subprocess.DEVNULL if hide_output else None)

    def run_pre_cmds(self, hide_output: bool = False):
        for cmd in self.pre_cmds:
            self.__run_cmd(cmd, hide_output)

    def run_cleanup_cmds(self, hide_output: bool = False):
        for cmd in self.cleanup_cmds:
            self.__run_cmd(cmd, hide_output)

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
    post_delay: int = 0 # delay in seconds after the command is run
    envs: Dict[str, str] = field(default_factory=dict)

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

        if self.post_delay > 0:
            print(f"Sleeping for {self.post_delay} seconds after running commands...")
            time.sleep(self.post_delay)

        return success


    def __str__(self):
        # content={self.content.replace('\n', '\\n')}
        return f"DocsValue(language={self.language}, tags={self.tags}, ignored={self.ignored}, commands={self.commands}, background={self.background}, post_delay={self.post_delay})"

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
        post_delay = int([tag.split('=')[1] for tag in tags if 'docs-ci-post-delay' in tag][0]) if any('docs-ci-post-delay' in tag for tag in tags) else 0

        content = str(block_content).strip()

        value = DocsValue(
            language=language,
            tags=tags,
            content=content,
            ignored=ignored,
            background=background,
            post_delay=post_delay,
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

        # parse out env vars from commands. an example format is:
        # export SERVICE_MANAGER_ADDR=`make get-eigen-service-manager-from-deploy
        env_vars = {}
        for command in commands:
            env_vars.update(parse_env(command))



        value.commands = commands
        results.append(value)

    return results

def execute_backticks(value: str) -> str:
    """
    Execute commands inside backticks and return the value with output substituted.

    Args:
        value: String that may contain backtick commands

    Returns:
        String with backtick commands replaced by their output
    """
    backtick_match = re.search(r'`(.*?)`', value)
    if not backtick_match:
        return value

    cmd_to_execute = backtick_match.group(1)
    try:
        executed_value = subprocess.check_output(
            cmd_to_execute, shell=True, text=True, cwd=REPO_ROOT
        ).strip()
        return value.replace(f"`{cmd_to_execute}`", executed_value)
    except subprocess.CalledProcessError as e:
        print(f"Warning: Failed to execute command in backticks: {cmd_to_execute}")
        print(f"Error: {e}")
        return value  # Return original value if execution fails

def parse_env(command: str) -> Dict[str, str]:
    """
    Parse environment variable commands, handling backtick execution and inline env vars.

    Args:
        command: String containing potential env var assignments and commands

    Returns:
        Dictionary of environment variables (can be empty if no env vars found)
    """
    # Early return if no '=' is present in the command
    if '=' not in command:
        return {}

    # First check for export KEY=VALUE pattern
    export_match = re.match(r'^export\s+([A-Za-z_][A-Za-z0-9_]*)=(.*)$', command.strip())
    if export_match:
        key = export_match.group(1)
        value = execute_backticks(export_match.group(2))
        return {key: value}

    # Check for inline environment variables (KEY=VALUE command args)
    inline_match = re.match(r'^([A-Za-z_][A-Za-z0-9_]*=[^ ]+(?: [A-Za-z_][A-Za-z0-9_]*=[^ ]+)*) (.+)$', command.strip())
    if inline_match:
        env_vars = {}
        env_part = inline_match.group(1)

        # Extract all KEY=VALUE pairs
        for pair in env_part.split():
            if '=' in pair:
                key, value = pair.split('=', 1)
                env_vars[key] = execute_backticks(value)

        return env_vars

    # Check for standalone KEY=VALUE
    standalone_match = re.match(r'^([A-Za-z_][A-Za-z0-9_]*)=(.*)$', command.strip())
    if standalone_match:
        key = standalone_match.group(1)
        value = execute_backticks(standalone_match.group(2))
        return {key: value}

    # If we get here, there were no environment variables we could parse
    return {}


if __name__ == "__main__":
    # print(parse_env('export SERVICE_MANAGER_ADDR=`make get-eigen-service-manager-from-deploy`'))
    # print(parse_env('export SERVICE_MANAGER_ADDR=123'))
    # print(parse_env('SERVICE_CONFIG_FILE=service_config.json make deploy-service'))
    main()
