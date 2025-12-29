GITHUB_USER := "thesurlydev"
NAME := "my-app"
PORT := "8080"
DB_URL := "postgres://postgres:postgres@localhost:5432/postgres"
DESCRIPTION := "This is a sample application built using Axum and PostgreSQL."
DESTINATION := "../"

# Clean up the generated project
clean:
    rm -rf {{ DESTINATION }}{{ NAME }}

# Generate a new project from this template
generate: clean
    CARGO_GENERATE_LOG=trace cargo generate \
      -c cargo-generate.toml \
      --path . \
      --name {{ NAME }} \
      -d github_user={{ GITHUB_USER }} \
      -d description="{{ DESCRIPTION }}" \
      -d port={{ PORT }} \
      -d github_support=true \
      -d db_support=true \
      -d db_url={{ DB_URL }} \
      --destination={{ DESTINATION }} \
      -o --verbose

# Build the generated project
build: generate
    cd {{ DESTINATION }}{{ NAME }} && cargo build

# Run the generated project
run: build
    cd {{ DESTINATION }}{{ NAME }} && cargo run

# E2E test configuration
E2E_CLUSTER := "axum-e2e"
E2E_DB_PORT := "5433"
E2E_APP_PORT := "8081"
E2E_PG_VERSION := "18"
DBCTL := "../dbctl/target/release/dbctl"

# Ensure dbctl is built
e2e-dbctl-build:
    cd ../dbctl && cargo build --release

# Install PostgreSQL version for e2e tests (if needed)
e2e-pg-install: e2e-dbctl-build
    {{ DBCTL }} install {{ E2E_PG_VERSION }} || true

# Start PostgreSQL cluster for e2e tests
e2e-db-start: e2e-pg-install
    @echo "Creating PostgreSQL cluster..."
    {{ DBCTL }} init {{ E2E_CLUSTER }} --version {{ E2E_PG_VERSION }} --port {{ E2E_DB_PORT }} --username postgres || true
    {{ DBCTL }} start {{ E2E_CLUSTER }}
    @echo "PostgreSQL cluster ready"

# Stop and remove PostgreSQL cluster
e2e-db-stop: e2e-dbctl-build
    -{{ DBCTL }} stop {{ E2E_CLUSTER }}
    -{{ DBCTL }} rm {{ E2E_CLUSTER }} --force

