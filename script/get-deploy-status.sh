#!/bin/bash

cd `git rev-parse --show-toplevel` || exit

if [ ! -f .env ]; then
    cp .env.example .env
    if [ $? -ne 0 ]; then
        echo "Failed to copy .env.example to .env"
        exit 1
    fi
fi

# Extract DEPLOY_ENV from the file
DEPLOY_ENV=$(grep "^DEPLOY_ENV=" .env | cut -d '=' -f2)

DEPLOY_ENV=$(echo "$DEPLOY_ENV" | tr '[:lower:]' '[:upper:]')

echo "$DEPLOY_ENV"
