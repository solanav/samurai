import telnetlib

ip_list = {
    '192.168.160.128': 5004,
    '192.168.160.128': 5006,
    '192.168.160.128': 5012,
    '192.168.160.128': 5014,
    '192.168.160.128': 5016,
    '192.168.160.128': 5000,
    '192.168.160.128': 5010,
    '192.168.160.128': 5008,
}

for ip, port in ip_list.items():
    print("Connecting to {}:{}".format(ip, port))
    t = telnetlib.Telnet(ip, port)
    msg = ("ls").encode('ascii')
    t.write(msg)
    output = t.read_all()
    print(output)
    t.close()