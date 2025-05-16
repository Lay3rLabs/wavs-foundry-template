#!/usr/bin/env python3

import os
import sys
import argparse
from pathlib import Path
import openai
from typing import Optional, List
from dotenv import load_dotenv
from doc_to_rule import convert_doc_to_rule, write_file

def get_project_root() -> Path:
    """Get the project root directory."""
    # Assuming the script is in scripts/ directory
    return Path(__file__).parent.parent

def get_scripts_dir() -> Path:
    """Get the scripts directory."""
    return Path(__file__).parent

def get_doc_files(components_dir: str) -> List[Path]:
    """Get all .mdx files from the components directory."""
    components_path = Path(components_dir)
    if not components_path.is_absolute():
        components_path = get_project_root() / components_path
    return list(components_path.glob('*.mdx'))

def process_docs(
    components_dir: str,
    output_dir: str,
    api_key: str,
    test_mode: bool = False,
    test_count: int = 2
) -> None:
    """Process documentation files and convert them to rule files."""
    # Get all .mdx files
    doc_files = get_doc_files(components_dir)
    
    if not doc_files:
        print(f"No .mdx files found in {components_dir}")
        return
    
    # If in test mode, only process the first N files
    if test_mode:
        doc_files = doc_files[:test_count]
        print(f"Test mode: Processing {len(doc_files)} files")
    
    # Process each file
    for doc_file in doc_files:
        print(f"\nProcessing: {doc_file.name}")
        
        try:
            # Read the documentation file
            with open(doc_file, 'r') as f:
                doc_content = f.read()
            
            # Convert to rule file
            rule_content = convert_doc_to_rule(doc_content, api_key)
            
            # Generate output filename and path
            output_filename = doc_file.stem.replace('_', '-').lower() + '.mdc'
            output_path = get_project_root() / output_dir / output_filename
            
            # Write rule file
            write_file(str(output_path), rule_content)
            print(f"Created: {output_path}")
            
        except Exception as e:
            print(f"Error processing {doc_file.name}: {e}")
            continue

def main():
    parser = argparse.ArgumentParser(description='Convert multiple documentation files to Cursor rule files')
    parser.add_argument('--components-dir', default='docs/handbook/components', 
                      help='Directory containing documentation files')
    parser.add_argument('--output-dir', default='.cursor/rules', 
                      help='Output directory for rule files')
    parser.add_argument('--api-key', help='OpenAI API key (or set OPENAI_API_KEY env var)')
    parser.add_argument('--env-file', default='.env', 
                      help='Path to .env file')
    parser.add_argument('--test', action='store_true',
                      help='Run in test mode (process only first 2 files)')
    parser.add_argument('--test-count', type=int, default=2,
                      help='Number of files to process in test mode')
    args = parser.parse_args()

    # Load environment variables from .env file
    env_path = get_scripts_dir() / args.env_file
    load_dotenv(env_path)

    # Get API key from args, environment, or .env file
    api_key = args.api_key or os.getenv('OPENAI_API_KEY')
    if not api_key:
        print("Error: OpenAI API key not provided. Use --api-key, set OPENAI_API_KEY environment variable, or add it to your .env file.")
        sys.exit(1)

    # Process the documentation files
    process_docs(
        components_dir=args.components_dir,
        output_dir=args.output_dir,
        api_key=api_key,
        test_mode=args.test,
        test_count=args.test_count
    )

if __name__ == '__main__':
    main() 
