[![Build Status](https://dev.azure.com/jonhoo/rusty-pipes/_apis/build/status/jonhoo.rusty-pipes?branchName=master)](https://dev.azure.com/jonhoo/rusty-pipes/_build/latest?definitionId=1&branchName=master)
<!-- [![Codecov](https://codecov.io/github/jonhoo/rusty-pipes/coverage.svg?branch=master)](https://codecov.io/gh/jonhoo/rusty-pipes) -->

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

Here's another fun step you now have to do. Go to "Project settings"
(bottom left), the "Service connections" (under "Pipelines"). There
should be one thing listed there. Note down its name.

Now, create a file `azure-pipelines.yml` in the root of the repository
you want CI for. If you want all the bells and whistles, write:

```yaml
stages:
 - template: azure/stages.yml@templates

resources:
  repositories:
    - repository: templates
      type: github
      name: jonhoo/rusty-pipes
      endpoint: PLACEHOLDER
```

Where `PLACEHOLDER` is the service connection name we found above.

Once that's all done, it's time to set up the Pipeline in Azure:

 - Go to https://dev.azure.com/
 - Click the appropriate project
 - Click "Pipelines"
 - Click "New Pipeline"
 - Click "Use the classic editor to create a pipeline without YAML.".
   You must do this. If you just click GitHub, you're taken to the OAuth
   authentication page that surrenders all your secrets.
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
