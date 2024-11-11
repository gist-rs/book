# Hello Actix CloudRun

![](/assets/kat.png) <span class="speech-bubble">An example for deploy Rust (`Actix`) on `CloudRun` (revisit 2024/11/11)</span>

> 💡 full source code is on [<i id="git-repository-button" class="fa fa-github"></i> github](https://github.com/katopz/hello-rust-actix-cloudrun)

## References

- Nearly completed but no repos 🤷‍♂️: https://www.youtube.com/watch?v=LRfraoVZDDg
- `Actix` but no CloudRun: https://github.com/kpcyrd/mini-docker-rust/blob/main/Dockerfile
- How to but no `Actix`: https://www.gmosx.ninja/posts/2020/09/21/how-to-deploy-a-rust-service-to-google-cloud-run/

## Run locally

```shell
# build
docker build -t hello-actix -f ./Dockerfile .

# run
docker run --rm --name hello-actix -p 8080:8080 -e "TARGET=foo" hello-actix

# view
open http://localhost:8080
```

## Deploy directly to CloudRun via macOS

```shell
# Ensure we are all set
gcloud auth login

# See the projects list
gcloud projects list

# Your config for CloudRun
export PROJECT_ID=YOUR_PROJECT_ID_SEE_ABOVE
export PROJECT_NUMBER=YOUR_PROJECT_NUMBER_SEE_ABOVE
export SERVICE_NAME=hello-actix

gcloud config set project $PROJECT_ID

# Enable cache https://github.com/GoogleContainerTools/kaniko
gcloud config set run/platform managed
gcloud config set builds/use_kaniko True
gcloud config set builds/kaniko_cache_ttl 24

# Give serviceAccount build permissions
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:service-$PROJECT_NUMBER@gcp-sa-cloudbuild.iam.gserviceaccount.com" \
    --role="roles/cloudbuild.serviceAgent"

# Submit build
gcloud builds submit --tag gcr.io/$PROJECT_ID/$SERVICE_NAME --timeout=30m

# Deploy with environment variables
gcloud run deploy --image gcr.io/$PROJECT_ID/$SERVICE_NAME --set-env-vars TARGET=foo

# Update environment variables (optional for testing)
gcloud run services update $SERVICE_NAME --update-env-vars TARGET=bar
```

## A trap (2024)

> see [Granting a role to the Cloud Build service agent](https://cloud.google.com/build/docs/securing-builds/configure-access-for-cloud-build-service-account#gcloud)

```
ERROR: (gcloud.builds.submit) FAILED_PRECONDITION: invalid bucket "123.cloudbuild-logs.googleusercontent.com"; service account 123-compute@developer.gserviceaccount.com does not have access to the bucket
```

### Solution

```bash
gcloud projects add-iam-policy-binding $PROJECT_ID \
    --member="serviceAccount:service-$PROJECT_NUMBER@gcp-sa-cloudbuild.iam.gserviceaccount.com" \
    --role="roles/cloudbuild.serviceAgent"
```

## CI/CD Options

- Connect to a GitHub repository with Cloud Build: https://cloud.google.com/build/docs/automating-builds/github/connect-repo-github
- Creating a CI/CD environment for serverless containers on Cloud Run with GitHub Actions: https://github.com/GoogleCloudPlatform/community/blob/master/tutorials/cicd-cloud-run-github-actions/index.md
- GitHub Actions best practices for Rust projects: https://www.infinyon.com/blog/2021/04/github-actions-best-practices/#optimizing-rusts-build-speed-with-sccache
