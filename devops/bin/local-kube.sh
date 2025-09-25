#!/bin/bash

REGISTRY=localhost:32000
RUST_IMAGE="rust:1.83-bookworm"
PROJECT_PREFIX=pretorian41--
PROJECT_NAME=easy-box-service
IMAGE_PREFIX=${REGISTRY}/${PROJECT_NAME}
API_COMPONENT=api
IMAGE_VERSION=$(get_last_build_version $CWD)
BASE_PATH="$(
  cd "$(dirname "$0")/../../" > /dev/null 2>&1 || exit
  pwd -P
)"
HELM_PATH="${BASE_PATH}/devops/helm/${PROJECT_NAME}"
API_DOCKERFILE_PATH="$BASE_PATH/devops/build/$API_COMPONENT/Dockerfile"


get_last_build_version_file() {
  FILE_NAME=".last_build_version"
  CWD="$(cd "$(dirname "${BASH_SOURCE[0]}")" > /dev/null 2>&1 && pwd)"
  LAST_BUILD_VERSION_FILE=$CWD/$FILE_NAME

  touch $LAST_BUILD_VERSION_FILE
  echo $LAST_BUILD_VERSION_FILE
}

get_last_build_version() {
  IMAGE_VERSION=$(< $(get_last_build_version_file))
  echo $IMAGE_VERSION
}

generate_new_build_version() {
  NEW_BUILD_VERSION=$(date +%s)
  save_last_build_version $NEW_BUILD_VERSION

  echo $NEW_BUILD_VERSION
}

save_last_build_version() {
  echo $1 > $(get_last_build_version_file)
}

bin_install() {
  docker run --rm -v "$BASE_PATH":/app -w /app rust:1.83.0-bookworm \
  cargo build --release
}

build_images() {
  docker build --pull -f "$API_DOCKERFILE_PATH" $BASE_PATH -t "$IMAGE_PREFIX/$API_COMPONENT:$IMAGE_VERSION"
  docker push "$IMAGE_PREFIX/$API_COMPONENT:$IMAGE_VERSION"
}

DEBUG_VALUES=""
DRY_RUN=""
[ -d "$BASE_PATH"/project/vendor ] || COMPOSER="true"
while [ "$1" != "" ]; do
  PARAM=$(echo $1 | awk -F= '{print $1}')
  case $PARAM in
  --build-bin)
    BUILD_BIN="true"
    ;;
  --build)
    BUILD="true"
    IMAGE_VERSION=$(generate_new_build_version)
    ;;
  --dep-update)
    helm dependency update "$HELM_PATH"
    ;;
  --debug-values)
    DEBUG_VALUES=--debug
    ;;
  --debug-templates)
    DEBUG_TEMPLATES="true"
    ;;
  --dry-run)
    DRY_RUN="--dry-run"
    ;;
  esac
  shift
done

if [ "$BUILD_BIN" == true ]; then
  bin_install
fi

if [ -z "$IMAGE_VERSION" ]; then
  echo "No last build version found. Generating new version."
  IMAGE_VERSION=$(generate_new_build_version)
  echo "New version is ${IMAGE_VERSION}"
  BUILD="true"
fi

if [ "$BUILD" ]; then
  build_images $IMAGE_VERSION
fi
echo $DEBUG_TEMPLATES

if [ "$DEBUG_TEMPLATES" ]; then
  helm template "$HELM_PATH" \
    --set "service.image.prefix"=$IMAGE_PREFIX \
    --set "service.api.component"=$API_COMPONENT \
    --set "service.image.version"=$IMAGE_VERSION \
    --set "service.volumes[0].hostPath"="${BASE_PATH}/src" \
    -f "$HELM_PATH/values.yaml" \
    -f "$HELM_PATH/values-local.yaml"
fi

  helm upgrade "$PROJECT_PREFIX$PROJECT_NAME" "$HELM_PATH" \
  --set "image.repository=${IMAGE_PREFIX}/${API_COMPONENT}" \
  --set "image.tag=${IMAGE_VERSION}" \
  --set "image.pullPolicy=IfNotPresent" \
  --set "service.volumes[0].hostPath=${BASE_PATH}/target/release" \
  -f "$HELM_PATH/values.yaml" \
  -f "$HELM_PATH/values-local.yaml" \
  --install $DEBUG_VALUES $DRY_RUN