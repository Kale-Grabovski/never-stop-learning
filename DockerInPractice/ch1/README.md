Run interactively

```
docker build .
docker tag 833deaddeaf todoapp
docker run -i -t -p 8000:8000 --name example todoapp
```

Now check http://localhost:8000 to see nodejs app

Check image exists
```
docker images
```

Run in background

```
docker start example
```

Check container running

```
docker ps -a
```

The docker diff subcommand shows you what files have been affected
since the image was instantiated as a container.

```
docker diff example
```
