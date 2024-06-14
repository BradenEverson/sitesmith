# SiteSmith

SiteSmith is a Rust CLI tool designed to generate a personal website. It reads project and work experience data from JSON files and uses a template to create an HTML website.

## Features

- Parses JSON data for projects and work experience.
- Utilizes an HTML template to format and generate the website.
- Customizable templates for projects and work entries.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Install SiteSmith using Cargo:
    ```sh
    cargo install sitesmith
    ```

## Usage

To generate your website, run the `sitesmith` CLI tool with the following arguments:

```sh
sitesmith --projects <projects.json> --work <work.json> --template <template.html> --output <output.html>
```

