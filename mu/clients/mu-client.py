#!/usr/bin/env python3
"""Приклад клієнта для unix socket API μ-daemon.

Надсилає платіжний intent, читає відповідь.

Використання:
  python3 mu-client.py <socket_path> <recipient> <amount_minor> <agent_id>

Приклад:
  python3 mu-client.py /tmp/mu-policy.sock 0x8335... 5000000 agent-1
"""

import socket, struct, sys


def build_frame(recipient: str, amount: int, chain_id: int,
                agent_id: str, nonce: int, ts: int) -> bytes:
    """Збирає кадр у форматі mu-gate (wire.rs)."""
    payload = bytearray()
    payload += struct.pack('>H', 1)                    # version
    payload += struct.pack('B', len(recipient))        # recipient len
    payload += recipient.encode('ascii')
    payload += struct.pack('>Q', amount >> 64)          # amount u128 BE
    payload += struct.pack('>Q', amount & 0xFFFFFFFFFFFFFFFF)
    payload += struct.pack('>Q', chain_id)
    payload += struct.pack('B', len(agent_id))
    payload += agent_id.encode('ascii')
    payload += struct.pack('>Q', nonce)
    payload += struct.pack('>Q', ts)
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

    frame = build_frame(recipient, amount, 8453, agent_id, nonce=1, ts=1000)

    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sock.connect(sock_path)
    sock.sendall(frame)
    resp = sock.recv(1)
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
    code = resp[0]
    if code == 0xFF:
        print('OK: intent accepted')
    else:
        print(f'Denied: {codes.get(code, f"unknown {code:#x}")}')


if __name__ == '__main__':
    main()
