//! Живой smoke M7a-sui против Sui testnet.
//! Запуск (у агента, где есть сеть):
//!   SUI_RPC1=https://fullnode.testnet.sui.io:443 \
//!   SUI_RPC2=<второй провайдер> \
//!   SUI_COIN_TYPE=<type-tag тестового USDC> \
//!   SUI_BUILD_METHOD=<метод сборки из офиц. docs (см. ETAP2, источники)> \
//!   SUI_RECIPIENT=0x<32B hex> \
//!   cargo run -p sui-smoke
//! Кошелёк: детерминированный dev-ключ SoftVault ([3;32]) — адрес печатается,
//! пополнить с faucet testnet перед прогоном.
use mu_common::{Amount, CanonAddress};
use mu_connect::sui::{signing_digest, sui_address_from_pubkey, SuiConnector};
use mu_connect::sui_jsonrpc::{JsonRpcClient, SuiRpcConfig};
use mu_connect::{Intent, TxRef, TxStatus};
use mu_vault::backend::SoftVault;
use mu_vault::Vault;
use std::process::Command;

/// HTTP POST через системный curl — без TLS-зависимостей в бинаре.
fn curl_post(url: &str, body: &str, timeout_ms: u64) -> Result<String, String> {
    let out = Command::new("curl")
        .args(["-sS", "--max-time", &format!("{}", timeout_ms.div_ceil(1000).max(1)),
               "-H", "Content-Type: application/json", "-d", body, url])
        .output()
        .map_err(|e| format!("curl spawn: {e}"))?;
    if !out.status.success() {
        return Err(format!("curl exit {}: {}", out.status, String::from_utf8_lossy(&out.stderr)));
    }
    String::from_utf8(out.stdout).map_err(|e| format!("utf8: {e}"))
}

fn env(k: &str) -> String {
    std::env::var(k).unwrap_or_else(|_| panic!("env {k} required (см. шапку файла)"))
}
fn client(url: String, build_method: String) -> JsonRpcClient {
    JsonRpcClient {
        cfg: SuiRpcConfig {
            url,
            m_build: build_method,
            m_dry: "sui_dryRunTransactionBlock".into(),
            m_exec: "sui_executeTransactionBlock".into(),
            m_get: "sui_getTransactionBlock".into(),
            timeout_ms: 10_000,
        },
        post: curl_post,
    }
}

fn main() {
    let vault = SoftVault::for_test([1; 32], [2; 32], [3; 32]);
    let signer = vault.tx_signer_p256().expect("p256 signer");
    let (_s, pk) = signer.sign_prehash(&signing_digest(b"probe")).expect("probe sign");
    let wallet_addr = sui_address_from_pubkey(&pk);
    println!("dev wallet Sui address (P-256, flag 0x02): 0x{}",
             wallet_addr.iter().map(|b| format!("{b:02x}")).collect::<String>());
    println!("→ сверить: sui keytool (README-LIVE §3); пополнить testnet-USDC на этот адрес\n");

    let recipient_hex = env("SUI_RECIPIENT");
    let rcp = CanonAddressLike32(&recipient_hex).parse().expect("SUI_RECIPIENT: 0x + 64 hex");

    let conn = SuiConnector {
        network: "testnet",
        coin_type: env("SUI_COIN_TYPE"),
        wallet_addr,
        rpc1: client(env("SUI_RPC1"), env("SUI_BUILD_METHOD")),
        rpc2: client(env("SUI_RPC2"), env("SUI_BUILD_METHOD")),
    };
    let intent = Intent {
        recipient: CanonAddress::canon("0xabcdef0123456789abcdef0123456789abcdef01000000000000000000000000", 1).unwrap(), // 20B-поле unused в sui-пути
        amount: Amount::from_minor(1_000_000), // 1 единица 6-decimals
        chain_id: 1,
    };

    println!("== quote =="); println!("{:?}", conn.quote(&intent));
    println!("== execute ==");
    let signer2 = vault.tx_signer_p256().unwrap();
    match conn.execute(&intent, &rcp, signer2) {
        Ok(TxRef::Real { tx_hash, .. }) => {
            println!("ACCEPTED digest={}", tx_hash.iter().map(|b| format!("{b:02x}")).collect::<String>());
            println!("== status poll ==");
            for i in 0..10 {
                std::thread::sleep(std::time::Duration::from_secs(2));
                let st = conn.status(&TxRef::Real { tx_hash, chain_nonce: 0 });
                println!("[{i}] {st:?}");
                if matches!(st, Ok(TxStatus::Settled { .. }) | Ok(TxStatus::Failed { .. })) { break; }
            }
        }
        other => println!("outcome: {other:?}  ← сверить с матрицей RISK-M7-1 (Unknown = НЕ повторять)"),
    }
}

struct CanonAddressLike32<'a>(&'a str);
impl<'a> CanonAddressLike32<'a> {
    fn parse(&self) -> Option<[u8; 32]> {
        let s = self.0.strip_prefix("0x").unwrap_or(self.0);
        if s.len() != 64 { return None; }
        let mut a = [0u8; 32];
        for i in 0..32 { a[i] = u8::from_str_radix(&s[i*2..i*2+2], 16).ok()?; }
        Some(a)
    }
}
