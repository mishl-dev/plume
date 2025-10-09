# Plume

This is a Rust project that provides an API for searching DuckDuckGo. The API is built using the Actix-Web framework and the utoipa library for OpenAPI documentation.

## Installation

To install the project, clone the repository and build the project using Cargo:

```bash
git clone https://github.com/mishl-dev/Plume
cd Plume
cargo build
```

## Usage

To run the API, use the following command:

```bash
cargo run
```

The API will start running on `http://localhost:8080`. You can send a POST request to `/search` with a JSON payload containing the query and the number of pages to retrieve. For example:

```json
{
  "query": "rust",
  "pages": 2
}
```

The API will return a JSON response with the search results. Here's an example response:

```json
{
  "results": [
    {
      "title": "Rust - A safe, concurrent, practical language",
      "link": "https://www.rust-lang.org/",
      "snippet": "Rust is a systems programming language that runs blazingly fast, \\nand prevents segfaults, data races, and more. \\nIt is designed to be memory safe without sacrificing performance...",
      "favicon": "https://www.rust-lang.org/static/images/favicon.ico",
    },
    {
      "title": "Rust - The Rust Programming Language",
      "link": "https://doc.rust-lang.org/book/",
      "snippet": "The Rust Programming Language (often called Rust) is a systems programming language that runs blazingly fast, \\nand promotes safety by design. Rust emphasizes zero-cost abstractions, \\nproviding performance comparable to C and C++...",
      "favicon": "https://doc.rust-lang.org/favicon.ico",
    }
  ]
}
```

## OpenAPI Documentation

The API is documented using OpenAPI 3.0. You can view the documentation at `http://localhost:8080/swagger-ui/`. The documentation includes information about the available endpoints, request and response schemas, and more.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.