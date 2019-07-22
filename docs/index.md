[![GitHub last commit](https://img.shields.io/github/last-commit/crate-ci/azure-pipelines.svg)](https://github.com/crate-ci/azure-pipelines)
[![Build Status](https://dev.azure.com/crate-ci/crate-ci/_apis/build/status/azure-pipelines?branchName=master)](https://dev.azure.com/crate-ci/crate-ci/_build/latest?definitionId=3&branchName=master)
[![Codecov](https://codecov.io/github/crate-ci/azure-pipelines/coverage.svg?branch=master)](https://codecov.io/gh/crate-ci/azure-pipelines)

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

And now, to quote the French, [allons-y](https://www.lawlessfrench.com/expressions/allons-y/)!

## Quick-start

1. Follow the [setup instructions](setup.md)
2. Create a file `azure-pipelines.yml` in the root of your repository:
   
   ```yaml
   stages:
    - template: azure/stages.yml@templates
   
   resources:
     repositories:
       - repository: templates
         type: github
         name: crate-ci/azure-pipelines
         endpoint: PLACEHOLDER
   ```
   
   Where `PLACEHOLDER` is the service connection name from setup.

## My project is special

The main template this repository provides, `azure/stages.yml`, is
fairly opinionated about how you should run your CI. This doesn't fit
every project. If you have particular needs, it's quite easy to
[mix-and-match](configuration.md) CI components to get exactly what you
want. Or, alternatively, to [write your own additional
stages](custom.md).

## Showing off your new CI

If you want to add a status badge, click "Pipelines" on the left in your
Azure DevOps panel, then your new pipeline, then the vertical tripe dots
top right, the "Status badge". While you're at it, add the badge to your
`Cargo.toml` too:

```toml
[badges]
azure-devops = { project = "AZURE_USER/AZURE_PROJECT", pipeline = "PIPELINE_NAME", build = "FOOBAR" }
```

Where `FOOBAR` is a weird extra parameter determined entirely by your
pipeline. When you have your pipeline open, look for `definitionId` in
the URL, and put the number you see there as `build`. If you don't do
this, your shown status badge will be correct, but it will link to the
wrong pipeline for…
[reasons](https://developercommunity.visualstudio.com/idea/642367/use-pipeline-name-in-status-badge-links.html).
