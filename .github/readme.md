# Github Workflow

- Create a service principal with ACR(Azure Container Registry) push and pull roles in Azure.
```sh
# Set the default resource group
az config set defaults.group=Ortege

ACR_NAME="Ortege01"
SERVICE_PRINCIPAL_NAME="ortege_acr_rw"
ACR_REGISTRY_ID=$(az acr show --name $ACR_NAME --query "id" --output tsv)

CR_ENDPOINT=$(az acr show --name $ACR_NAME --query "loginServer" --output tsv)
CR_PASSWORD=$(az ad sp create-for-rbac --name $SERVICE_PRINCIPAL_NAME --scopes $ACR_REGISTRY_ID --role acrpush --query "password" --output tsv)
CR_USERNAME=$(az ad sp list --display-name $SERVICE_PRINCIPAL_NAME --query "[].appId" --output tsv)

echo "Container registry endpoint: $CR_ENDPOINT"
echo "Service principal ID: $CR_USERNAME"
echo "Service principal password: $CR_PASSWORD"
```

- Set repository secrets in Github project settings page using the above values.
```sh
CR_ENDPOINT
CR_USERNAME
CR_PASSWORD
```
