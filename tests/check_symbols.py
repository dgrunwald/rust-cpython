#!/usr/bin/env python
import re
import sysconfig
import subprocess
import os
import sys
import platform

if os.path.dirname(__file__):
    os.chdir(os.path.dirname(__file__))

if platform.system() in ('Windows', 'Darwin') or platform.system().startswith('CYGWIN'):
    sys.exit(0) # test not supported on windows or osx - ignore it

so_files = [
    sysconfig.get_config_var("LIBDIR")+"/"+sysconfig.get_config_var("LDLIBRARY"),
    sysconfig.get_config_var("LIBPL")+"/"+sysconfig.get_config_var("LDLIBRARY"),
]
so_file = None
for name in so_files:
    if os.path.isfile(name):
        so_file = name
if not so_file:
    print('Could not find %r' % so_files)
    sys.exit(1)

so_symbols = set()
for line in subprocess.check_output(['readelf', '-Ws', so_file]).splitlines():
    if line:
        so_symbols.add(line.decode('utf-8').split()[-1])

assert 'PyList_Type' in so_symbols
assert 'PyList_New' in so_symbols

cargo_cmd = ['cargo', 'rustc']
cfgs = []
if sys.version_info.major == 3:
    cargo_cmd += ['--manifest-path', '../python3-sys/Cargo.toml']
    for i in range(4, sys.version_info.minor+1):
        cfgs += ['--cfg', 'Py_3_{}'.format(i)]
else:
    cargo_cmd += ['--manifest-path', '../python27-sys/Cargo.toml']

interesting_config_flags = [
    "Py_USING_UNICODE",
    "Py_UNICODE_WIDE",
    "WITH_THREAD",
    "Py_DEBUG",
    "Py_REF_DEBUG",
    "Py_TRACE_REFS",
    "COUNT_ALLOCS"
]
for name in interesting_config_flags:
    if sysconfig.get_config_var(name):
        cfgs += ['--cfg', 'py_sys_config="{}"'.format(name)]
interesting_config_values = ['Py_UNICODE_SIZE']
for name in interesting_config_values:
    cfgs += ['--cfg', 'py_sys_config="{}_{}"'.format(name, sysconfig.get_config_var(name))]


def match_braces(text):
    stack = []
    locs = dict()
    for i, c in enumerate(asttree):
        if c == '{':
            stack.append(i)
        elif c == '}':
            try:
                locs[stack.pop()] = i
            except IndexError:
                break
    return locs


foreignsig = 'ForeignMod {'
foreign_sections = []
foreign_symbols = set()

output = subprocess.check_output(cargo_cmd + ['--', '-Z', 'unpretty=ast-tree,expanded'] + cfgs)
asttree = output.decode('ascii')
while asttree:
    idx = asttree.find(foreignsig)
    if idx < 0:
        break
    asttree = asttree[asttree.find(foreignsig):]
    locs = match_braces(asttree)
    if locs:
        endpos = locs[len(foreignsig) - 1] + 1
        foreign_sections.append(asttree[:endpos])
        asttree = asttree[endpos:]

for section in foreign_sections:
    lines = section.split('\n')
    for idx in range(len(lines)):
        line = lines[idx]
        if ('kind: Fn(' in line) or ('kind: Static(' in line):
            foreign_symbols.add(re.sub(r'\s*ident: (.*)#[0-9]*,', r'\1', lines[idx-1]))

assert 'PyList_Type' in foreign_symbols, "Failed getting statics from rustc -Z unpretty=ast-tree,expanded"
assert 'PyList_New' in foreign_symbols, "Failed getting functions from rustc -Z unpretty=ast-tree,expanded"

names = sorted(foreign_symbols - so_symbols)
if names:
    print('Symbols missing in {}:'.format(so_file))
    print('\n'.join(names))
    sys.exit(1)
else:
    print('Symbols in {} OK.'.format(so_file))
