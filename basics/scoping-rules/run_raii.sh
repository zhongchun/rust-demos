#! /usr/bin/env bash

rustc raii.rs && valgrind ./raii
