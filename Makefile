release ?=

# $(info release is $(release))

ifdef release
  release_arg :=--release
  target :=release
  extension :=
else
  release_arg :=
  target :=debug
  extension :=debug
endif

# .DEFAULT_GOAL := all
.PHONY: linux web run

linux:
	cargo build $(release_arg)

web:
	cargo build $(release_arg) --target wasm32-unknown-unknown

run: web
	cd web && basic-http-server .

app: linux
	./target/$(target)/particle

all: linux web app

help:
	@echo "usage: make $(prog) [release=1]"