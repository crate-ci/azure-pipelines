Getting Azure Pipelines and its connection to GitHub set up correctly is
not entirely straightforward. This document takes you through _exactly_
the steps you need to do. Stray from these at your own risk.

## Setting up Azure DevOps

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

## Adding CI for a GitHub repository

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
