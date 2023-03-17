set dotenv-load
set positional-arguments

run *ARGS:
    cargo run -- "$@"

test *ARGS:
    cargo test -- "$@"

copy_if_updated target src:
    checkexec {{target}} {{src}} -- cp {{src}} {{target}}

build_osa target src:
    checkexec {{target}} {{src}} -- osacompile -o {{target}} {{src}}

build_dir:
    mkdir -p build/

build_scripts: build_dir
    @just build_osa build/list_chrome_tabs.scpt src/applescript/list_chrome_tabs.applescript
    @just build_osa build/activate_chrome_tab.scpt src/applescript/activate_chrome_tab.applescript

build: build_scripts
    cargo build

install:
    #just build_scripts
    #sudo mkdir -p /usr/local/var/open-tab/
    #@sudo just copy_if_updated /usr/local/var/open-tab/list_chrome_tabs.scpt build/list_chrome_tabs.scpt
    #@sudo just copy_if_updated /usr/local/var/open-tab/activate_chrome_tab.scpt build/activate_chrome_tab.scpt

    #SCRIPT_DIR=/usr/local/var/open-tab/ cargo build --release
    #@sudo just copy_if_updated /usr/local/bin/open-tab ${CARGO_TARGET_DIR:-target}/release/open-tab

    cargo build --release

check:
    cargo check