# Rewrite It

This repository contains a web application that leverages Ollama to rewrite your code into new languages.

## Running the Application

The easiest way to build and run the application is with `docker-compose` by running:

```sh
docker-compose up --build
```

Please follow [the install instructions](https://docs.docker.com/compose/install/) if you do not already have this installed.

Once the server is up and running, you will need to give the Ollama server some models. This is best acheived whilst the server is running with the command:

```sh
docker exec ollama-serve ollama pull qwen2.5-coder:0.5b
```

This command will install a small (Less than 400mb) and code oriented model for you to use. Please feel free to install any other models you wish to run.



## Design decisions



## Improvements

- Add validation to the output. Each language would need its own parser

## Assumptions

T