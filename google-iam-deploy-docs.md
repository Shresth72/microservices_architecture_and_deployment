# Setting up CICD pipeline on GitHub Actions, and deploy container to Google App Engine

This is a setup process on establishing a Continuous Deployment pipeline using GitHub Actions to facilitate the deployment of a containerized application to GCP.

The configuration is triggered when the GitHub files are updated or a new pull request is initiated.
The steps include:

- Builds a Docker image
- Assigns it the corresponding version number as a tag
- Uploads it to the Google Artifact Registry.
- Populate workflow secrets as environment variables
- Deploy the container to the Google App Engine

## Before starting the Google Cloud SDK should be installed and authenticated

- Install [g-cloud](https://cloud.google.com/sdk/docs/install-sdk) cli
- Login to g-cloud CLI
  
```bash
gcloud auth login
```

## Create a Workload Identity Federation

- Check all the available configurations with their linked projects, and grab the project_id and service_account you need

```bash
gcloud config configurations list
```

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

#for windows
$Env:TEST = "test"

#to get value
Get-Item Env:\TEST
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

- To list all the providers and delete them

```bash
gcloud iam workload-identity-pools list --location=LOCATION

gcloud iam workload-identity-pools delete "${WORKLOAD_IDENTITY_POOL}" --location=LOCATION
```

- Now, for GitHub access create a Workload Provider inside the pool, give any name to the provider

```bash
gcloud iam workload-identity-pools providers create-oidc "${WORKLOAD_PROVIDER}" --project="${PROJECT_ID}" --location="global" --workload-identity-pool="${WORKLOAD_IDENTITY_POOL}" --display-name="${WORKLOAD_PROVIDER}" --attribute-mapping="google.subject=assertion.sub,attribute.actor=assertion.actor,attribute.repository=assertion.repository" --issuer-uri="https://token.actions.githubusercontent.com"
```

- Allow a GitHub Action based in your repository to login to the service account via the provider.

```bash
export REPO=github-username/repo-name

gcloud iam service-accounts add-iam-policy-binding "${SERVICE_ACCOUNT}@${PROJECT_ID}.iam.gserviceaccount.com" --project="${PROJECT_ID}" --role="roles/iam.workloadIdentityUser" --member="principalSet://iam.googleapis.com/${WORKLOAD_IDENTITY_POOL_ID}/attribute.repository/${REPO}"
```

- Get the unique identifier of the created provider
  
```bash
gcloud iam workload-identity-pools providers describer "${WORKLOAD_PROVIDER}" --project="${PROJECT_ID}" --location="global" --workload-identity-pool="${WORKLOAD_IDENTITY_POOL}" --format="value(name)"
```

- **Save the returned value** and this will be used in the workflow file later on.

## Assign Permissions to the Google Service Account to manage required service APIs

```bash
gcloud projects add-iam-policy-binding $PROJECT_ID --member="serviceAccount:${SERVICE_ACCOUNT}@${PROJECT_ID}.iam.gserviceaccount.com" --role="roles/artifactregistry.admin" --role="roles/iam.serviceAccountUser" --role="roles/cloudsql.admin" --role="roles/appengine.serviceAdmin" --role="roles/firebase.admin" 
```

- Now, you can verify the assigned roles!

```bash
gcloud projects get-iam-policy $PROJECT_ID --flatten="bindings[].members" --format='table(bindings.role)' --filter="bindings.members:${SERVICE_ACCOUNT}@${PROJECT_ID}.iam.gserviceaccount.com"
```

## Create a Repository in Google Artifact Registry

Artifact Registry can be used to manage and store build artifacts (Docker Images, NPM packages) in a scalable and integrated repository service.
Manage repository access with g-cloud IAM and interact with the repositories via g-cloud and cloud console.
Also, integrates Cloud Build and CI/CD systems.
