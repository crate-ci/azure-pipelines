---
title: Re-usable components
layout: docs
---

Sometimes, the main template `azure/stages.yml` is just too far from
what you want your CI to be. Or, maybe you just want to run a few extra
steps, like publishing your documentation to GitHub Pages or making a
new binary release available from tagged pushes to `master`. Do not
fret, we've got you covered there too! The CI scripts are arranged as
re-usable components, so you can re-use only the parts that you need.

To write your own stage, simply add another list item under `stages:`
in your `azure-pipelines.yml` file:


```yaml
stages:
 - stage: foobar
   ...
 - template: azure/stages.yml@templates
 - stage: skynet
   ...

resources:
  repositories:
    - repository: templates
      type: github
      name: crate-ci/azure-pipelines
      ref: refs/heads/v0.2
      endpoint: PLACEHOLDER
```

You can also remove the `azure/stages.yml` entirely if you do not want
to use the standard set of stages and just roll your own. The Azure
DevOps documentation on writing
[stages](https://docs.microsoft.com/en-us/azure/devops/pipelines/process/stages)
and
[jobs](https://docs.microsoft.com/en-us/azure/devops/pipelines/process/phases)
is surprisingly good, so we won't repeat that here. You may still want
to use some of the components that this repository provides though, so
we'll go through those below.

A good idea might be to _start_ by copy-pasting our `azure/stages.yml`
and then starting from there. If you do, you should start by removing
the cruft from all the lines that look like this:

{% raw %}
```yaml
 - stage: ${{ format('{0}check', parameters.prefix) }}
 - stage: ${{ format('{0}test', parameters.prefix) }}
   dependsOn: ${{ format('{0}check', parameters.prefix) }}
```

So that they instead just read

```yaml
 - stage: check
 - stage: test
   dependsOn: check
```

You can also simplify the various ifs like this one

```yaml
${{ if ne(parameters.prefix, '') }}:
  displayName: ${{ format('Test suite ({0})', parameters.prefix) }}
${{ if eq(parameters.prefix, '') }}:
  displayName: Test suite
```
{% endraw %}

by keeping just the last line and removing the double-space indent.


## Task templates

These are [steps that you can
re-use](https://docs.microsoft.com/en-us/azure/devops/pipelines/process/templates#step-re-use)
in your own jobs. That is to say, you can write:

```yaml
jobs:
 - job: myjob
   steps:
     - script: run this cmd
     - template: azure/foobar.yml@templates
       parameters:
         ...
     - script: run this other cmd
```

To re-use the CI steps from the template `azure/foobar.yml`.

### Install Rust

```yaml
template: azure/install-rust.yml@templates
```

Installs Rust and additional components and targets as needed.

#### Parameters

 - `rust`: which Rust version to install (defaults to `stable`)
 - `components`: list of Rustup [components to install](https://rust-lang.github.io/rustup-components-history/) (defaults to none)
 - `targets`: list of [Rust targets to install](https://github.com/rust-lang/rustup.rs/#cross-compilation) (defaults to none)
 - `setup`: list of additional tasks to run after installation (e.g., to install dependencies; defaults to none)

## Job templates

These are [jobs that you can
re-use](https://docs.microsoft.com/en-us/azure/devops/pipelines/process/templates#job-reuse)
wholesale in your own stages. That is to say, you can write:

```yaml
stages:
 - stage: mystage
   displayName: Some custom stage
   jobs:
     - job: my other job
     - template: azure/foobar.yml@templates
       parameters:
         ...
     - job: another job
```

To re-use the CI job template `azure/foobar.yml`.

### Compilation check

```yaml
template: azure/cargo-check.yml@templates
```

Runs `cargo check` with no features, default features, and all features
against all subcrates and targets. You can pass the parameter `benches:
true` to also test benchmarks, `rust: 1.34.0` to test on a particular
Rust version, and `setup: [...]` to run [additional setup
steps](configuration.md#additional-setup-steps).

By default, this template will also run `cargo check --all-features` to
ensure that all of your features compile, even when they are all used
together (cargo features [should be
additive](https://github.com/rust-lang/cargo/issues/4328)). This is
usually what you want, but in _some_ cases you have features that should
only be enabled on particular compiler versions, targets, or platforms.
If that applies to you, you can set the parameter `all_features` to
`false`, and then the `--all-features` check will not be run.

### Tests

```yaml
template: azure/tests.yml@templates
```

Runs `cargo test` on all platforms and across stable, beta, and nightly.
You can pass the parameter `envs: {...}` to pass [environment
variables](configuration.md#environment-variables), and `setup: [...]`
to run [additional setup
steps](configuration.md#additional-setup-steps). You can also pass
`features` and/or `nightly_features` to include additional cargo features
when running the tests. `nightly_features` will only be included on runs
with the nightly compiler. See the [test docs](#test) for details.

### Test

```yaml
template: azure/test.yml@templates
parameters:
  rust: <string> = 'stable'
  cross: <bool> = false
  allow_fail: <bool> = false
  test_ignored: <bool> = false
  single_threaded: <bool> = false
  features: <[string]> = []
  envs:
    NAME: value
  services:
    NAME: container
  setup:
    - task
```

Runs `cargo test` on Linux (or all platforms if `cross: true`) You can
pass the parameter `envs: {...}` to pass [environment
variables](configuration.md#environment-variables), and `setup: [...]`
to run [additional setup
steps](configuration.md#additional-setup-steps). If you pass
`allow_fail: true`, errors during testing will not count as a failure of
the job, which can be useful for things like ignoring failures on
nightly versions of the compiler. To run tests [marked with
`#[ignore]`](https://doc.rust-lang.org/book/ch11-02-running-tests.html#ignoring-some-tests-unless-specifically-requested),
set `test_ignored: true`. To run tests with
[`--test-threads=1`](https://doc.rust-lang.org/book/ch11-02-running-tests.html#running-tests-in-parallel-or-consecutively),
set `single_threaded: true`. To run tests with particular features
enabled, pass `features: [feat1,feat2,subcrate/feat3]`.
To spin up additional [service
containers](https://docs.microsoft.com/en-us/azure/devops/pipelines/process/service-containers?view=azure-devops&tabs=yaml),
pass them in `services` (though note that these will generally only work
on Linux, so `cross: true` likely won't work).

### Style

```yaml
template: azure/style.yml@templates
```

Runs `rustfmt` and `clippy` on stable Rust and on beta with allowed
failures. You can include the parameter `setup: [...]` to run
[additional setup steps](configuration.md#additional-setup-steps).

### Rust formatting check (rustfmt)

```yaml
template: azure/rustfmt.yml@templates
```

Runs `rustfmt` on Rust stable (set with `rust` parameter) optionally
allowing failures (`allow_fail` parameter).

### Rust linter (clippy)

```yaml
template: azure/cargo-clippy.yml@templates
```

Runs `cargo clippy` on Rust stable (set with `rust` parameter)
optionally allowing failures (`allow_fail` parameter). You can also pass
`setup: [...]` to run [additional setup
steps](configuration.md#additional-setup-steps).

### Coverage

```yaml
template: azure/coverage.yml@templates
```

This job will run [`tarpaulin`](https://github.com/xd009642/tarpaulin)
and upload the coverage test results to
[codecov.io](https://codecov.io/). It takes the required parameter
`codecov_token` that includes the codecov.io upload token (see the
[setup instructions](setup.md#code-coverage)). You can also pass the
parameter `envs: {...}` to pass [environment
variables](configuration.md#environment-variables), `services: {...}` to
run additional [service
containers](https://docs.microsoft.com/en-us/azure/devops/pipelines/process/service-containers?view=azure-devops&tabs=yaml),
and `setup: [...]` to run [additional setup
steps](configuration.md#additional-setup-steps).

By default, your pipeline will test against the stable Rust version
bundled with the [tarpaulin Docker
image](https://github.com/xd009642/tarpaulin#docker). If your project
only compiles on nightly, you can use the nightly Docker image
(`latest-nightly`) instead by setting the parameter `nightly` to `true`.
Note that you cannot set a _specific_ nightly version, but are instead
tied to the version that tarpaulin ships. For this reason, nightly
coverage will always be run with failures allowed (yellow CI) to avoid
spurious CI failures.

## A note on git submodules

By default these jobs will all set the [submodule fetch
policy](https://docs.microsoft.com/en-us/azure/devops/pipelines/repos/github#submodules)
to `recursive`, and will thus fetch all your repository's git submodules
recursively. You cannot generally override this behavior when re-using
components from CI. If you must, you should write your own jobs on top
of `install-rust.yml` and `coverage.yml`. `install-rust` will not change
`checkout: self` at all, and `coverage` lets you override it by setting
the parameter `submodules: true` for single-depth checkout, `submodules:
false` for no submodules, or `submodules: manual` with a `checkout:
self` entry in `setup` for complete manual control.
