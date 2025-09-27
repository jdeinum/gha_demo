# Rust CI 

To add CI to your project, first run these general commands:

```bash
git remote add ci git@github.com:jdeinum/rust_ci.git # switch to http
git fetch --all
```

Then run the commands for the version you want to use.

## Debian

```bash
git checkout ci/master chef.debian.dockerfile
git checkout ci/master .github/workflows/rust_build_debian_chef.yaml
```

## Alpine

```bash
git checkout ci/master chef.alpine.dockerfile
git checkout ci/master .github/workflows/rust_build_alpine_chef.yaml
```

## Nix 

```bash
git checkout ci/master nix.dockerfile
git checkout ci/master flake.nix
git checkout ci/master .github/workflows/rust_build_nix.yaml
```

> **Warning: I think this one works, but I haven't set up caching for it yet so
> I'd caution using this, as your build times will probably be really high**


## Earth Build (Earthly)

[RIP](https://earthly.dev/blog/shutting-down-earthfiles-cloud/) Earthly. Waiting
until Earth Build is online and then perhaps I will implement this.
