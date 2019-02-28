## Mount bash history

`alias dockbash='docker run -e HIST_FILE=/root/.bash_history -v=$HOME/.bash_history:/root/.bash_history`

## Run docker without sudo

`sudo addgroup -a username docker`

## Remove all exited containers

`docker ps -a -q --filter status=exited | xargs --no-run-if-empty docker rm`


## ENTRYPOINT

```
#!/bin/bash 
echo "Cleaning logs over $1 days old" 
find /log_dir -ctime "$1" -name '*log' -exec rm {} \; 
```

Note that the log cleaning takes place on the /log_dir folder. This folder will only 
exist when you mount it at runtime. You may have also noticed that there’s no check 
for whether an argument has been passed in to the script. The reason for this will be 
revealed as we go through the technique. 
Now let’s create a Dockerfile in the same directory to create an image, with the 
script running as the defined command, or entrypoint. 

Dockerfile:

```
FROM ubuntu:17.04 
ADD clean_log /usr/bin/clean_log 
RUN chmod +x /usr/bin/clean_log 
ENTRYPOINT ["/usr/bin/clean_log"] 
CMD ["7"] 
```

TIP You’ll observe that we generally prefer the array form for CMD and 
ENTRYPOINT (for example, CMD ["/usr/bin/command"]) over the shell form 
(CMD /usr/bin/command). This is because the shell form automatically
prepends a /bin/bash -c command to the command you supply, which can 
result in unexpected behavior. Sometimes, however, the shell form is more 
useful (see technique 55). 
The difference between ENTRYPOINT and CMD often confuses people. The key point to 
understand is that an entrypoint will always be run when the image is started, even if a 
command is supplied to the docker run invocation. If you try to supply a command, it 
will add that as an argument to the entrypoint, replacing the default defined in the 
CMD instruction. You can only override the entrypoint if you explicitly pass in an 
--entrypoint flag to the docker run command. This means that running the image 
with a /bin/bash command won’t give you a shell; rather, it will supply /bin/bash as 
an argument to the clean_log script. 
 The fact that a default argument is defined by the CMD instruction means that the 
argument supplied need not be checked. Here’s how you might build and invoke this 
tool: 

```
docker build -t log-cleaner . 
docker run -v /var/log/myapplogs:/log_dir log-cleaner 365
```

If an application within a Docker container appears not to be accessible 
to you from the host machine, despite the port being open, it can be worth 
trying to update the addresses to listen on to 0.0.0.0 in the application config 
file and restarting. It may be that the application is rejecting you because 
you’re not coming from its localhost.

Perl replacing script:

```
perl -p -i -e 's/127\.0\.0\.1/0.0.0.0/g' *
perl -p -i -e 's@/usr/share/www@/var/www/html/@g' /etc/apache2/*
```
