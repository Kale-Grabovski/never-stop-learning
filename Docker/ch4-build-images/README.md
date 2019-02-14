## Inject file into image

FROM debian
RUN mkdir -p /opt/libeatmydata
ADD my.tar.gz /opt/libeatmydata/ # only works for local archives
RUN ls -lRt /opt/libeatmydata

```
docker build --no-cache .
```

## Intelligent cache busting

```
FROM ubuntu:16.04
RUN apt-get install -y git and many other packages
ADD https://api.github.com/repos/nodejs/node/commits > /dev/null # Checks if there are new commits since last image build
RUN echo "Built at: $(date)" >> /build_time
RUN git clone https://github.com/nodejs/node
WORKDIR node
RUN make && make install
```

Check timestamp after couple of hours to be sure if it's working

```
docker build -t linux_last_updated .
docker run linux_last_updated cat /build_time
```
