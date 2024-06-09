default:
    @just --list

install:
    @npm i
    @cargo install cargo-watch

tailwind:
    @npx tailwindcss -w -i ./assets/styles.css -o ./public/styles.css

run:
    @cargo watch -x run

format:
    @prettier --write templates/
