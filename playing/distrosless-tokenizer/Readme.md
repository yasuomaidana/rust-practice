# Readme

Here are some instructions to run this code as a distroless image.

```bash
docker build . -t distrosless-local
```

See images
```bash
docker images | grep distrosless-local
```

Run the image exposing ports
```bash
docker run -it -p 8000:8000 distrosless-local
```