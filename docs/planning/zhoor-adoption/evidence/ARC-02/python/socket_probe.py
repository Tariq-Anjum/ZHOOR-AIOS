# NON-PRODUCTION ARC-02 SPIKE: Linux AF_UNIX/SO_PEERCRED probe.
import os
import socket
import struct
import tempfile

with tempfile.TemporaryDirectory(prefix="zhoor-arc02-") as d:
    path = os.path.join(d, "peer.sock")
    server = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    server.bind(path)
    server.listen(1)
    client = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    client.connect(path)
    accepted, _ = server.accept()
    raw = accepted.getsockopt(socket.SOL_SOCKET, socket.SO_PEERCRED, struct.calcsize("3i"))
    pid, uid, gid = struct.unpack("3i", raw)
    print(f"pid={pid} uid={uid} gid={gid} mechanism=socket.getsockopt(SOL_SOCKET,SO_PEERCRED) unsafe_ffi=false")
    client.close(); accepted.close(); server.close()
