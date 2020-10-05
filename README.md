# cargo-mrepo

Cargo subcommand to help working on multi-repository projects.

## Simple

### Patch

```
cargo mrepo patch <dependency-crate-name or path-to-crate>
```

Clone the dependency crate to local (if necessary) and automatically [`patch the dependency`](https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html) so that the project uses the local clone.

### Unpatch

```
cargo mrepo unpatch <dependency-crate-name or path-to-crate>
```

Stop using the local clone.

## Group

### Start working on multi-repository

```
cargo mrepo group init
```

Create a group. Run this command in a directory which contains multiple Rust projects which can depend on each other.

### Stop working on multi-repository

```
cargo mrepo group clear
```

Clear a group.

### Add a locally cloned crate.

```
cargo mrepo group patch <crate-name or path-to-crate>
```

Insert patches so that all the projects under the group points to the local clone of the specified crate.

### Remove a locally cloned crate.

```
cargo mrepo group unpatch <crate-name or path-to-crate>
```

Remove patches.
