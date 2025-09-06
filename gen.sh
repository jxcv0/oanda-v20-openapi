#!/bin/bash

openapi-generator-cli generate \
  -i openapi.yaml \
  -g rust \
  -o . \
  --additional-properties=packageName=oanda-v20-gen,library=reqwest \
  --global-property apis,models \
  # --global-property apiDocs=false,modelDocs=false,apiTests=true,modelTests=true \
  ##--openapi-generator-ignore-list "README.md,git_push.sh,docs/**"

