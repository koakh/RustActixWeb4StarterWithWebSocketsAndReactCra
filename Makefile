SHELL := /bin/bash
# default only here for wuick changes
HTTP_SERVER_URI := 0.0.0.0:8543
REACT_APP_PORT_WS := 8080
# RUST_LOG := trace,actix_server=trace,actix_web=trace
# RUST_LOG := debug,actix_server=debug,actix_web=debug
LOG_DEFAULT_LEVEL=DEBUG
LOGFILE_DEFAULT_LEVEL=DEBUG
# used to override defaults
CONFIG_PATH_SSL := ./config/ssl
CERT_FILE_NAME_KEY := key.pem
CERT_FILE_NAME_CERT := cert.pem

build:
	cargo build

startServer:
	@RUST_BACKTRACE=full \
    BIND_ADDR=0.0.0.0:$(REACT_APP_PORT_WS) \
		HTTP_SERVER_URI=$(HTTP_SERVER_URI) \
		CONFIG_PATH_SSL=$(CONFIG_PATH_SSL) \
		CERT_FILE_NAME_KEY=$(CERT_FILE_NAME_KEY) \
		CERT_FILE_NAME_CERT=$(CERT_FILE_NAME_CERT) \
		cargo run -- start-server \
			-i \
			/var/log/zypper.log \
			-f "^.*c3-.*.log$$" \
			-l "(?i)(.*)"

startConfigServer:
	@RUST_BACKTRACE=full \
    BIND_ADDR=0.0.0.0:$(REACT_APP_PORT_WS) \
		HTTP_SERVER_URI=$(HTTP_SERVER_URI) \
		RUST_LOG=$(RUST_LOG) \
		LOG_DEFAULT_LEVEL=$(LOG_DEFAULT_LEVEL) \
		LOGFILE_DEFAULT_LEVEL=$(LOGFILE_DEFAULT_LEVEL) \
    cargo run -- \
			"start-server" \
			"-c" \
			"./config/config.json"

startConfigServerSudo:
	@cargo build && \
		sudo -u c3 \
		RUST_BACKTRACE=full \
    BIND_ADDR=0.0.0.0:$(REACT_APP_PORT_WS) \
		HTTP_SERVER_URI=$(HTTP_SERVER_URI) \
		RUST_LOG=$(RUST_LOG) \
		LOG_DEFAULT_LEVEL=$(LOG_DEFAULT_LEVEL) \
		LOGFILE_DEFAULT_LEVEL=$(LOGFILE_DEFAULT_LEVEL) \
		target/debug/actixweb4-starter \
			"start-server" \
			"-c" \
			"/etc/actixweb4-starter/config.json"

startClient:
	@cd samples/actixweb/websocket && \
		binserve

startReactClient:
	@cd samples/lazy-log && \
		HOST=192.168.1.120 npm run start

# always remove last build to prevent stalled files
deb:
	@rm app/build -r || true \
		&& cargo deb -v

pushDeb:
	@./pushToRemoteRepo.sh $(VERSION)

.PHONY: start_server
