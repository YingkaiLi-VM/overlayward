PROTO_DIR  := proto
GO_OUT     := sdk/go
PY_OUT     := sdk/python
CPP_OUT    := sdk/cpp
C_OUT      := sdk/c
PROTOS     := $(wildcard $(PROTO_DIR)/overlayward/v1/*.proto)

.PHONY: gen-all gen-go gen-python gen-cpp gen-c build

build:
	cargo build

gen-all: gen-go gen-python gen-cpp gen-c

gen-go:
	@mkdir -p $(GO_OUT)
	protoc \
		--go_out=$(GO_OUT) --go_opt=paths=source_relative \
		--go-grpc_out=$(GO_OUT) --go-grpc_opt=paths=source_relative \
		-I$(PROTO_DIR) $(PROTOS)
	@echo "Go SDK generated in $(GO_OUT)"

gen-python:
	@mkdir -p $(PY_OUT)
	python -m grpc_tools.protoc \
		--python_out=$(PY_OUT) \
		--grpc_python_out=$(PY_OUT) \
		--pyi_out=$(PY_OUT) \
		-I$(PROTO_DIR) $(PROTOS)
	@echo "Python SDK generated in $(PY_OUT)"

gen-cpp:
	@mkdir -p $(CPP_OUT)
	protoc \
		--cpp_out=$(CPP_OUT) \
		--grpc_out=$(CPP_OUT) --plugin=protoc-gen-grpc=`which grpc_cpp_plugin` \
		-I$(PROTO_DIR) $(PROTOS)
	@echo "C++ SDK generated in $(CPP_OUT)"

gen-c:
	cargo build -p ow-ffi
	@echo "C header generated in $(C_OUT)/overlayward.h"
