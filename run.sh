#!/bin/bash
function assert_is_installed {
	local readonly name="$1"
	
	if [[ ! $(command -v ${name}) ]]; then
		log_error "the binary '$name' its required"
		exit 1
	fi
}

function check_dependencies {
	assert_is_installed "cargo"
}

function run {
    echo "Don't forget update the .env"
    cargo run
}

check_dependencies
run

