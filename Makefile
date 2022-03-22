SHELL := /bin/bash
# backend
# default only here for quick changes
HTTP_SERVER_URI := 0.0.0.0:8543
HTTP_SERVER_API_KEY := uOtXEZXYslKyB0n3g3xRmCaaNsAwB5KmgFcy1X7bbcbtS9dhOpKuhZ04Mfr2OKGL
# RUST_LOG := trace,actix_server=trace,actix_web=trace
LOG_LEVEL=DEBUG
LOGFILE_LEVEL=DEBUG
# used to override defaults
CONFIG_PATH_SSL := ./config/ssl
CERT_FILE_NAME_KEY := key.pem
CERT_FILE_NAME_CERT := cert.pem
# frontend
REACT_HOST := localhost
REACT_BROWSER=none
REACT_APP_HOST_WS := $(REACT_HOST)
REACT_APP_PORT_WS := 8543
REACT_APP_PORT := 8545

build:
	cargo build

startServer:
	@RUST_BACKTRACE=full \
    BIND_ADDR=0.0.0.0:$(REACT_APP_PORT_WS) \
		HTTP_SERVER_URI=$(HTTP_SERVER_URI) \
		CONFIG_PATH_SSL=$(CONFIG_PATH_SSL) \
		CERT_FILE_NAME_KEY=$(CERT_FILE_NAME_KEY) \
		CERT_FILE_NAME_CERT=$(CERT_FILE_NAME_CERT) \
		REACT_APP_SHOW_DEBUG_IN_CONSOLE_LOG=true \
		REACT_APP_HTTP_SERVER_API_KEY=$(HTTP_SERVER_API_KEY) \
		cargo run -- start-server \
			-i \
			/var/log/zypper.log \
			-f "^.*c3-.*.log$$" \
			-l "(?i)(.*)"

# prefered way
startConfigServer:
	@RUST_BACKTRACE=full \
    BIND_ADDR=0.0.0.0:$(REACT_APP_PORT_WS) \
		HTTP_SERVER_URI=$(HTTP_SERVER_URI) \
		RUST_LOG=$(RUST_LOG) \
		LOG_LEVEL=$(LOG_LEVEL) \
		LOGFILE_LEVEL=$(LOGFILE_LEVEL) \
		REACT_APP_SHOW_DEBUG_IN_CONSOLE_LOG=true \
		REACT_APP_HTTP_SERVER_API_KEY=$(HTTP_SERVER_API_KEY) \
    cargo run -- \
			"start-server"
# BOF : UNCOMMENT to use config
#			"-c" \
#			"./config/config.json"
# EOF : UNCOMMENT to use config

startConfigServerSudo:
	@cargo build && \
		sudo -u c3 \
		RUST_BACKTRACE=full \
    BIND_ADDR=0.0.0.0:$(REACT_APP_PORT_WS) \
		HTTP_SERVER_URI=$(HTTP_SERVER_URI) \
		RUST_LOG=$(RUST_LOG) \
		LOG_LEVEL=$(LOG_LEVEL) \
		LOGFILE_LEVEL=$(LOGFILE_LEVEL) \
		REACT_APP_HTTP_SERVER_API_KEY=$(HTTP_SERVER_API_KEY) \
		target/debug/actixweb4-starter \
			"start-server"
# BOF : UNCOMMENT to use config
#			"-c" \
#			"/etc/actixweb4-starter/config.json"
# BOF : UNCOMMENT to use config

# used to debug frontend with hotReload
startReactClient:
	@BROWSER=$(REACT_BROWSER) \
	HOST=$(REACT_HOST) \
	HTTPS=true \
	PORT=$(REACT_APP_PORT) \
	REACT_APP_HOST_WS=$(REACT_APP_HOST_WS) \
	REACT_APP_PORT_WS=$(REACT_APP_PORT_WS) \
	PUBLIC_URL="." \
	NODE_TLS_REJECT_UNAUTHORIZED="0" \
	REACT_APP_SHOW_DEBUG_IN_CONSOLE_LOG=true \
	npm start --prefix app

# always remove last build to prevent stalled files
deb:
	@rm app/build -r || true \
		&& cargo deb -v

pushDeb:
	@./pushToRemoteRepo.sh $(VERSION)

buildDockerImage:
	@docker build . -t actixweb4-starter

.PHONY: start_server
