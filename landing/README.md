# Landing & Stripe Payment Flow

## Структура

```
landing/
├── index.html        # Лендінг (самодостатній HTML, темна тема)
├── gen-license.py    # Генератор license.key для продавця
└── README.md         # Цей файл
```

## Stripe Payment Link (налаштування)

1. Зайдіть у **Stripe Dashboard → Products → Create Product**
2. Назва: "μ Gate Commercial Embed License"
3. Ціна: $299 разово
4. Створіть **Payment Link** для цього продукту
5. Замініть `href="#"` в `landing/index.html` на URL Payment Link

## Потік після покупки (ручний)

```
Клієнт → Stripe Checkout → оплата $299 → Email продавцю
                                                  ↓
                              Продавець генерує license.key:
                                export MU_LICENSE_SECRET=<секрет>
                                python3 landing/gen-license.py <customer_id>
                                                  ↓
                              Продавець надсилає license.key клієнту email'ом
                                                  ↓
                              Клієнт кладе license.key в робочу директорію μ-daemon
                                та перезапускає → "License: COMMERCIAL"
```

## Генерація секретного ключа

```bash
# Згенерувати нову пару (лише раз, перед релізом):
python3 -c "
import nacl.bindings, binascii
seed = nacl.bindings.randombytes(32)
sk = nacl.bindings.crypto_sign_seed_keypair(seed)[0]
print('MU_LICENSE_SECRET:', binascii.hexlify(sk).decode())
print('Вставити в mu-license/src/lib.rs як LICENSE_VERIFY_KEY:')
print(list(nacl.bindings.crypto_sign_sk_to_pk(sk)))
"
```

> ⚠️ `LICENSE_VERIFY_KEY` у `mu-license/src/lib.rs` — це публічний ключ.
> Замініть тестовий на реальний перед релізом.

## DoD

- [ ] Stripe Payment Link створено (замінити href)
- [ ] Секретний ключ згенеровано, публічний вшито в бінар
- [ ] Тестова покупка → license.key → бінар приймає
