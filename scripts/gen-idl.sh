#!/bin/bash

# echo "Copying generated program IDL and types to app/generated directory."

mkdir -p app/generated

name=$(anchor keys list)

result=$(echo "$name" | awk -F ':' '{print $1}')

cp target/types/$result.ts app/generated/$result.ts
printf 'import { Idl } from "@coral-xyz/anchor"\n\nconst IDL: Idl = {\n' >app/generated/idl.ts

tail -n +2 "target/idl/$result.json" >>app/generated/idl.ts
printf '\n\nexport default IDL;' >>app/generated/idl.ts

npx prettier --write app/generated
# npx eslint --fix app/generated
