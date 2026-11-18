#!/usr/bin/env python3
"""Приклад клієнта для unix socket API μ-daemon.

Надсилає платіжний intent, читає відповідь.

Використання:
  python3 mu-client.py <socket_path> <recipient> <amount_minor> <agent_id> --key <agent.key> [--chain 8453]

Приклад:
  python3 mu-client.py /tmp/mu-daemon.sock 0x5aAe... 5000000 agent-1 --key $MU_HOME/agent.key

Кадр підписується ed25519-ключем агента (agent.key, 64 hex, створює `mu-keygen init`).
Потрібен пакет `cryptography` (pip install cryptography). Ключ тримає ЛИШЕ агент:
демон знає тільки agent.pub.
"""

import socket, struct, sys, time


def load_agent_key(path: str):
    import os, stat
    from cryptography.hazmat.primitives.asymmetric.ed25519 import Ed25519PrivateKey
    if os.stat(path).st_mode & (stat.S_IRWXG | stat.S_IRWXO):
        sys.exit(f'{path}: must be mode 600 (readable by the owner only)')
    seed = bytes.fromhex(open(path).read().strip())
    if len(seed) != 32:
        sys.exit(f'{path}: expected 32 bytes as 64 hex characters')
    return Ed25519PrivateKey.from_private_bytes(seed)


def build_frame(recipient: str, amount: int, chain_id: int,
                agent_id: str, nonce: int, ts: int, key=None) -> bytes:
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
    # підпис ed25519 над усім payload (включно з agent_id); без ключа — нульовий (демон відхилить)
    sig = key.sign(bytes(payload)) if key is not None else b'\x00' * 64
    body = bytes(payload) + sig
    return struct.pack('<I', len(body)) + body


def main():
    args = sys.argv[1:]
    opts = {}
    for name in ('--key', '--chain'):
        if name in args:
            i = args.index(name)
            opts[name] = args[i + 1]
            del args[i:i + 2]
    if len(args) < 4:
        print(__doc__)
        sys.exit(1)

    sock_path, recipient, amount, agent_id = args[0], args[1], int(args[2]), args[3]
    key = load_agent_key(opts['--key']) if '--key' in opts else None

    # nonce must grow on every call and survive restarts: milliseconds since the epoch
    now = time.time()
    frame = build_frame(recipient, amount, int(opts.get('--chain', 8453)), agent_id,
                        nonce=int(now * 1000), ts=int(now), key=key)

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
        0x10: 'Denied: above the ceiling (Ω)',
        0x11: 'Denied: recipient / daily limit / resource (Δ)',
        0x12: 'Denied: owner refused',
        0x13: 'Denied: owner did not answer in time',
        0x20: 'Settled',
        0x30: 'Failed (nothing was paid)',
        0x40: 'Outcome pending — reserve held until reconciled',
    }
    code = resp[0] if resp else 0x00
    print(f'{code:#04x}: {codes.get(code, "unknown")}')

if __name__ == '__main__':
    main()
