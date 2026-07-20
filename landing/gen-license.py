#!/usr/bin/env python3
"""Генератор license.key для клієнтів після Stripe-покупки.

Використання:
  # 1. Налаштувати секретний ключ (той самий, що вшитий в бінар):
  export MU_LICENSE_SECRET=<32_bytes_hex>

  # 2. Згенерувати ліцензію для клієнта:
  python3 gen-license.py <customer_hex_id> > license.key

  # 3. Надіслати license.key клієнту email'ом
  # Клієнт кладе файл поруч з mu-daemon і перезапускає.

Формат license.key:
  customer_id (32 bytes) ‖ ed25519_signature (64 bytes) = 96 bytes

Вимагає: pip install pynacl (або використовуйте Rust: cargo run --bin gen-license)
"""

import binascii, hashlib, os, sys


def ed25519_sign(secret_hex: str, data: bytes) -> bytes:
    """Ed25519 підпис через NaCL (або subprocess до Rust)."""
    try:
        import nacl.bindings
        sk = binascii.unhexlify(secret_hex)
        if len(sk) != 32:
            raise ValueError("secret must be 32 bytes (64 hex chars)")
        return nacl.bindings.crypto_sign_ed25519_sk_to_seed(sk), \
               nacl.bindings.crypto_sign(data, sk)[:64]
    except ImportError:
        # fallback: Rust binary
        import subprocess
        seed = binascii.unhexlify(secret_hex)
        # write customer_id to temp, call cargo run
        return seed, b""


def main():
    secret = os.environ.get("MU_LICENSE_SECRET")
    if not secret:
        print("Помилка: встановіть MU_LICENSE_SECRET (64 hex символи)", file=sys.stderr)
        sys.exit(1)

    if len(sys.argv) < 2:
        print(f"Використання: {sys.argv[0]} <customer_32_byte_hex>", file=sys.stderr)
        sys.exit(1)

    customer_hex = sys.argv[1].strip()
    customer_id = binascii.unhexlify(customer_hex)
    if len(customer_id) != 32:
        print("Помилка: customer_id має бути 32 байти (64 hex символи)", file=sys.stderr)
        sys.exit(1)

    # Підпис: sha256(customer_id)
    digest = hashlib.sha256(customer_id).digest()

    # NaCL signing
    try:
        import nacl.bindings
        sk = binascii.unhexlify(secret)
        sig = nacl.bindings.crypto_sign(digest, sk)[:64]
    except ImportError:
        print("Помилка: встановіть `pip install pynacl` або використовуйте Rust", file=sys.stderr)
        sys.exit(1)

    # license.key = customer_id (32) + signature (64) = 96 bytes
    license_key = customer_id + sig
    sys.stdout.buffer.write(license_key)
    print(f"\n✓ license.key згенеровано ({len(license_key)} байт)", file=sys.stderr)


if __name__ == '__main__':
    main()
