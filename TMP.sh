# TODO: this works, but does not fit in a PKG_NAME format that wkg expects
#
#   registry:
#     image: registry:3.0.0
#     container_name: oci-registry
#     # environment:
#       # - OTEL_TRACES_EXPORTER=none
#       # - OTEL_SDK_DISABLED=true
#     ports:
#       - 5000:5000

REGISTRY="127.0.0.1:5000"
PKG_NAME="evmpriceoraclerust"  # Replace with your actual package name
COMPONENT_FILENAME="evm_price_oracle.wasm"  # Replace with your actual component filename
TAG="0.0.1"  # Or any tag you want to use
ARTIFACT_PATH="./compiled/${COMPONENT_FILENAME}"
CONTENT_TYPE="application/octet-stream"

# Calculate the digest
DIGEST=$(sha256sum "$ARTIFACT_PATH" | cut -d ' ' -f 1)
DIGEST_HEADER="sha256:$DIGEST"
SIZE=$(stat -c %s "$ARTIFACT_PATH")
echo "Uploading artifact: $ARTIFACT_PATH"
echo "Size: $SIZE bytes"
echo "Digest: $DIGEST_HEADER"

echo "Initiating blob upload..."
UPLOAD_URL=$(curl -s -X POST \
  -H "Content-Length: 0" \
  "http://$REGISTRY/v2/$PKG_NAME/blobs/uploads/" \
  -v 2>&1 | grep -i "< location:" | cut -d ' ' -f 3 | tr -d '\r')

if [ -z "$UPLOAD_URL" ]; then
  echo "Failed to get upload URL. Make sure the registry is running and the repository exists."
  exit 1
fi

echo "Upload URL: $UPLOAD_URL"

# Step 2: Upload the blob
echo "Uploading blob..."
BLOB_RESPONSE=$(curl -s -X PUT \
  -H "Content-Type: $CONTENT_TYPE" \
  -H "Content-Length: $SIZE" \
  --data-binary "@$ARTIFACT_PATH" \
  "${UPLOAD_URL}&digest=$DIGEST_HEADER")

if [[ $? -ne 0 ]]; then
  echo "Failed to upload blob"
  exit 1
fi

# Step 3: Create a manifest
echo "Creating manifest..."
MANIFEST=$(cat <<EOF
{
  "schemaVersion": 2,
  "mediaType": "application/vnd.oci.image.manifest.v1+json",
  "config": {
    "mediaType": "application/vnd.oci.image.config.v1+json",
    "digest": "$DIGEST_HEADER",
    "size": $SIZE
  },
  "layers": [
    {
      "mediaType": "$CONTENT_TYPE",
      "digest": "$DIGEST_HEADER",
      "size": $SIZE
    }
  ]
}
EOF
)

# Step 4: Push the manifest
echo "Pushing manifest..."
MANIFEST_RESPONSE=$(curl -s -X PUT \
  -H "Content-Type: application/vnd.oci.image.manifest.v1+json" \
  --data-binary "$MANIFEST" \
  "http://$REGISTRY/v2/$PKG_NAME/manifests/$TAG")

if [[ $? -ne 0 ]]; then
  echo "Failed to push manifest"
  exit 1
fi

echo "Successfully pushed $ARTIFACT_PATH to $REGISTRY/$PKG_NAME:$TAG"
