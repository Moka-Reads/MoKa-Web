# MoKa Reads Web 

> This project is under the [GPLv2 License](LICENSE)

This is the web application for the MoKa Reads project. This is built using the [Rocket](https://rocket.rs/) web framework and 
uses the [Tera](https://tera.netlify.app/) template engine. The web application is built to provide a way for users to 
interact with the MoKa Reads project. This includes the ability to view articles, cheat sheets, and resources. Keeping the 
website opensource allows for users to contribute fixes and improvements to the website that may occur during 
their use of the website.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)

### Running Locally

Clone the repository and navigate to the root directory of the project. Then run the following command to build the
project:

```shell
$ git clone https://github.com/Moka-Reads/MoKa-Web.git
$ cd MoKa-Web
$ cargo run 
```

This will build the project and run it locally. The website will be available at `http://127.0.0.1:8080`.

### Running with Docker Locally 

Clone the repository and navigate to the root directory of the project. Then run the following command to build the
project:

```shell
$ git clone https://github.com/Moka-Reads/MoKa-Web.git
$ cd MoKa-Web
$ make build_docki
$ make start_container
```

This will build the project and run it locally. The website will be available at `http://0.0.0.0:8000`.

> To stop the container use the command `make stop_container` & to view the container interactively use `make exec_it`


### Pulling Docker Image 

You may choose to use the official docker image from Github packages to run the container, to do so you may run the following commands: 

```shell
$ docker pull ghcr.io/moka-reads/moka-web:beta
```

## Improvements 

- [X] Homepage (Desktop/Mobile)
  - [ ] Install notification for PWA Mobile 
- [ ] Article Home (Mobile) 
  - Scaling issues
- [ ] Awesome Lists (Mobile)
    - Scaling issues with box and pagination 
- [X] Asset Caching instead of `FileServer`


## Contributing

If you would like to contribute to the project, please read the [CONTRIBUTING.md](CONTRIBUTING.md) file for more
information.
