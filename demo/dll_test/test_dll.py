from ctypes import CDLL, WinDLL

# mac
lib = WinDLL("/Users/gaozhe/GolandProjects/learn-rust/demo/dll-demo/target/release/dll.dll")
#
lib.hello()
