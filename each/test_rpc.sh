#!/bin/bash

grpcurl -plaintext -import-path . -proto provoke.proto -d '{"executor": "shell", "command": "hello"}' '[::1]:50051' each_pro
voker.Provoker/Provoke