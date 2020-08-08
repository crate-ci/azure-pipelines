[![Build Status](https://dev.azure.com/crate-ci/crate-ci/_apis/build/status/azure-pipelines?branchName=master)](https://dev.azure.com/crate-ci/crate-ci/_build/latest?definitionId=3&branchName=master)
[![Codecov](https://codecov.io/github/crate-ci/azure-pipelines/coverage.svg?branch=master)](https://codecov.io/gh/crate-ci/azure-pipelines)
[![Documentation](https://img.shields.io/badge/documentation-here-blue.svg)](https://crate-ci.github.io/azure-pipelines/)

Ah, so you want to set up continuous integration (CI) testing for your
Rust project, and you decided you wanted to use Azure Pipelines for it?
Well, you're in the right place!

Azure Pipelines, like many other CI services, basically requires you to
fully spell out all the steps to your CI. This is very handy if you have
a complex CI pipeline, but is pretty inconvenient if you just want
something that _just works_. This project aims to bridge that gap. It
also tries to guide you through how to even get Azure Pipelines set up
in the first place, which can be a daunting thing to get right!

If you're curious what your CI will ultimately look like, go take a look
at [`tracing-timing`'s
CI](https://dev.azure.com/jonhoo/jonhoo/_build/latest?definitionId=1&branchName=master)
for example. By default, it tests on all platforms, checks that your
code compiles with and without any features it may have, and ensures
that your code works with an older Rust version. You can also
mix-and-match these checks if you wish.

The repository provides three main templates:

 - `default.yml`, which is a highly opinionated default CI that you can
   use for most "normal" Rust projects.
 - `nightly-only.yml`, which is a highly opinionated default CI that you
   can use for Rust projects that only support nightly versions of Rust.
 - `install-rust.yml`, a minimal template that just installs Rust and
   has you write out the commands you want CI to run (see
   `default.yml` for inspiration). You can specify a Rust version,
   additional targets, and additional components.

Below are instructions for how to set up your repository with testing
from this repository, for setting up code coverage, and for configuring
various parameters of the default CI template.

---

## If you've done this before:

If you've done this before, and just want the standard YAML again for
`azure-pipelines.yml`, here it is:

```yaml
jobs:
 - template: default.yml@templates

resources:
  repositories:
    - repository: templates
      type: github
      name: crate-ci/azure-pipelines
      ref: refs/heads/v0.4
      endpoint: YOU_NEED_TO_SET_THIS
```

---

## If you're getting something new set up:

Getting Azure Pipelines and its connection to GitHub set up correctly is
not entirely straightforward. This document takes you through _exactly_
the steps you need to do. Stray from these at your own risk.

### Setting up Azure DevOps

Azure _loves_ to try to get you to sign in to GitHub using OAuth,
thereby giving them access to all your public _and private_ repos. This
makes us sad. Here's how you do it "the new way" instead.

First, make sure you have the Azure Pipelines GitHub Application installed:

 - Install the GitHub Azure Pipelines app: https://github.com/apps/azure-pipelines
 - Click "Configure"
 - Click the user or organization you want CI for
 - Towards the bottom, either choose "All repositories" or select the
   repository you want CI for

Then, make sure you have an Azure Project for your GitHub organization:

 - "Create project" over at https://dev.azure.com/
 - Make it public (probably)

Note that Azure associates only _one_ of your projects with a given
organization's GitHub Apps install, so you _cannot_ have multiple Azure
Projects that are linked to different GitHub projects under the same
GitHub user/organization. This is stupid, but such is life.

This template uses [Build
stages](https://docs.microsoft.com/en-us/azure/devops/pipelines/process/stages),
which is a [preview
feature](https://docs.microsoft.com/en-us/azure/devops/project/navigation/preview-features)
of Azure Pipelines. You therefore need to enable support for it. To do
so, click your profile icon in the top-right corner and click "Preview
features". In the drop-down at the top in the panel that appears, choose
"for this organization", then enable "Multi-stage pipelines".

### Adding CI for a GitHub repository

At this point I'll assume you have an Azure Project correctly set up for
your GitHub user or organization (as described above).

Before we continue, there's a fun little step you have to do first. Go
to "Project settings" (bottom left), the "Service connections" (under
"Pipelines"). There should be one thing listed there, and it's your
authenticated connection to GitHub. Note down its name.

Now, create a file `azure-pipelines.yml` in the root of the repository
you want CI for. If you want all the bells and whistles, write:

```yaml
jobs:
 - template: default.yml@templates

resources:
  repositories:
    - repository: templates
      type: github
      name: crate-ci/azure-pipelines
      ref: refs/heads/v0.4
      endpoint: PLACEHOLDER
```

Where `PLACEHOLDER` is the service connection name we found above.
The template also has a number of [configuration
options](configuration.md) with opinionated defaults. If you have a
particularly "weird" project, you can also [mix-and-match individual CI
components](custom.md).

Once that's all committed and pushed, it's time to set up the Pipeline in Azure:

 - Go to https://dev.azure.com/
 - Click the appropriate project
 - Click "Pipelines"
 - Click "New Pipeline"
 - Click "Use the classic editor to create a pipeline without YAML.".
   You must do this ([bug
   report](https://developercommunity.visualstudio.com/content/problem/642369/pipelines-creation-falls-back-to-github-oauth-auth.html)).
   If you just click GitHub, you're taken to the OAuth authentication
   page that surrenders all your secrets.
 - Click "GitHub"
 - Choose your repository using the triple-dot button
 - Click "Continue"
 - Give your pipeline a name -- probably the name of your project
 - Click "Apply" next to "YAML"
 - Under "Agent pool", select "Hosted"
 - Under "YAML file path", select "azure-pipelines.yml"
 - Click "Save & queue" towards the top.
   And then click it again...
 - Click "Save and run" bottom right

*Hopefully* Azure was now happy with your efforts. If it is, you'll be
taken to your new shiny "Pipeline summary" page, and it will show you
your build and tests progress! Congrats, you now have Azure Pipelines
CI! If you instead get a big red box at the top of the "Run pipeline"
box with an error, try to see if you can figure out which part of the
magic incantation you missed. If it all looks right to you, file an
issue!

---

## If you want to add support for code coverage:

This pipeline is also set up to use
[`tarpaulin`](https://github.com/xd009642/tarpaulin) and
[codecov.io](https://codecov.io/) for test coverage reporting. To enable
this, here's what you have to do:

 - Sign up for https://codecov.io/ if you haven't already
 - Log in (again, if you haven't already)
 - Install the [GitHub Codecov Application](https://github.com/marketplace/codecov)
 - Click "Configure" next to "Codecov" [here](https://github.com/settings/installations)
 - Either enable access to "All repositories", or grant permission just
   for the project you want coverage for.
 - Go to https://codecov.io/gh/GITHUB_USER_OR_ORG/PROJECT/settings
 - Copy the "Repository Upload Token"
 - Go to https://dev.azure.com/
 - Click the Azure project for the owner of the project you want coverage for
 - Click "Pipelines"
 - Click the pipeline for the project you want coverage for
 - Click "Edit" top-right
 - Click the vertical triple dots top-right
 - Click "Variables"
 - Click "Add", name it whatever you wish (I use `CODECOV_TOKEN_SECRET`),
   paste in the "Repository Upload Token" from Codecov, and click the
   little padlock to mark the variable as "secret".
 - Click the little arrow next to "Save & queue" near the top
 - Click "Save"
 - Click "Save" again

Note that this gives access to your Codecov API key to anyone with push
access to the repository! [Use it
wisely](https://docs.microsoft.com/en-us/azure/devops/pipelines/process/variables#secret-variables).
Forks of your repository do _not_ have access to secrets by default.

Now just add this to your entries in `azure-pipelines.yml` that use the
templates `default.yml`:

```yaml
parameters:
 codecov_token: $(CODECOV_TOKEN_SECRET)
```

You may also want to give yourself a nice badge! Just go to the Settings
page on codecov.io again and click "Badge" on the left. To add it your
crates.io page, add this to `Cargo.toml`:

```toml
codecov = { repository = "GH_USER/GH_PROJECT", branch = "master", service = "github" }
```

### Code coverage for PRs

**If you [are really
sure](https://docs.microsoft.com/en-us/azure/devops/pipelines/repos/github#validate-contributions-from-forks)**
you want to allow coverage to run for the arbitrary code people may
submit in PRs to see your secrets, here's what you do:

 - Navigate to the project's pipeline
 - Click "Edit" top-right
 - Click the vertical triple-dot top-right, and then "Triggers"
 - Choose your repository under "Pull request validation"
 - Check the box next to "Build pull requests from forks of this repository"
 - Then, check the box next to "Make secrets available to builds of forks"
 - You may also want to check "Require a team member's comment before building a pull request"

**If you instead want to simply skip coverage on pull requests**, do
_not_ check the box next to "Make secrets available to builds of forks".
You should not need to change anything else.

---

## If you want to configure `default.yml`

The main template, `default.yml` comes configured with some
opinionated defaults. It will check your crate against a minimum Rust
version, check your project without features, and with all of them, and
it will run both rustfmt, clippy, and beta/nightly check on your
codebase. If, for whatever reason, you disagree with some of these
choices, or have a project with particular needs, copy-paste `default.yml`
into your `azure-pipelines.yml`, and replace

```yaml
 - template: install-rust.yml
```

with

```yaml
 - template: install-rust.yml@templates
```

Then you can tweak it to your heart's desire!

There are some smaller configuration parameters available for `default.yml`
too. Most of these also apply to `nightly-only.yml`.

### Testing on multiple platforms

```yaml
jobs:
 - template: default.yml@templates
   parameters:
     cross: <bool> = true
```

By default, your pipeline will test on Linux, MacOS, and Windows. To
only test on Linux, set `cross` to `false`.

### Minimum Supported Rust Version (MSRV)

```yaml
jobs:
 - template: default.yml@templates
   parameters:
     minrust: <false | rust version> = 1.32.0
```

By default, your pipeline will test against a minimum Rust version to
ensure that users of your crates are not required to run on the latest
stable version. The default minimum version may be bumped occasionally,
but will always stay at least 4 releases behind the newest release (~6
months). If you wish to test a _particular_ minimum version, say 1.34.0,
you would give that version number as the `minrust` parameter to
`default.yml`. If you wish to disable the MSRV check, set `minrust`
to `false`.

#### Minimum supported nightly

If you are using the `nightly-only.yml` template, the equivalent of MSRV
checking is to check that you support some particular "oldest" nightly.
You can check this by specifying a date for the `min` parameter:

```yaml
jobs:
 - template: nightly-only.yml@templates
   parameters:
     minrust: <false | YYYY-MM-DD> = false
```


### Environment variables

```yaml
jobs:
 - template: default.yml@templates
   parameters:
     env:
       <name>: <var>
```

If you tests require particular environment variables to be set, you can
set these using the `env` parameter. The given environment variables
will be passed in whenever your tests are run. You can set multiple
environment variables, and you can use
[variables](https://docs.microsoft.com/en-us/azure/devops/pipelines/process/variables).

### Additional setup steps

```yaml
jobs:
 - template: default.yml
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
[here](https://docs.microsoft.com/en-us/azure/devops/pipelines/tasks/)).
