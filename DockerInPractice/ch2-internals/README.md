## Port forwarding

```
docker run -d -i -p 1234:1234 --name daemon ubuntu:14.04 nc -l 1234
```

The -d flag, when used with docker run, runs the container as a daemon. The -i flag 
gives this container the ability to interact with your Telnet session. With -p you publish 
the 1234 port from the container to the host.
The --name flag lets you give the con￾tainer a name so you can refer to it later.
Finally, you run a simple listening echo 
server on port 1234 with netcat (nc)

```
telnet localhost 1234
```

then type
```
hello daemon
^]
```

Cleanup: `docker rm daemon`


## Restart container on failure

`docker run -d --restart=on-failure:10 ubuntu /bin/false` 

This command runs the container as a daemon (-d) and sets a limit on the number of 
restart attempts (--restart=on-failure:10), exiting if this is exceeded.
It runs a simple command (/bin/false) that completes quickly and will definitely fail.

Infinite restart:

`docker run -d --restart=always ubuntu echo done`

## New docker images location

`dockerd -g /home/dockeruser/mydocker`

A new set of folders and files will be created in this directory.
These folders are internal to Docker, so play with them at your peril (as we’ve discovered!)


## Proxy docker socket to listening what's going on

`dockerd -g /home/dockeruser/mydocker`

A new set of folders and files will be created in this directory.
These folders are internal to Docker, so play with them at your peril.

## Docker pull

To obtain images from external locations, you can use the docker pull command.
By default, images will be downloaded from the Docker Hub: 
`docker pull tutum/wordpress`

Images will also be retrieved automatically when you try to run them if they’re not 
already present on your machine. 
 To run the first blog, use the following command: 
 
`docker run -d -p 10001:80 --name blog1 tutum/wordpress`

This docker run command runs the container as a daemon (-d) with the publish flag 
(-p). It identifies the host port (10001) to map to the container port (80) and gives 
the container a name to identify it (--name blog1 tutum/wordpress).
