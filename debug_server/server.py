import socket

HOST='localhost'
PORT=9393
MAX_MSG=16

sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
sock.bind((HOST, PORT))

sock.listen(1)

while True:
    connection, client_address = sock.accept()
    data = sock.recv(MAX_MSG)