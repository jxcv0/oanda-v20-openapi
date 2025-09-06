#!/bin/bash

openapi-generator-cli generate \
  -i openapi.yaml \
  -g rust \
  -o . \
  --additional-properties=packageName=oanda-v20-openapi,library=reqwest \

