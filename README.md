# Rewrite It

This repository contains a web application that leverages Ollama to rewrite your code into new languages. You can use any installed ollama models, which will be found by the server and can be selected via a dropdown.

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

To build and run the model without you will need:

- Rust and Cargo which can be installed via Rustup.
- TailwindCSS and DaisyUI which can be installed through Node.
- Ollama and images.

First compile the css using Tailwind with app.css as the input and static/main.css as the output. Build and run the project with `PORT=8080 cargo run`. Ensure a Ollama instance is running on port 11434.

## Design Decisions and Tradeoffs

I decided to design a language rewriter as this seemed like the most interesting challenge. It provided a good focus to develop a clean and useable UI compared to other challenges just providing text output.

I chose to do this as a locally hosted Rust web application as this is something I am familiar with and could progress quickly. The system using DDD and Hexagonal architecture principles to keep the LLM service abstract from the rest of the code.

To give the users a good experience inputing code, CodeMirror was used to create a IDE on the page. This provides syntax highlighting and a natural coding experience for users, and could be extended with multiple plugins. Each language needs its own plugins so I limited my design to three to show the concept which could be expaned upon.

Ollama was used as it is a powerful way to run countless number of LLMs locally. API based LLMs were not used due to time constraints. Using Ollama allows the user to use any number of backends that they may want to install and use.

LLMs work well when they can iterate on their outputs, thus to improve the functionality of the service I added an iterate button which will prompt the LLM to retry with the given user input.

## Improvements

As this task had a time limit of 2 hours, if I had more time I would:

- Add validation to the output. Each language would need its own parser making this a time consuming process to set up.
- Add the ability to use external API LLMs models such as GPT-4o.

## Assumptions

We are presuming that:

- The user has the capabilities to run the models locally. Sub 1B parameter models are suitable for most consumer laptops
- The user only uses the three languages implemented. More could easily be added.
