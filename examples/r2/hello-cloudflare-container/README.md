# Containers Starter

[![Deploy to Cloudflare](https://deploy.workers.cloudflare.com/button)](https://deploy.workers.cloudflare.com/?url=https://github.com/cloudflare/templates/tree/main/containers-template)

![Containers Template Preview](https://imagedelivery.net/_yJ02hpOMj_EnGvsU2aygw/5aba1fb7-b937-46fd-fa67-138221082200/public)

<!-- dash-content-start -->

This is a [Container](https://developers.cloudflare.com/containers/) starter template.

It demonstrates basic Container coniguration, launching and routing to individual container, load balancing over multiple container, running basic hooks on container status changes.

<!-- dash-content-end -->

Outside of this repo, you can start a new project with this template using [C3](https://developers.cloudflare.com/pages/get-started/c3/) (the `create-cloudflare` CLI):

```bash
npm create cloudflare@latest -- --template=cloudflare/templates/containers-template
```

## Getting Started

First, run:

```bash
npm install
# or
yarn install
# or
pnpm install
# or
bun install
```

Then run the development server (using the package manager of your choice):

```bash
npm run dev
```

Open [http://localhost:8787](http://localhost:8787) with your browser to see the result.

You can start editing your Worker by modifying `src/index.ts` and you can start
editing your Container by editing the content of `container_src`.

## Deploying To Production

| Command          | Action                                |
| :--------------- | :------------------------------------ |
| `npm run deploy` | Deploy your application to Cloudflare |

## Learn More

To learn more about Containers, take a look at the following resources:

- [Container Documentation](https://developers.cloudflare.com/containers/) - learn about Containers
- [Container Class](https://github.com/cloudflare/containers) - learn about the Container helper class

Your feedback and contributions are welcome!

## Container Size Analysis

### Actual Build Sizes
- **Linux AMD64**: 2.1MB
- **Linux ARM64**: 2.09MB

### Size Optimization Techniques

This Dockerfile demonstrates several optimization strategies to achieve the minimal ~2MB container size:

1. **Multi-stage Build**: Uses a builder stage with Rust toolchain, then copies only the binary to a `scratch` base image
2. **Static Linking**: Compiles with `musl` target to create fully static binaries with no external dependencies
3. **Minimal Base**: Final stage uses `scratch` (empty) image, containing only the compiled binary
4. **Stripped Binary**: Rust's `--release` flag automatically strips debug symbols
5. **Cross-compilation**: Uses `cargo-zigbuild` with Zig for efficient cross-platform compilation

### Size Breakdown
- Final container layers: ~937KB (binary + metadata)
- Total compressed size: 2.1MB (includes Docker layers and metadata)
- Runtime memory footprint: Minimal, as it's a single statically-linked binary

This approach is ideal for:
- Serverless platforms (Cloudflare Workers, AWS Lambda)
- Edge computing deployments
- Fast startup times and low resource usage
- CI/CD pipelines with quick deployment

## Build Commands

### Linux AMD64
```bash
# Build
docker buildx build --platform linux/amd64 -t hello-cloudflare-container-amd64 --load .

# Run
docker run --rm -it -p 8081:8080 hello-cloudflare-container-amd64
```

### Linux ARM64 (MacOS M1/M2)
```bash
# Build
docker buildx build --platform linux/arm64 -t hello-cloudflare-container-arm64 --load .

# Run
docker run --rm -it -p 8081:8080 hello-cloudflare-container-arm64
```
