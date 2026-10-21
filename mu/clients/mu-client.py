#!/usr/bin/env python3
"""Приклад клієнта для unix socket API μ-daemon.

Надсилає платіжний intent, читає відповідь.

Використання:
  python3 mu-client.py <socket_path> <recipient> <amount_minor> <agent_id> [resource]

Приклад:
  python3 mu-client.py /tmp/mu-policy.sock 0x8335... 5000000 agent-1
"""

import socket, struct, sys


def build_frame(recipient: str, amount: int, chain_id: int,
                agent_id: str, nonce: int, ts: int,
                resource: str | None = None) -> bytes:
    """Збирає кадр у форматі mu-gate (wire.rs), версія 2 з resource."""
    payload = bytearray()
    payload += struct.pack('>H', 2)                    # version 2
    payload += struct.pack('B', len(recipient))        # recipient len
    payload += recipient.encode('ascii')
    payload += struct.pack('>Q', amount >> 64)          # amount u128 BE
    payload += struct.pack('>Q', amount & 0xFFFFFFFFFFFFFFFF)
    payload += struct.pack('>Q', chain_id)
    payload += struct.pack('B', len(agent_id))
    payload += agent_id.encode('ascii')
    payload += struct.pack('>Q', nonce)
    payload += struct.pack('>Q', ts)
    body = (resource or '').encode('ascii')
    payload += struct.pack('B', len(body))              # resource len (0 = none)
    payload += body
    payload += b'\x00' * 64                             # підпис (тест)
    frame = struct.pack('<I', len(payload)) + payload
    return bytes(frame)


def main():
    if len(sys.argv) < 5:
        print(__doc__)
        sys.exit(1)

    sock_path = sys.argv[1]
    recipient = sys.argv[2]
    amount = int(sys.argv[3])
    agent_id = sys.argv[4]

    resource = sys.argv[5] if len(sys.argv) > 5 else None
    frame = build_frame(recipient, amount, 8453, agent_id, nonce=1, ts=1000, resource=resource)

    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sock.connect(sock_path)
    sock.sendall(frame)
    code = sock.recv(1)
    header = ''
    if code == b'\x20':
        raw_len = sock.recv(2)
        if len(raw_len) == 2:
            n = struct.unpack('<H', raw_len)[0]
            header = sock.recv(n).decode('utf-8', 'replace') if n else ''
    sock.close()

    codes = {
        0x00: 'Parse error',
        0x01: 'Unknown agent',
        0x02: 'Bad signature',
        0x03: 'Stale timestamp',
        0x04: 'Replay nonce',
        0x05: 'Rate limited',
        0x06: 'Busy',
    }
    code = code[0] if code else 0
    if code == 0xFF:
        print('OK: intent accepted')
    elif code == 0x20:
        print(f'OK: settled; header: {header}')
    else:
        print(f'Denied: {codes.get(code, f"unknown {code:#x}")}')


if __name__ == '__main__':
    main()
