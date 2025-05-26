# Documentation to Cursor Rules Converter

This directory contains scripts to convert documentation files into Cursor rule files.

## Scripts

## Environment Setup

1. Create a `.env` file in the `scripts` directory:
```bash
# Create the .env file
touch scripts/.env

# Add your OpenAI API key to the file -- https://platform.openai.com/docs/models
echo "OPENAI_API_KEY=your_api_key_here" > scripts/.env
```

2. Install required Python packages:
```bash
pip install -r requirements.txt
```

### Single File Conversion (`doc_to_rule.py`)

Converts a single documentation file to a Cursor rule file.

```bash
python3 doc_to_rule.py <input_file> [options]
```

Options:
- `--output-dir`: Output directory for rule files (default: `.cursor/rules`)
- `--api-key`: OpenAI API key (or set OPENAI_API_KEY env var)
- `--env-file`: Path to .env file (default: `scripts/.env`)

Example:
```bash
python3 scripts/doc_to_rule.py docs/handbook/components/component.mdx
```

### Batch Processing (`batch_doc_to_rule.py`)

Processes multiple documentation files in a directory.

```bash
python3 scripts/batch_doc_to_rule.py [options]
```

Options:
- `--components-dir`: Directory containing documentation files (default: `docs/handbook/components`)
- `--output-dir`: Output directory for rule files (default: `.cursor/rules`)
- `--api-key`: OpenAI API key (or set OPENAI_API_KEY env var)
- `--env-file`: Path to .env file (default: `scripts/.env`)
- `--test`: Run in test mode (process only first 2 files)
- `--test-count`: Number of files to process in test mode (default: 2)

Examples:
```bash
# Process all files
python3 batch_doc_to_rule.py

# Test mode (process only 2 files)
python3 batch_doc_to_rule.py --test

# Custom test count
python3 batch_doc_to_rule.py --test --test-count 3

# Custom input/output directories
python3 batch_doc_to_rule.py --components-dir docs/custom --output-dir .cursor/custom-rules
```



## Output

Generated rule files will be placed in the `.cursor/rules` directory (or custom output directory if specified) with the following naming convention:
- Input: `component.mdx` → Output: `component.mdc`
- Input: `blockchain-interactions.mdx` → Output: `blockchain-interactions.mdc`

## Notes

- The scripts automatically clean up any triple backticks at the start and end of generated files
- Code block formatting within the content is preserved
- Full markdown links are used instead of @ references
- The output is optimized for LLM consumption
