# Setting up CICD pipeline on GitHub Actions, and deploy container to Google App Engine

This is a setup process on establishing a Continuous Deployment pipeline using GitHub Actions to facilitate the deployment of a containerized application to GCP.

The configuration is triggered when the GitHub files are updated or a new pull request is initiated.
The steps include:

- Builds a Docker image
- Assigns it the corresponding version number as a tag
- Uploads it to the Google Artifact Registry.
- Populate workflow secrets as environment variables
- Deploy the container to the Google App Engine

## Before starting the Google Cloud SDK should be installed and authenticated:

- Install [g-cloud](https://cloud.google.com/sdk/docs/install-sdk) cli
- Login to g-cloud CLI
  
```bash
gcloud auth login
```

## Create a Workload Identity Federation

- Enable GCP's Identity and Access Management (IAM) API for the chosen Project.
  
```bash
gcloud services enable iamcredentials.googleapis.com --project "${PROJECT_ID}"
```

- To change the default project_id run:
  
```bash
gcloud config set project PROJECT_ID
```

- Setup the workload identity pool env variable in the system, to manage roles in Google IAM service. Any name can be chosen for the the pool

```bash
export WORKLOAD_IDENTITY_POOL=my-pool
```

- Create the workload

```bash
gcloud iam workload-identity-pools create "${WORKLOAD_IDENTITY_POOL}" --project="${PROJECT_ID}" --location="global" --display-name="${WORKLOAD_IDENTITY_POOL}"
```

- Get the unique identifier of that pool
  
```bash
gcloud iam workload-identity-pools describe "${WORKLOAD_IDENTITY_POOL}" --project="${PROJECT_ID}" --location="global" --format="value(name)"
```

- Export the returned value to a new variable
  
```bash
export WORKLOAD_IDENTITY_POOL_ID=pool_id
```

- Now, for GitHub access create a Workload Provider inside the pool, give any name to the provider

```bash
gcloud iam workload-identity-pools providers create-oidc "${WORKLOAD_PROVIDER}" --project="${PROJECT_ID}" --location="global" --workload-identity-pool="${WORKLOAD_IDENTITY_POOL}" --display-name="${WORKLOAD_PROVIDER}" --attribute-mapping="google.subject=assertion.sub,attribute.actor=assertion.actor,attribute.repository=assertion.repository" --issuer-uri="https://token.actions.githubusercontent.com"
```
