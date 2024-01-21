# start development
run-dev:
  cd frontend && trunk serve --open

# build for deployment
build-release:
  cd frontend && trunk clean && trunk build --release

# run locally with shuttle
shuttle-local:
  cargo shuttle run

# deploy to shuttle
shuttle-deploy:
  rm -r dist && cp -r backend/dist/ dist && cargo shuttle project restart && cargo shuttle deploy