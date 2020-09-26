kubectl explain deployment.spec
kubectl create deployment lab5-nginx --image=nginx --dry-run --output=yaml > nginx.yaml
kubectl get pods -o wide
kubectl describe pod verybusy
kubectl exec -it verybusy -c busy /bin/sh

kubectl expose deployment nginx --port=80 --target-port=80
kubectl get svc
kubectl describe svc nginx


Nginx deployment:

```
apiVersion: apps/v1
kind: Deployment
metadata:
  name: nginx
  labels:
    app: nginx
spec:
  replicas: 3
  selector:
    matchLabels:
      app: nginx
  template:
    metadata:
      labels:
        app: nginx
    spec:
      containers:
      - name: nginx
        image: nginx:latest
```

kubectl create -f nginx.yaml
kubectl get all
kubectl edit deployment/nginx
kubectl get all --selector app=nginx
kubectl scale deployment lab5-nginx --replicas=5


Pod yaml example:

```
apiVersion: v1
kind: Pod
metadata:
  name: busybox2
  namespace: default
spec:
  containers:
  - name: busy
    image: busybox
    command:
      - sleep
      - "3600"
```


