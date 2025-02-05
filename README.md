# rust_parse_string

Parse a string and count even, odd, normal chars and special chars

To run the `rust` function you can build the present `Dockerfile` using `docker`.

Do the following steps:

1. Download `rust` image and build the function:

```bash
docker build -t rust_parse_string .
```

2. Run the function the following way:

```bash
docker run --rm -it --name parse_string rust_parse_string:latest
```

The function will parse the string `asdf213987.;;--al::""34asd;` and count the number of even, odd, normal and special characters.