# Run e2e tests on generated project
e2e: e2e-db-stop
    #!/usr/bin/env bash
    set -euo pipefail

    E2E_NAME="e2e-test-app"
    E2E_DEST="../"
    E2E_DB_URL="postgres://postgres@localhost:{{ E2E_DB_PORT }}/e2e_test"
    E2E_BASE_URL="http://localhost:{{ E2E_APP_PORT }}"
    DBCTL="{{ DBCTL }}"

    TEMPLATE_DIR="$(pwd)"
    cleanup() {
        echo "Cleaning up..."
        # Kill the server if running
        if [ -n "${SERVER_PID:-}" ] && kill -0 "$SERVER_PID" 2>/dev/null; then
            kill "$SERVER_PID" 2>/dev/null || true
            wait "$SERVER_PID" 2>/dev/null || true
        fi
        # Stop the database cluster
        cd "$TEMPLATE_DIR" && just e2e-db-stop
        # Remove generated project
        rm -rf "${E2E_DEST}${E2E_NAME}"
        echo "Cleanup complete"
    }
    trap cleanup EXIT

    echo "=== E2E Test: Starting ==="

    # 1. Clean up any existing test project
    rm -rf "${E2E_DEST}${E2E_NAME}"

    # 2. Start the database cluster
    echo "=== Starting PostgreSQL cluster ==="
    just e2e-db-start

    # 3. Create the test database
    echo "=== Creating test database ==="
    $DBCTL createdb e2e_test -c {{ E2E_CLUSTER }}

    # 4. Generate the project
    echo "=== Generating project ==="
    CARGO_GENERATE_LOG=trace cargo generate \
        -c cargo-generate.toml \
        --path . \
        --name "$E2E_NAME" \
        -d github_user=testuser \
        -d description="E2E Test Application" \
        -d port={{ E2E_APP_PORT }} \
        -d github_support=false \
        -d db_support=true \
        -d db_url="$E2E_DB_URL" \
        --destination="$E2E_DEST" \
        -o --verbose

    cd "${E2E_DEST}${E2E_NAME}"

    # 5. Run migrations
    echo "=== Running migrations ==="
    sqlx migrate run --database-url "$E2E_DB_URL"

    # 6. Build and run the server
    echo "=== Building project ==="
    cargo build

    echo "=== Starting server ==="
    cargo run &
    SERVER_PID=$!

    # Wait for server to be ready
    echo "Waiting for server to start..."
    for i in {1..30}; do
        if curl -s "$E2E_BASE_URL/health" > /dev/null 2>&1; then
            echo "Server is ready!"
            break
        fi
        if [ $i -eq 30 ]; then
            echo "ERROR: Server failed to start"
            exit 1
        fi
        sleep 1
    done

    echo ""
    echo "=== Testing Endpoints ==="

    # Test 1: Health check
    echo "--- Test: Health check ---"
    HEALTH=$(curl -s "${E2E_BASE_URL}/health")
    echo "Response: $HEALTH"
    echo ""

    # Test 2: Login with seed admin user
    echo "--- Test: Login ---"
    LOGIN_RESP=$(curl -s -X POST "${E2E_BASE_URL}/auth/login" \
        -H "Content-Type: application/json" \
        -d '{"client_id":"admin","client_secret":"test"}')
    echo "Response: $LOGIN_RESP"
    TOKEN=$(echo "$LOGIN_RESP" | jq -r '.data.access_token // empty')
    if [ -z "$TOKEN" ] || [ "$TOKEN" = "null" ]; then
        echo "ERROR: Failed to get access token"
        exit 1
    fi
    echo "Got token: ${TOKEN:0:50}..."
    echo ""

    # Test 3: Get all users (authenticated)
    echo "--- Test: Get all users ---"
    USERS=$(curl -s -X GET "${E2E_BASE_URL}/users" \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer $TOKEN")
    echo "Response: $USERS"
    echo ""

    # Test 4: Get user by ID
    echo "--- Test: Get user by ID ---"
    USER=$(curl -s -X GET "${E2E_BASE_URL}/users/00000000-0000-0000-0000-000000000001" \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer $TOKEN")
    echo "Response: $USER"
    echo ""

    # Test 5: Create a new user
    echo "--- Test: Create user ---"
    NEW_USER=$(curl -s -X POST "${E2E_BASE_URL}/users" \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer $TOKEN" \
        -d '{"username":"testuser","email":"test@example.com"}')
    echo "Response: $NEW_USER"
    NEW_USER_ID=$(echo "$NEW_USER" | jq -r '.data.id // empty')
    if [ -z "$NEW_USER_ID" ] || [ "$NEW_USER_ID" = "null" ]; then
        echo "ERROR: Failed to create user"
        exit 1
    fi
    echo "Created user ID: $NEW_USER_ID"
    echo ""

    # Test 6: Update user
    echo "--- Test: Update user ---"
    UPDATED_USER=$(curl -s -X PUT "${E2E_BASE_URL}/users/${NEW_USER_ID}" \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer $TOKEN" \
        -d '{"username":"updateduser","email":"updated@example.com","modified_by":"admin"}')
    echo "Response: $UPDATED_USER"
    echo ""

    # Test 7: Search users
    echo "--- Test: Search users ---"
    SEARCH_RESULT=$(curl -s -X GET "${E2E_BASE_URL}/users?username=updateduser" \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer $TOKEN")
    echo "Response: $SEARCH_RESULT"
    echo ""

    # Test 8: Delete user
    echo "--- Test: Delete user ---"
    DELETE_RESULT=$(curl -s -X DELETE "${E2E_BASE_URL}/users/${NEW_USER_ID}" \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer $TOKEN")
    echo "Response: $DELETE_RESULT"
    echo ""

    # Test 9: Verify deletion
    echo "--- Test: Verify user deleted ---"
    USERS_AFTER=$(curl -s -X GET "${E2E_BASE_URL}/users" \
        -H "Content-Type: application/json" \
        -H "Authorization: Bearer $TOKEN")
    echo "Response: $USERS_AFTER"
    USER_COUNT=$(echo "$USERS_AFTER" | jq -r '.data | length')
    echo "User count after deletion: $USER_COUNT"
    echo ""

    echo "=== All E2E tests passed! ==="
