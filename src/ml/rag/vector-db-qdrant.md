# Vector Database

## Qdrant

⚠️ WIP

## Setup

```
docker run -p 6333:6333 -p 6334:6334 \
    -v $(pwd)/qdrant_storage:/qdrant/storage:z \
    qdrant/qdrant
```

## Forward port to localhost

```
netsh interface portproxy add v4tov4 listenaddress=192.168.1.33 listenport=8080 connectaddress=127.0.0.1 connectport=8080
```

## Examples

- [x] [Qdrant & Text Data](https://drive.google.com/file/d/1atab1iS_I87TVouAh19fKdPhtpUbCu3i)
- [x] [RAG with OpenAI and Qdrant](https://drive.google.com/file/d/1wm4cHwKJ09DsFnYoalJ7R9cyxZBMlI7s)
