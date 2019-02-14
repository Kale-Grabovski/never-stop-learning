## Terms

Username - Your Docker registry username.
Registry - Registries hold images. A registry is a store you can upload images to or download them from. Registries can be public or private.
Registry host - The host on which the Docker registry runs.
Docker Hub Index - The default public registry hosted at https://hub.docker.com.
Docker registry - The same as a registry host. It appears to be a deprecated term.

## Host-like container

```
docker run -d phusion/baseimage
docker exec -it 432521414 /bin/bash
```

## Run supervisor

```
FROM ubuntu:latest
  ENV DEBIAN_FRONTEND noninteractive
  RUN apt-get update && apt-get install -y python-pip apache2 tomcat7
  RUN pip install supervisor
  RUN mkdir -p /var/lock/apache2
  RUN mkdir -p /var/run/apache2
  RUN mkdir -p /var/log/tomcat
  RUN echo_supervisord_conf > /etc/supervisord.conf
  ADD ./supervisord_add.conf /tmp/supervisord_add.conf
  RUN cat /tmp/supervisord_add.conf >> /etc/supervisord.conf
  RUN rm /tmp/supervisord_add.conf
  CMD ["supervisord","-c","/etc/supervisord.conf"]
```

```
docker build -t supervised .
docker run -p 9000:80 --name supervised supervised
```

## Commit changes

```
docker run -d -p 8000:8000 --name todobug1 dockerinpractice/todoapp
> 071f6a36c23a19801285b82eafc99333c76f63ea0aa0b44902c6bae482a6e036
docker exec -i -t todobug1 /bin/bash
apt-get update && apt-get install vim -y
```

exit container now

```
docker commit todobug1
```

tag it

```
docker tag 071f6a36c23a19801285b82eafc99333c76f63ea0aa0b44902c6bae482a6e036 tag_name
docker run -p 8001:8000 tag_name
```
