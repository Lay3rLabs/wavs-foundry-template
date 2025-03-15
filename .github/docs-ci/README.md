# Docs CI

Docs CI is a github action tool that converts your markdown files into runnable instances for verification.

## Requirements
* Docs marked with ```` ```bash ```` are auto run
* Docs marked with ```` ```bash docs-ci-ignore ```` are ignored
* Startup commands must be non blocking (run in the background with `&` in the commands)
* Specify a directory to search for docs from (it will go in order of file names to run through) or a specific document
