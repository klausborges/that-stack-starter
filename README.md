# that-stack-starter

just a template to start full-stack rust app quickly with a decent live reload experience

## requirements

- `rust`
- `node`
- `just`

## installing

clone the repo and install the required dependencies

```sh
just install
```

for most things, look at `justfile` or simply run `just` for a list of commands

## running

after installing the dependencies, run both the server and the `tailwind` watch command

```sh
just run
just tailwind
```

## goals

- set up reload for rust without connection loss
- set up live reload for templates
- integrate the basic build tooling for tailwind and vanilla js

and that's it!
