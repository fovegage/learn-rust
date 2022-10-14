def win_test():
    from ctypes import WinDLL
    # mac
    lib = WinDLL("/Users/gaozhe/GolandProjects/learn-rust/demo/dll-demo/target/release/dll.dll")
    #
    lib.hello()


def mac_test():
    from ctypes import CDLL
    lib = CDLL(r"../dll-demo/target/release/dll.dll")
    lib.hello()
