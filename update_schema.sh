#!/bin/bash

for schema_dir in $(find . -name "schema.rs" -path "*/examples/*" -exec dirname {} \; | xargs -I {} dirname {}); do
  # (cd "$schema_dir" && cargo run --example schema)
  (cd "$schema_dir" && cargo schema)
done