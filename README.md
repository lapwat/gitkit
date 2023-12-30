# gitkit

Manage your Git repositories

## Dependencies

- cd
- git

## Installation

```sh
cargo install --path .
```

## Usage

```
Usage: gitkit [OPTIONS] <COMMAND>

Commands:
  add   Clone a git repository
  test  Clone a git test repository
  cd    Generate a cd command to be executed in your shell
  sync  Commit all modifications and push them to remote
  help  Print this message or the help of the given subcommand(s)

Options:
  -u, --user <USER>
          GitHub username [env: USER=lapwat] [default: $USER]
  -d, --directory <DIRECTORY>
          Directory where your repositories are stored [env: DIRECTORY=] [default: ~/projects]
  -t, --tests-directory <TESTS_DIRECTORY>
          Directory where your test repositories are stored [env: TESTS_DIRECTORY=] [default: ~/tests]
  -h, --help
          Print help
  -V, --version
          Print version
```

## `cd` command

This program cannot change your current directory.

The `gitkit cd` command generates a `cd` command to be executed by your shell.

Add this function to your `.bashrc` / `.zshrc`:

```sh
function gkcd () {
    $(gitkit cd $1)
}
```
