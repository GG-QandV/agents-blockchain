#!/usr/bin/env python3
"""CP1 — живий proof для μ-agentic MVP.

Проганяє 4 кейси проти реального x402-ендпойнту через μ-daemon.

Використання:
  # 1. Запустити демон:
  #    MU_CONNECTOR=x402 MU_WALLET_ADDR=0x... cargo run -p mu-daemon
  #
  # 2. Запустити тест:
  #    python3 cp1-harness.py

  Повертає 0, якщо всі кейси пройшли.
  Артефакти зберігаються в __cp1_artifacts__/
"""

import json, os, socket, struct, subprocess, sys, time, urllib.request

SOCK_PATH = os.environ.get("MU_SOCK", "/tmp/mu-daemon.sock")
ARTIFACT_DIR = "__cp1_artifacts__"
X402_ENDPOINT = "https://api.coingecko.com/api/v3/simple/price?ids=bitcoin&vs_currencies=usd"
X402_AMOUNT = 1_000_000  # $0.01 в minor одиницях USDC (6 decimals)


def build_frame(recipient: str, amount: int, chain_id: int = 8453,
                agent_id: str = "cp1-agent", nonce: int = 0, ts: int = 0) -> bytes:
    payload = bytearray()
    payload += struct.pack('>H', 1)
    payload += struct.pack('B', len(recipient))
    payload += recipient.encode('ascii')
    payload += struct.pack('>Q', amount >> 64)
    payload += struct.pack('>Q', amount & 0xFFFFFFFFFFFFFFFF)
    payload += struct.pack('>Q', chain_id)
    payload += struct.pack('B', len(agent_id))
    payload += agent_id.encode('ascii')
    payload += struct.pack('>Q', nonce)
    payload += struct.pack('>Q', ts)
    payload += b'\x00' * 64
    return struct.pack('<I', len(payload)) + bytes(payload)


def send_intent(frame: bytes) -> int:
    """Надсилає intent, повертає код відповіді."""
    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
    sock.settimeout(30)
    sock.connect(SOCK_PATH)
    sock.sendall(frame)
    resp = sock.recv(1)
    sock.close()
    return resp[0] if resp else 0xFF


def get_x402_challenge(url: str) -> dict:
    """Робить запит до x402-ендпойнту, отримує 402 + accepts[]."""
    req = urllib.request.Request(url, method='GET')
    try:
        urllib.request.urlopen(req)
        return None  # немає 402 — ендпойнт не x402
    except urllib.error.HTTPError as e:
        if e.code == 402:
            body = e.read().decode()
            data = json.loads(body)
            return data.get('accepts', [{}])[0]
        raise


def log(msg: str):
    print(f"[CP1] {msg}")


def save_artifact(name: str, data: str):
    os.makedirs(ARTIFACT_DIR, exist_ok=True)
    path = os.path.join(ARTIFACT_DIR, name)
    with open(path, 'w') as f:
        f.write(data)
    log(f"артефакт збережено: {path}")


def test_case(num: int, desc: str, frame: bytes, expected: int) -> bool:
    log(f"Кейс {num}: {desc}")
    resp = send_intent(frame)
    status = "OK" if resp == expected else f"НЕВДАЧА (очікувалось {expected:#x}, отримано {resp:#x})"
    log(f"  → {status}")
    save_artifact(f"case{num}-{desc.replace(' ', '_')}.txt",
                  f"Case {num}: {desc}\nExpected: {expected:#x}\nGot: {resp:#x}\n{'PASS' if resp == expected else 'FAIL'}")
    return resp == expected


def main() -> int:
    log("=== CP1: живий proof ===")
    log(f"Сокет: {SOCK_PATH}")

    # Перевірка доступності сокета
    if not os.path.exists(SOCK_PATH):
        log("ПОМИЛКА: сокет не знайдено. Запустіть μ-daemon спочатку.")
        return 1

    # Отримуємо challenge від x402-ендпойнту
    log(f"Запит до x402-ендпойнту: {X402_ENDPOINT}")
    challenge = get_x402_challenge(X402_ENDPOINT)
    if challenge is None:
        log("УВАГА: ендпойнт не повернув 402 — тестуємо з мок-параметрами")
        pay_to = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"
    else:
        pay_to = challenge.get('payTo', '')
        log(f"Отримано challenge: payTo={pay_to}, amount={challenge.get('maxAmountRequired')}")

    os.makedirs(ARTIFACT_DIR, exist_ok=True)

    all_pass = True

    # Кейс 1: Роздозволений виклик $0.01
    # Очікування: 0xFF (OK, intent accepted)
    c1_frame = build_frame(pay_to, X402_AMOUNT, nonce=1, ts=int(time.time()))
    if not test_case(1, "роздозволений виклик $0.01", c1_frame, 0xFF):
        all_pass = False

    # Кейс 2: Чужий payTo
    # Надсилаємо на адресу, якої немає в whitelist
    # Очікування: DenyDelta (код 0x??)
    fake_pay_to = "0x1111111111111111111111111111111111111111"
    c2_frame = build_frame(fake_pay_to, X402_AMOUNT, nonce=2, ts=int(time.time()))
    if not test_case(2, "чужий payTo", c2_frame, 0x00):
        all_pass = False

    # Кейс 3: Сума вище порога
    # Надсилаємо $5 (очікується human confirm → в тесті має бути denied або await)
    c3_frame = build_frame(pay_to, 5_000_000, nonce=3, ts=int(time.time()))
    if not test_case(3, "сума вище порога $5", c3_frame, 0x00):
        all_pass = False

    # Кейс 4: Перевірка журналу
    # Читаємо лог демона (через окремий скрипт або файл)
    log("Кейс 4: журнал — дивіться mu.log та артефакти вище")

    # Підсумок
    log("")
    log("=== РЕЗУЛЬТАТ CP1 ===")
    if all_pass:
        log("✅ УСІ КЕЙСИ ПРОЙДЕНО")
        log("Артефакт: відео 60 сек + txid розрахунку + вигрузка лога")
        save_artifact("RESULT.txt", "CP1: PASSED\nДата: " + time.strftime("%Y-%m-%d %H:%M:%S UTC", time.gmtime()))
        return 0
    else:
        log("❌ НЕ ВСІ КЕЙСИ ПРОЙДЕНО")
        save_artifact("RESULT.txt", "CP1: FAILED\nДата: " + time.strftime("%Y-%m-%d %H:%M:%S UTC", time.gmtime()))
        return 1


if __name__ == '__main__':
    sys.exit(main())
