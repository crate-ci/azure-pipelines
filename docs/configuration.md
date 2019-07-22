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
     minrust: <false | rust version>
```

By default, your pipeline will test against a minimum Rust version to
ensure that users of your crates are not required to run on the latest
stable version. The default minimum version may be bumped occasionally,
but will always stay at least 4 releases behind the newest release (~6
months). If you wish to test a _particular_ minimum version, say 1.34.0,
you would give that version number as the `minrust` parameter to
`azure/stages.yml`. If you wish to disable the MSRV check, set `minrust`
to `false`.

### Benchmark checking

```yaml
stages:
 - template: azure/stages.yml@templates
   parameters:
     benches: <bool>
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
