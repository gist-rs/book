# Hello Github Action

![](/assets/kat.png) Here's how we release `Rust` library with built binary via `Github Action` (rn with some caveat).

## Release

```yml
name: Release
on:
  push:
    # tag with `git tag v1.0.0 && git push --tags`
    # 😱 but it's not working due to https://github.com/rust-build/rust-build.action/issues/46
    tags:
      - 'v*.*.*'
  release:
    # Use this for workaround, create release via web gui instead.
    types: [created]

jobs:
  release:
    name: release ${{ matrix.target }}
    runs-on: ubuntu-latest
    strategy:
      fail-fast: false
      matrix:
        include:
          - target: x86_64-unknown-linux-musl
            archive: tar.gz tar.xz tar.zst
          - target: x86_64-pc-windows-gnu
            archive: zip
          - target: x86_64-apple-darwin
            archive: zip
    steps:
      - uses: actions/checkout@master
      - name: Compile and release
        uses: rust-build/rust-build.action@v1.4.3
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
        with:
          RUSTTARGET: ${{ matrix.target }}
          ARCHIVE_TYPES: ${{ matrix.archive }}
```
