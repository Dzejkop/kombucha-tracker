#!/usr/bin/env bash
docker container create --name kombucha-db -e POSTGRES_PASSWORD=pass123 postgres
