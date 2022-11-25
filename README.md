# {{param "name" (param "github.repo") "What is your project name?" | titlecase}}

<!-- {{if 0}} -->
To create a new repository from this template repository for Rust projects,
using the [GitHub CLI](https://github.com/cli/cli) run:

```bash
gh extension install heaths/gh-template
gh template clone <name> --template heaths/template-rustlang --public
```

The `gh template` command will:

1. Create a new repository with the given `<name>` on GitHub.
2. Copy the `heaths/template-rustlang` files into that repo.
3. Clone the new repository into a directory named `<name>` in the current directory.
4. Apply built-in and passed parameters, or prompt for undefined parameters, to format template files.

This will create a new repo with the given `<name>` in GitHub, copy the
`heaths/template-rustlang` files into that repo, and clone it into a
subdirectory of the current directory named `<name>`.

See [heaths/gh-template](https://github.com/heaths/gh-template) for more information
about this GitHub CLI extension.
<!-- {{end}} -->

## License

Licensed under the [MIT](LICENSE.txt) license.
