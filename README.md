# {{param "name" (param "github.repo") "What is your project name?" | titlecase}}

<!-- {{if 0}} -->
To create a new repository from this template repository for Rust projects,
using the [GitHub CLI](https://github.com/cli/cli) run:

```bash
gh repo create <name> --template heaths/template-rustlang --public --clone
cd <name>

gh extension install heaths/gh-template
gh template apply
```

This will create a new repo with the given `<name>` in GitHub, copy the
`heaths/template-rustlang` files into that repo, and clone it into a
subdirectory of the current directory named `<name>`.

Using the [heaths/gh-template](https://github.com/heaths/gh-template) extension,
the CLI will format the project template prompting for parameters as needed.
<!-- {{end}} -->

## License

Licensed under the [MIT](LICENSE.txt) license.
