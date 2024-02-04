# Continuous Deployment Setup

**Github**
-
- Setup main branch
- Github Actions -> Configure NodeJs
```yml
# This workflow will do a clean installation of node dependencies, cache/restore them, build the source code and run tests across different versions of node
# For more information see: https://docs.github.com/en/actions/automating-builds-and-tests/building-and-testing-nodejs

name: Continuous Integration

on:
pull_request:
branches: [ "main" ]

jobs:
CI_verification:

runs-on: ubuntu-latest

strategy:
matrix:
    node-version: [14.x]
    # See supported Node.js release schedule at https://nodejs.org/en/about/releases/

steps:
- uses: actions/checkout@v3
- name: Use Node.js ${{ matrix.node-version }}
uses: actions/setup-node@v3
with:
    node-version: ${{ matrix.node-version }}

- name: Test Customer Service
working-directory: ./customer
run: |
    npm ci
    npm test
```
- Set new branch rules
- Continuous Deployment Setup complete for dev environment

**Next For Production Environment**
-
- Go to AWS for setting up QA env
- Identidy Access Management Dashboard
- Users -> Add User
- Set Access Key for Id and Secret Key
- Go to Attach existing policies directly
- Set AdministrationAccess-AWSElasticBeanstalk
- Create User

**Set Elastic Beanstalk Access For Github Actions**
- 
- Set Repository Actions Secrets -> AWS_ACCESS_KEY_ID && AWS_SECRET_ACCESS_KEY
- Create Application in ElasticBeanstalk
- Choose Sample Application and Create Application

- Create a new NodeJs action workflow in Github Actions
- Set env vars and continuous deployment with AWS

- Workflow will upload zip on its own, when a new commit or merge is done.

- Now, setup a new web server environment on Elastic Beanstalk
- 