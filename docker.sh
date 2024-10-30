#!/bin/bash

# define the location of .env
ENV_FILE="./auth-service/.env"

# check if .env exists
if ! [[ -f "$ENV_FILE" ]]; then
  echo "Error: .env file not found!"
  exit 1
fi

# Read each line in the .env file
while IFS= read -r line; do
  #Skip empty lines
  if [[ -n "$line" ]] && [[ "$line" != \#* ]]; then
    # split line
    key=$(echo "$line" | cut -d '=' -f1)
    value=$(echo "$line" | cut -d '=' -f2-)

    export "$key=$value"
  fi
done < <(grep -v '^#' "$ENV_FILE")

#run docker compose commands
docker compose build
docker compose up