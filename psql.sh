#!/bin/zsh
PGPASSWORD=test psql "postgres://unexist@localhost/foo?sslmode=disable"
