
GITHUB_USER := "thesurlydev"
NAME := "test"
PORT := "8080"
STATIC_PORT := "3000"
STATIC_ASSETS_DIR := "assets"
DB_URL := "postgres://postgres:postgres@localhost:5432/postgres"
DESCRIPTION := "foo"
DESTINATION := "../"

# Clean up the generated project
clean:
  rm -rf {{DESTINATION}}{{NAME}}

# Generate a new project from this template
generate: clean
  CARGO_GENERATE_LOG=trace cargo generate \
    -c cargo-generate.toml \
    --path . \
    --name {{ NAME }} \
    -d github_user={{ GITHUB_USER }} \
    -d description={{DESCRIPTION}} \
    -d port={{PORT}} \
    -d db_support=true \
    -d db_url={{DB_URL}} \
    -d static_support=true \
    -d static_assets_port={{STATIC_PORT}} \
    -d static_assets_dir={{STATIC_ASSETS_DIR}} \
    --destination={{DESTINATION}} \
    -o --verbose

# Build the generated project
build: generate
  cd {{DESTINATION}}{{NAME}} && cargo build

# Run the generated project
run: generate
  cd {{DESTINATION}}{{NAME}} && docker compose up --build
