## Mount bash history

`alias dockbash='docker run -e HIST_FILE=/root/.bash_history -v=$HOME/.bash_history:/root/.bash_history`

## Run docker without sudo

`sudo addgroup -a username docker`

## Remove all exited containers

`docker ps -a -q --filter status=exited | xargs --no-run-if-empty docker rm`
