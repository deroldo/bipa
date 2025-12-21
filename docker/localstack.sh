#!/bin/bash

echo "Iniciando criação/atualização de secrets no AWS Secrets Manager com arquivos JSON..."

# Função para criar ou atualizar um secret
create_or_update_secret() {
  local secret_name=$1
  local secret_file=$2

  if aws --endpoint-url=http://localhost:4566 secretsmanager describe-secret \
       --secret-id "$secret_name" \
       --region us-east-1 >/dev/null 2>&1; then
    echo "Secret '$secret_name' já existe. Atualizando..."
    aws --endpoint-url=http://localhost:4566 secretsmanager update-secret \
      --secret-id "$secret_name" \
      --region us-east-1 \
      --secret-string "file://$secret_file"
  else
    echo "Secret '$secret_name' não existe. Criando..."
    aws --endpoint-url=http://localhost:4566 secretsmanager create-secret \
      --name "$secret_name" \
      --region us-east-1 \
      --description "Secret criado a partir de $secret_file" \
      --secret-string "file://$secret_file"
  fi
}

create_or_update_secret "bipa" "/etc/secrets-manager-public.json"
create_or_update_secret "bipa_secrets" "/etc/secrets-manager-sensitive.json"