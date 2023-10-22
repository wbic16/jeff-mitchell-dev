# start development
run-dev:
  cd site && trunk serve --open

# build for deployment
build-release:
  cd site && trunk clean && trunk build --release

# run locally with shuttle
shuttle-run:
  cargo shuttle run

# deploy to shuttle
shuttle-deploy:
  rm -r dist && cp -r server/dist/ dist && cargo shuttle project restart && cargo shuttle deploy