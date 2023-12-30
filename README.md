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
  cd    Print the path to git repository
  sync  Commit all modifications and push them to remote
  help  Print this message or the help of the given subcommand(s)

Options:
  -u, --user <USER>
          GitHub username [env: USER=] [default: $USER]
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

The `gitkit cd` command prints the path where the git repository should be on your system.

Add this function to your `.bashrc` / `.zshrc`:

```sh
function gkcd () {
    cd $(gitkit cd $1) 2>/dev/null
    if [ $? -eq 0 ]; then
        return 0
    fi

    cd $(gitkit cd --test $1) 2>/dev/null
}
```
