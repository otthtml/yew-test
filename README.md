# yew-test

Simple project designed to test yew.


## How to run?

Requirements: Build the docker image in .devcontainer. Run the container and exec into it.

### Backend

### Frontend
1. In the .devcontainer, run:
2. Run `cd front && trunk serve`.
3. Open `localhost:8080` on your browser.

#### Troubleshooting
`Blocking waiting for file lock on build directory` -> run `cd front && pkill rls cargo && cargo clean && trunk serve`.

## How to run all tests?
1. In the .devcontainer, run:
2. 