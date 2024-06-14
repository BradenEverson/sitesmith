# SiteSmith

SiteSmith is a Rust CLI tool designed to generate a personal website. It reads project and work experience data from JSON files and uses a template to create an HTML website.

## Features

- Parses JSON data for projects and work experience.
- Utilizes an HTML template to format and generate the website.
- Customizable templates for projects and work entries.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Install SiteSmith using Cargo:

```bash
cargo install sitesmith
```

## Usage

To generate your website, run the `sitesmith` CLI tool with the following arguments:

```bash
sitesmith --projects <projects.json> --work <work.json> --template <template.html> --output <output.html>
```

### Arguments

- `--projects`: Path to the JSON file containing project data.
- `--work`: Path to the JSON file containing work experience data.
- `--template`: Path to the HTML template file.
- `--output`: Path to the output HTML file to be generated.

### Example

```bash
sitesmith --projects projects.json --work work.json --template template.html --output index.html
```

## JSON Data Format

### Projects JSON Format

The projects JSON file should contain an array of project entries. Each entry should have the following structure:

```json
[
    {
        "name": "Project Name",
        "descr": ["Project description item 1", "Project description item 2"],
        "extra": ["<p>Additional project information</p>"]
    },
    ...
]
```

### Work Experience JSON Format

The work JSON file should contain an array of work experience entries. Each entry should have the following structure:

```json
[
    {
        "name": "Job Title",
        "timespan": "Duration",
        "location": "Location",
        "descr": ["Job description item 1", "Job description item 2"],
        "extra": ["<p>Additional job information</p>", "<b>Even more job info!</b>"]
    },
    ...
]
```

## HTML Template

The HTML template should include placeholders for the projects and work experience sections. For example:

```html
<!DOCTYPE html>
<html>
<head>
    <title>My Personal Website</title>
    <link rel="stylesheet" type="text/css" href="style.css">
</head>
<body>
    <header>
        <h1>Welcome to My Personal Website</h1>
    </header>
    <main>
        <section id="projects">
            {projects}
        </section>
        <section id="work">
            {work}
        </section>
    </main>
    <footer>
        <p>© 2024 Your Name</p>
    </footer>
</body>
</html>
```

The placeholders `{projects}` and `{work}` will be replaced with the generated HTML content for the projects and work experience respectively.

## Example

You can see a live example of a website generated using SiteSmith [here](https://bradeneverson.github.io/).

The source code for this example is available on GitHub: [bradeneverson.github.io](https://github.com/BradenEverson/bradeneverson.github.io).


## Contributing

1. Fork the repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes.
4. Commit your changes (`git commit -am 'Add new feature'`).
5. Push to the branch (`git push origin feature-branch`).
6. Create a new Pull Request.

