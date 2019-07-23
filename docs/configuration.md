---
title: CI configuration options
layout: docs
---

The main template, `azure/stages.yml` comes configured with some
opinionated defaults. It will check your crate against a minimum Rust
version, check your project without features, and with all of them, and
it will run both rustfmt, clippy, and beta/nightly check on your
codebase. If, for whatever reason, you disagree with some of these
choices, many of them are configurable directly through [template
parameters](https://docs.microsoft.com/en-us/azure/devops/pipelines/process/templates#passing-parameters).
For particularly tricky corner-cases, you can also [re-use individual CI
components](custom.md).

### Minimum Supported Rust Version (MSRV)

```yaml
stages:
 - template: azure/stages.yml@templates
   parameters:
     minrust: <false | rust version> = 1.32.0
```

By default, your pipeline will test against a minimum Rust version to
ensure that users of your crates are not required to run on the latest
stable version. The default minimum version may be bumped occasionally,
but will always stay at least 4 releases behind the newest release (~6
months). If you wish to test a _particular_ minimum version, say 1.34.0,
you would give that version number as the `minrust` parameter to
`azure/stages.yml`. If you wish to disable the MSRV check, set `minrust`
to `false`.

### Coverage on Rust nightly

```yaml
stages:
 - template: azure/stages.yml@templates
   parameters:
     nightly_coverage: <bool> = false
```

By default, your pipeline will test against the stable Rust version
bundled with the [tarpaulin Docker
image](https://github.com/xd009642/tarpaulin#docker). If your project
only compiles on nightly, you can use the nightly Docker image
(`latest-nightly`) instead by setting this parameter to `true`. Note
that you cannot set a _specific_ nightly version, but are instead tied
to the version that tarpaulin ships. For this reason, nightly coverage
will always be run with failures allowed (yellow CI) to avoid spurious
CI failures.

### Ignored tests

```yaml
stages:
 - template: azure/stages.yml@templates
   parameters:
     test_ignored: <bool> = false
```

Set this parameter to `true` to also run tests [marked with
`#[ignore]`](https://doc.rust-lang.org/book/ch11-02-running-tests.html#ignoring-some-tests-unless-specifically-requested).

### Feature flags

```yaml
stages:
 - template: azure/stages.yml@templates
   parameters:
     test_features: <string> = ''
     nightly_feature: <string> = ''
```

If this parameter is set, it is passed along with `--features` to `cargo
test`. This is useful if you have non-default features that you'd like
to test on CI. You can also set `nightly_feature` which will only be
included when run on nightly, though do note that since nightly tests
are always allowed to fail, you will only see yellow CI if these tests
fail. If you have features like this, you _probably_ also want to
disable [checking all features](#disable-checking-all-features). You can
pass multiple features separated by comma (no spaces).

If you are working within a cargo workspace, you will have to pass
features as `subcrate/feature` as described in [this
issue](https://github.com/rust-lang/cargo/issues/5015). There is not
currently a way to disabling default features for CI tests.

### Single-threaded test execution

```yaml
stages:
 - template: azure/stages.yml@templates
   parameters:
     single_threaded: <bool> = false
```

Some codebases cannot run their test suite in a [multi-threaded
fashion](https://doc.rust-lang.org/book/ch11-02-running-tests.html#running-tests-in-parallel-or-consecutively)
(e.g., because they rely on external stateful tools). If this is the
case for you, set this parameter to `true` and your tests will be run
with `--test-threads=1`.

### Disable checking all features

```yaml
stages:
 - template: azure/stages.yml@templates
   parameters:
     all_features: <bool> = true
```

The default CI setup will run `cargo check --all-features` to ensure
that all of your features compile, even when they are all used together
(cargo features [should be
additive](https://github.com/rust-lang/cargo/issues/4328)). This is
usually what you want, but in _some_ cases you have features that should
only be enabled on particular compiler versions, targets, or platforms.
If you set this parameter to `false`, the `--all-features` check will
not be run.

### Benchmark checking

```yaml
stages:
 - template: azure/stages.yml@templates
   parameters:
     benches: <bool> = false
```

Since the built-in benchmarking harness (`extern crate test`) is [not
yet stable](https://github.com/rust-lang/rust/issues/29553), the CI
configuration does not check benchmarks by default. You can change this
by passing the parameter `benches: true`. Note that this will only
_check_ your benchmarks, not _run_ them!

### Environment variables

```yaml
stages:
 - template: azure/stages.yml@templates
   parameters:
     envs:
       <name>: <var>
```

If you tests require particular environment variables to be set, you can
set these using the `envs` parameter. The given environment variables
will be passed in whenever your tests are run. You can set multiple
environment variables, and you can use
[variables](https://docs.microsoft.com/en-us/azure/devops/pipelines/process/variables):

```yaml
stages:
 - template: azure/stages.yml@templates
   parameters:
     envs:
       ENV_A: foobar
       ENV_B: $(PIPELINE_VAR)
```

### Additional setup steps

```yaml
stages:
 - template: azure/stages.yml
   parameters:
     setup:
       - <steps>
```

Occasionally your project requires additional setup steps for tests to
be run. This may include installing packages, downloading dependencies,
fetch files, or anything else you might think of. To add such extra
steps, use the `setup` parameter and give it a list of
[tasks](https://docs.microsoft.com/en-us/azure/devops/pipelines/process/tasks)
(you can see all of Azure's built-in tasks
[here](https://docs.microsoft.com/en-us/azure/devops/pipelines/tasks/)):

```yaml
stages:
 - template: azure/stages.yml
   parameters:
     setup:
       - script: touch src/setup.rs
       - script: apt install foobar
         condition: not(eq(variables['Agent.OS'], 'Windows_NT'))
```
