# Temp

A Program to create and reuse files written in Rust. It puts stuff learned from [The Book](https://doc.rust-lang.org/book/) and the [CLI Recipe](https://rust-cli.github.io/book/index.html) to practice.

## Installation

tbd

## Usage
Commands prepended with a `?` are optional. All paths can either be absolute or relative.

### Using templates
```
$ temp create <template-key> <outputpath> ?<filename>

$ temp create vue src/components Slider.vue
```

Generates `Slider.vue` at `./src/components/` based on the template saved under the key `vue`. If you ommit the filename the file will be called `New.vue`.

### Adding templates
```
$ temp add <template-key> <path-to-template>

$ temp add mjml src/my-mail.mjml
```

Adds a template saved under `mjml` based on the file located at `src/my-mail.mjml`.

## Progress
- [ ] Use Main function to examplify usage

Modularize functionality
- [ ] Have one config object that defines actions and variables
- [ ] Create one function to call in order to run the program
- [ ] Keep one file that keeps track of all existing templates

Create files based on saved templates
- [ ] Parse the commandline arguments
- [ ] Read file to extract the correct template
- [ ] Create a file at the appropriate location based on the chosen template

Add Templates to file
- [ ] Create an entry in the file with `fileEnding` and `template` saved under the provided `template-key`.