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

### Running with Docker

Clone the repository and navigate to the root directory of the project. Then run the following command to build the
project:

```shell
$ git clone https://github.com/Moka-Reads/MoKa-Web.git
$ cd MoKa-Web
$ docker build -t moka-web .
$ docker run -p 8000:8000 moka-web
```

This will build the project and run it locally. The website will be available at `http://0.0.0.0:8000`.

## Checklist 

- [X] Homepage 
- [X] Mission Page 
- [X] Licenses Page
- [X] Cheatsheet Home & Items 
- [ ] Articles Home & Items
- [ ] Guides Home & Items
- [ ] Online Courses Redirect

## Improvements

- [ ] Properly handle Unwrap() calls
- [ ] Create Profiling Tests

## Contributing

If you would like to contribute to the project, please read the [CONTRIBUTING.md](CONTRIBUTING.md) file for more
information.
