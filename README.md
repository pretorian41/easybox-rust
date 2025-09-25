## Local development

Create a local values-local.yaml:

```bash
cp devops/helm/easy-box-service/values-local.yaml.dist devops/helm/easy-box-service/values-local.yaml
```


All others:

```bash
bash ./devops/bin/local-kube.sh --build-bin --build
```
