#!python
import os

## where we find libsilcclient.a, libsilc.a, and other STATIC libraries
#lib_search_path = ['/lib','/usr/lib','/usr/local/lib','/usr/local/silc/lib']

## where we should find things to include
# TODO: Move this into the build config for the arch.
include_search_path =  ['#kernel/arch/armv-7a/include']

## These are our source files
sources = ['kernel.c', 'boot.S']
#test_sources = ['ChatWindow.cpp', 'dbg.cpp', 'FXBroadcast.cpp', 'MainWindow.cpp', 'Server.cpp','#tests/testrunner.cpp']

# update the environment with options from fltk-config
#static_libs = ['/usr/local/lib/libFOX.a']

#### You should not change these.  These are only here
#### If you want to start your own build setup with a
#### different layout than mine.
kernel_base_dir = 'kernel'
source_base_dir = 'src'
build_base_dir = 'build'
target_name = 'kernel'
