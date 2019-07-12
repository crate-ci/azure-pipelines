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

**NOTE: Due to a [bug in
Pipelines](https://developercommunity.visualstudio.com/content/problem/633563/multi-stage-pipelines-show-in-progress-github-chec.html#),
multi-stage pipelines like the one provided by this repo leave a bunch
of GitHub checks as "[in progress](https://github.com/jonhoo/rusty-pipes/runs/167214784)".**

And now, to quote the French, [allons-y](https://www.lawlessfrench.com/expressions/allons-y/)!

## First-time setup

Azure _loves_ to try to get you to sign in to GitHub using OAuth,
thereby giving them access to all your public _and private_ repos. This
makes me sad. Here's how you do it "the new way" instead.

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
features". In the resulting box, enable "Multi-stage pipelines".

## Each-time setup

At this point I'll assume you have an Azure Project correctly set up for
your GitHub user or organization (as described above).

Before we continue, there's a fun little step you have to do first. Go
to "Project settings" (bottom left), the "Service connections" (under
"Pipelines"). There should be one thing listed there, and it's your
authenticated connection to GitHub. Note down its name.

Now, create a file `azure-pipelines.yml` in the root of the repository
you want CI for. If you want all the bells and whistles, write:

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

Where `PLACEHOLDER` is the service connection name we found above.
Alternatively, you can mix-and-match templates using

```yaml
stages:
 - stage: check
   displayName: Compilation check
   jobs:
     - template: azure/cargo-check.yml@templates
       parameters:
         name: cargo_check
 - stage: test
   displayName: Test suite
   dependsOn: check
   jobs:
     - template: azure/tests.yml@templates
       parameters:
         minrust: 1.34.0
 - stage: style
   displayName: Style linting
   dependsOn: check
   jobs:
     - template: azure/style.yml@templates
```

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

## Showing off your new CI

If you want to add a status badge, click "Pipelines" on the left,
then your new pipeline, then the vertical tripe dots top right, the
"Status badge". While you're at it, add the badge to your `Cargo.toml`
too:

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

## Minimum Rust version

By default, this repository also checks that your project compiles with
Rust 1.32.0. This version was chosen as it was the first version that
supported the Rust 2018 edition and the "uniform paths" change. This
version may be bumped occasionally, but will always stay at least 4
releases behind the newest release (~6 months). If you wish to test a
_particular_ minimum version, say 1.34.0, add this after
`azure/stages.yml@templates`:

```yaml
parameters:
  minrust: 1.34.0
```

## Code coverage

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
templates `azure/stages.yml` or `azure/coverage.yml`:

```yaml
parameters:
 codecov_token: $(CODECOV_TOKEN_SECRET)
```

If you aren't using `stages.yml`, you can add a code coverage step with

```yaml
 - stage: coverage
   displayName: Code coverage
   jobs:
     - template: azure/coverage.yml@templates
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
