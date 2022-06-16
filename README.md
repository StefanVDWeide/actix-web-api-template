# Actix-Web API Template


## Features
* Minimal Actix-web 4.X App
* Basic Database Functionality Included (Based on PostgreSQL)
* Support for .env and files
* Integration tests
* See TO-DO section for upcomming features

## Application Structure
This API is split up in 4 different modules, each providing it's own distinct functionality. The modules are:

* Users
* Posts
* Errors
* Helpers

The `main.rs` file is the entry point for the application.

Both the `migrations` directory and the `schema.rs` file are related to the database. The directory contains all files descirbing the changes we have made to the data (such as adding a table) during development. The `schema.rs` file is generated based on the database and is used to interact with it in the code.

---

## Installation

### Template and Dependencies

* Clone this repository:

	```
	$ git clone https://github.com/StefanVDWeide/actix-web-api-template.git
	```

### Dependency installations

```bash
cargo build
```

### Setting up a PostgreSQL Database

[Install PostgreSQL on Mac](https://www.robinwieruch.de/postgres-sql-macos-setup/)

`pg_ctl -D /usr/local/var/postgres start`
`pg_ctl -D /usr/local/var/postgres stop`

### Migrations
TODO

## Running the Application

```bash
cargo run
```

---


## Conclusion
TODO

### Todo's and Improvements

- [] Add JWT auth
- [x] Add tests
- [] Add request limiter
- [] Add GitHub actions to run tests
- [] Add GitHub action to check requirements using Dependabot
- [] Add instructions to deploy to a production


---


## Acknowledgements
[https://zsiciarz.github.io/24daysofrust/book/vol2/day17.html](https://zsiciarz.github.io/24daysofrust/book/vol2/day17.html)
