# start development
run-dev:
  cd site && trunk serve --open

# build for deployment
build-release:
  cd site && trunk build --release

# run locally with shuttle
shuttle-run:
  cargo shuttle run

# deploy to shuttle
shuttle-deploy:
  cargo shuttle project restart --idle-minutes 0 && cargo shuttle deploy