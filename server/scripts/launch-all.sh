#!/usr/bin/env bash

# Colors

START=$(tput setaf 4)
SECTION=$(tput setaf 5)
WARNING=$(tput setaf 3)
SUCCESS=$(tput setaf 2)
RESET=$(tput sgr0)

printf "$START* Running at $(date)$RESET\n"

# Start node_exporter

RUSTY_PATH=$(pwd)
NODE_EXPORTER_PATH=${NODE_EXPORTER_PATH:-../node_exporter}

if [ -f "$NODE_EXPORTER_PATH"/node_exporter ]; then
  (cd "$NODE_EXPORTER_PATH" && "$RUSTY_PATH"/server/scripts/run-node-exporter.sh)
else
  echo "${WARNING}Warning: node_exporter wasn't found in $(pwd)/$NODE_EXPORTER_PATH$RESET"
fi

# Start Grafana stack

printf "${SECTION}* Launching Grafana stack...$RESET\n\n"

# Here we only care about stderr
(cd server/docker && docker compose -f grafana.yaml up --wait -d) > /dev/null

# Update and launch server

printf "\n${SECTION}* Updating and launching server...$RESET\n\n"

cp server/scripts/auto-update.sh /tmp/rusty-auto-update.sh && bash /tmp/rusty-auto-update.sh

# Update and launch non-adhoc plugins

printf "\n${SECTION}* Updating and launching plugins...$RESET\n\n"

PLUGINS_PATH=${PLUGINS_PATH:-../RustyController-plugins}

mkdir -p /var/log/rusty-controller/plugins/ && cd "$PLUGINS_PATH" && git pull && bash run-all.sh

echo -e "\n${SUCCESS}* Finished!$RESET"
echo
