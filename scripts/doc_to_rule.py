#!/usr/bin/env python3

import os
import sys
import argparse
from pathlib import Path
import openai
from typing import Optional
from dotenv import load_dotenv

def clean_triple_backticks(content: str) -> str:
    """Remove triple backticks from the start and end of the file while preserving code block backticks."""
    # Remove leading triple backticks if they exist
    if content.startswith('```'):
        content = content[3:]
    
    # Remove trailing triple backticks if they exist
    if content.endswith('```'):
        content = content[:-3]
    
    return content

def clean_leading_blank_lines(content: str) -> str:
    """Remove any blank lines at the start of the content."""
    lines = content.splitlines()
    # Find the first non-empty line
    start_idx = 0
    for i, line in enumerate(lines):
        if line.strip():
            start_idx = i
            break
    # Return content starting from the first non-empty line
    return '\n'.join(lines[start_idx:])

def read_file(file_path: str) -> str:
    """Read the content of a file."""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
            if not content.strip():
                raise ValueError(f"File {file_path} is empty")
            return content
    except UnicodeDecodeError:
        print(f"Error: File {file_path} is not UTF-8 encoded")
        sys.exit(1)
    except Exception as e:
        print(f"Error reading file {file_path}: {e}")
        sys.exit(1)

def write_file(file_path: str, content: str) -> None:
    """Write content to a file."""
    try:
        os.makedirs(os.path.dirname(file_path), exist_ok=True)
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
    except Exception as e:
        print(f"Error writing file {file_path}: {e}")
        sys.exit(1)

def validate_doc_content(content: str) -> bool:
    """Validate that the documentation content is properly formatted."""
    if not content.strip():
        return False
    if not any(line.strip() for line in content.split('\n')):
        return False
    return True

def convert_doc_to_rule(doc_content: str, api_key: str) -> str:
    """Convert documentation content to a Cursor rule file using OpenAI API."""
    if not validate_doc_content(doc_content):
        raise ValueError("Invalid documentation content")

    client = openai.OpenAI(api_key=api_key)
    
    cursor_rules_structure = """# Cursor Rules Location

How to add new cursor rules to the project

1. Always place rule files in PROJECT_ROOT/.cursor/rules/:
    .cursor/rules/
    ├── your-rule-name.mdc
    ├── another-rule.mdc

2. Follow the naming convention:
    - Use kebab-case for filenames
    - Always use .mdc extension
    - Make names descriptive of the rule's purpose

3. Directory structure:
    PROJECT_ROOT/
    ├── .cursor/
    │   └── rules/
    │       ├── your-rule-name.mdc
    │       └── ...
    └── ...

4. Never place rule files:
    - In the project root
    - In subdirectories outside .cursor/rules
    - In any other location

5. Cursor rules have the following structure:

---
description: Short description of the rule's purpose
globs: **/*.rs
alwaysApply: true
---
# Rule Title

Main content explaining the rule with markdown formatting.

1. Step-by-step instructions
2. Code examples
3. Guidelines
4. Best practices
5. full markdown links at end, one per line with descriptions

Example:
```typescript
// Good example
function goodExample() {
  // Implementation following guidelines
}

// Bad example
function badExample() {
  // Implementation not following guidelines
}
```

For more information:
- [Official Documentation](https://docs.example.com)
- [API Reference](https://api.example.com)
"""

    prompt = f"""Convert the following documentation into a Cursor rule file following this structure:

{cursor_rules_structure}

Remember to be very concise. this is for llms to read.

Important formatting rules:
1. Use full markdown links at the end (e.g. [Link Text](https://url.com))
2. Do not add triple backticks at the start or end of the file
3. Keep code blocks within the content only
4. Preserve all code examples and their formatting
5. Maintain the logical structure of the documentation
6. Include all relevant code snippets and examples

Here's the documentation to convert:

{doc_content}
"""

    response = client.chat.completions.create(
        model="gpt-4-turbo-preview",
        messages=[
            {"role": "system", "content": "You are a technical documentation expert who specializes in converting documentation into concise Cursor rule files. You follow the exact structure provided in the prompt and make the content as direct and concise as possible. Use as few words as possible to convey meaning. For references, always use full markdown links like [Link Text](https://url.com) - never use @ references. Never add triple backticks (```) at the start or end of the file. Preserve all code examples and their formatting."},
            {"role": "user", "content": prompt}
        ],
        temperature=0.3,
    )
    
    # Clean up the content
    content = response.choices[0].message.content
    content = clean_triple_backticks(content)
    content = clean_leading_blank_lines(content)
    return content

def main():
    parser = argparse.ArgumentParser(description='Convert documentation to Cursor rule files')
    parser.add_argument('input_file', help='Path to the input documentation file')
    parser.add_argument('--output-dir', default='.cursor/rules', help='Output directory for rule files')
    parser.add_argument('--api-key', help='OpenAI API key (or set OPENAI_API_KEY env var)')
    parser.add_argument('--env-file', default=os.path.join(os.path.dirname(__file__), '.env'), help='Path to .env file')
    args = parser.parse_args()

    # Load environment variables from .env file
    load_dotenv(args.env_file)

    # Get API key from args, environment, or .env file
    api_key = args.api_key or os.getenv('OPENAI_API_KEY')
    if not api_key:
        print("Error: OpenAI API key not provided. Use --api-key, set OPENAI_API_KEY environment variable, or add it to your .env file.")
        sys.exit(1)

    # Read input file
    try:
        doc_content = read_file(args.input_file)
    except Exception as e:
        print(f"Error reading input file: {e}")
        sys.exit(1)

    # Convert documentation to rule file
    try:
        rule_content = convert_doc_to_rule(doc_content, api_key)
    except Exception as e:
        print(f"Error converting documentation: {e}")
        sys.exit(1)

    # Generate output filename
    input_path = Path(args.input_file)
    output_filename = input_path.stem.replace('_', '-').lower() + '.mdc'
    output_path = os.path.join(args.output_dir, output_filename)

    # Write rule file
    try:
        write_file(output_path, rule_content)
        print(f"Successfully created rule file: {output_path}")
    except Exception as e:
        print(f"Error writing rule file: {e}")
        sys.exit(1)

if __name__ == '__main__':
    main() 
