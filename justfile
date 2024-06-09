default:
    @just --list

install:
    @npm i

format:
    @prettier --write templates/
