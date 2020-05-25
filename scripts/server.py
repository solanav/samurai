import socket
from datetime import datetime
from multiprocessing import Process

HOST = '0.0.0.0'
PORT = 9393
MAX_MSG = 512


def handle(con, addr):
    while True:
        data = con.recv(MAX_MSG)
        if len(data) == 0:
            return
        print("[{}] [{}]: {}".format(datetime.now(), addr[0], data.decode()))


def main():
    sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
    sock.bind((HOST, PORT))

    sock.listen(1)

    while True:
        con, addr = sock.accept()
        p = Process(target=handle, args=(con, addr,))
        p.start()


if __name__ == "__main__":
    main()
