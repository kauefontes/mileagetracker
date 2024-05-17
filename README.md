# Mileage Tracker

This is a Rust web server project that uses the Actix framework to handle HTTP requests. The server exposes three endpoints:

1. `GET /`: This endpoint returns a list of vehicles. It uses the Tera template engine to render the HTML page and the SQLite database to fetch the vehicle data.

2. `GET /health`: This is a simple health check endpoint that returns "I'm Alive!!" when accessed.

3. `POST /submit-mileage`: This endpoint accepts form data to record a vehicle's mileage. It prints the form data to the console and returns "submit mileage".

The project uses the following crates:

- `actix_web`: A powerful and flexible web framework that provides a solid foundation for building web applications with Rust.
- `r2d2_sqlite`: An implementation of the r2d2 connection manager for SQLite. It allows the server to manage a pool of connections to the SQLite database.
- `serde`: A library for serializing and deserializing data. It is used here to deserialize the form data received at the `POST /submit-mileage` endpoint.
- `tera`: A template engine inspired by Jinja2 and Django Templates. It is used to render the HTML page returned by the `GET /` endpoint.

In addition, the project uses htmx in the templates. htmx allows you to access AJAX, CSS Transitions, WebSockets and Server Sent Events directly in HTML, without having to write any JavaScript.

The purpose of the project appears to be tracking vehicle mileage. Users can view a list of vehicles and record the mileage for a specific vehicle.