# Hello Actix CloudRun

![](/assets/kat.png) <span class="speech-bubble">An example for deploy Rust (`Actix`) on `CloudRun` (revisit 2022)</span>

> 💡 full source code is on [<i id="git-repository-button" class="fa fa-github"></i> github](https://github.com/katopz/hello-rust-actix-cloudrun)

## References

- Nearly completed but no repos 🤷‍♂️: https://www.youtube.com/watch?v=LRfraoVZDDg
- `Actix` but no `CloudRun` 🤷‍♂️: https://github.com/kpcyrd/mini-docker-rust/blob/main/Dockerfile
- How to but no `Actix` 🤷‍♂️: https://www.gmosx.ninja/posts/2020/09/21/how-to-deploy-a-rust-service-to-google-cloud-run/

## Run locally

```bash
# build
docker build -t hello-actix -f ./Dockerfile .

# run
docker run --rm --name hello-actix -p 8080:8080 -e "TARGET=foo" hello-actix
```

## Deploy directly via `gcloud` to `CloudRun`

> 💡 Google Cloud Command Line Interface (gcloud CLI): https://cloud.google.com/cli

```bash
# Your config for CloudRun
export PROJECT_ID=YOUR_PROJECT_ID_GO_HERE_DO_NOT_JUST_COPY_AND_PASTE
export SERVICE_NAME=hello-actix

# Ensure we are all set
gcloud auth login
gcloud config set project $PROJECT_ID

# Enable cache https://github.com/GoogleContainerTools/kaniko
gcloud config set run/platform managed
gcloud config set builds/use_kaniko True
gcloud config set builds/kaniko_cache_ttl 24

# Submit build
gcloud builds submit --tag gcr.io/$PROJECT_ID/$SERVICE_NAME --timeout=30m

# Deploy with environment variables
gcloud run deploy --image gcr.io/$PROJECT_ID/$SERVICE_NAME --set-env-vars TARGET=foo

# Update environment variables (if need)
gcloud run services update $SERVICE_NAME --update-env-vars TARGET=bar
```

## CI/CD Options

- [Connect to a GitHub repository with Cloud Build](https://cloud.google.com/build/docs/automating-builds/github/connect-repo-github)
- [Creating a CI/CD environment for serverless containers on Cloud Run with GitHub Actions](https://github.com/GoogleCloudPlatform/community/blob/master/tutorials/cicd-cloud-run-github-actions/index.md)
- [GitHub Actions best practices for Rust projects](https://www.infinyon.com/blog/2021/04/github-actions-best-practices/#optimizing-rusts-build-speed-with-sccache)
