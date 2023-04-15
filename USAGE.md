# Usage

## Generating a new project

If you've cloned the repository, run the following to generate a new project:

```shell
cargo generate --path axum-template --name new-project-name
```

where `axum-template` is the path to this template and `new-project-name` is the name of the new project.

Otherwise, you can generate a new project directly from GitHub:

```shell
cargo generate --git digitalsanctum/axum-template --name new-project-name
```

## Companion Script

A companion script, [scripts/generate.sh](scripts/generate.sh), is provided to make generating a new project easier. 


## References

- [Cargo Generate Documentation](https://cargo-generate.github.io/cargo-generate/)