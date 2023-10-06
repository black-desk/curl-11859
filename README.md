# curl-11859

This is an example for reproduce curl/curl#11859.

## compile

```bash
cargo build
```

## run

Start a nginx server on http:://localhost::8000:

```bash
cd nginx
docker run --rm -v ./nginx.conf:/etc/nginx/nginx.conf:ro -v ./data:/data -p 8000:8000 nginx
```

```bash
cargo run
```
