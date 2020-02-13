from ctypes import cdll
from sys import platform

if platform == 'darwin':
    prefix = 'lib'
    ext = 'dylib'
elif platform == 'win32':
    prefix = ''
    ext = 'dll'
else:
    prefix = 'lib'
    ext = 'so'
print('target/debug/{}dummy.{}'.format(prefix, ext))
lib = cdll.LoadLibrary('target/debug/{}dummy.{}'.format(prefix, ext))
dummy = lib.dummy

output = dummy(input)
print('dummy return : {}'.format(output))
