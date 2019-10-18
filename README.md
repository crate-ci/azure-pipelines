[![Build Status](https://dev.azure.com/crate-ci/crate-ci/_apis/build/status/azure-pipelines?branchName=master)](https://dev.azure.com/crate-ci/crate-ci/_build/latest?definitionId=3&branchName=master)
[![Codecov](https://codecov.io/github/crate-ci/azure-pipelines/coverage.svg?branch=master)](https://codecov.io/gh/crate-ci/azure-pipelines)
[![Documentation](https://img.shields.io/badge/documentation-here-blue.svg)](https://crate-ci.github.io/azure-pipelines/)

Ah, so you want to set up continuous integration (CI) testing for your
Rust project, and you decided you wanted to use Azure Pipelines for it?
Well, you're in the right place!

Azure Pipelines, like many other CI services, basically requires you to
fully spell out all the steps to your CI. This is very handy if you have
a complex CI pipeline, but is pretty inconvenient if you just want
something that _works_. This project aims to bridge that gap. It also
tries to guide you through how to even get Azure Pipelines set up in the
first place, which can be a daunting thing to get right!

If you're curious what your CI will ultimately look like, go take a look
at [`tracing-timing`'s
CI](https://dev.azure.com/jonhoo/jonhoo/_build/latest?definitionId=1&branchName=master)
for example. By default, it tests on all platforms, checks that your
code compiles with and without any features it may have, and ensures
that your code works with an older Rust version. You can also
mix-and-match these checks if you wish.

To start, go take a look at [the documentation](https://crate-ci.github.io/azure-pipelines/).

## Documentation

You can find the documentation at
https://crate-ci.github.io/azure-pipelines/. It includes a "getting
started" guide, [detailed setup
instructions](https://crate-ci.github.io/azure-pipelines/setup.html),
documentation of [CI configuration
options](https://crate-ci.github.io/azure-pipelines/configuration.html),
and information about how you can [mix and match individual
templates](https://crate-ci.github.io/azure-pipelines/custom.html).

## If you've done this before:

If you've done this before, and just want the standard YAML again for
`azure-pipelines.yml`, here it is:

```yaml
stages:
 - template: azure/stages.yml@templates

resources:
  repositories:
    - repository: templates
      type: github
      name: crate-ci/azure-pipelines
      ref: refs/heads/v0.2
      endpoint: YOU_NEED_TO_SET_THIS
```
