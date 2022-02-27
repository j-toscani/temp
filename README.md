# Temp

A Program to create and reuse files written in Rust. It puts stuff learned from [The Book](https://doc.rust-lang.org/book/) and the [CLI Recipe](https://rust-cli.github.io/book/index.html) to practice.

## Installation

tbd

## Usage
Commands prepended with a `?` are optional. All paths can either be absolute or relative.

### Using templates
```
$ temp create <template-key> ?<outputpath>

$ temp create vue src/components/Slider.vue
```

Generates `Slider.vue` at `./src/components/` based on the template saved under the key `vue`. If you ommit the `<outpath>` the file will be called `New.vue` and create in the current directory.

### Adding templates
```
$ temp add <template-key> <path-to-template>

$ temp add mjml src/my-mail.mjml
```

Adds a template saved under `mjml` based on the file located at `src/my-mail.mjml`.
### Removing templates
```
$ temp remove <template-key> <path-to-template>

$ temp remove mjml
```

Removes the template saved under `mjml`.

## Progress
- [x] Use Main function to examplify usage

Modularize functionality
- [x] Have one config object that defines actions and variables
- [x] Create one function to call in order to run the program
- [x] Keep one file that keeps track of all existing templates

Create files based on saved templates
- [x] Parse the commandline arguments
- [x] Read file to extract the correct template
- [x] Create a file at the appropriate location based on the chosen template


- [x] Change way to store templates
- [x] Add/Remove Templates to/from file

Add tests to keep current functionality

## Additions
- Add remote templates
- Use a better suited format to store/access templates (e.g. `json, yml ...`)
- Add more properties to templates like `fileEnding` or `variants`
- Make it possible to create multiple templates for one `template-key` with one default template.
- Add more template options e.g. `defaultName`
- Add filetrees to enable templates for projects instead of just files