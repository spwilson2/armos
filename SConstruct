#!python
import os
import sys
from build_support import *
from build_config import *

#platform = ARGUMENTS.get('OS', Platform())
platform = 'armv7-a'

# setup the build environment variables
env = Environment(PLATFORM = platform,
                  BINDIR = bin,
                  INCDIR = include,
                  LIBDIR = lib,
                  CPPPATH = [include],
                  LIBPATH = [lib],
                  LIBS = 'world')

env.Append(CPATH=include_search_path)

# variables the sub build directories need
Export('env', 'sources', 'static_libs', 'test_sources')


# start the build
target_dir = '#' + SelectBuildDir(build_base_dir)
SConscript(target_dir + os.sep + 'SConscript')
BuildDir(target_dir, source_base_dir, duplicate=0)
Default(target_dir + os.sep + target_name)

# this sets up an alias for test that will compile the unit tests
# into the resulting testrunner program.
env.Alias('test', target_dir + os.sep + 'testrunner')
