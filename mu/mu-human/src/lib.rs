//! M9 mu-human — канал владельца.
//!
//! RISK-M9-1: PayConfirm приходит из конвейера (тот же объект, что уйдёт в M7a); связка intent_hash.
//! RISK-M9-2: Approved непроизводим кодом — auth_proof = подпись owner-ключом (auth-required)
//!            над domain("mu.human.v1") ‖ intent_hash; конвейер верифицирует подпись.
//! RISK-M9-5: TTL по монотонным часам; поздний Approved после дедлайна не исполняется.
//!
//! Платформенные диалоги (LocalAuthentication/BiometricPrompt/Hello) — за trait Presenter:
//! реальные реализации — на устройствах; здесь Presenter-мок для логики и тестов.
#![forbid(unsafe_code)]

use mu_common::{Amount, CanonAddress, Hash32};
use mu_vault::domain::tagged_digest;
use mu_vault::{DomainTag, P256Sig, Vault, VaultErr};
use p256::ecdsa::signature::hazmat::PrehashVerifier;
use p256::ecdsa::{Signature as P256Signature, VerifyingKey as P256Verifying};
use std::time::{Duration, Instant};

/// Данные диалога платежа. Собираются в M6 из intent'а, прошедшего Ω/Δ (RISK-M9-1).
/// purpose агента здесь ОТСУТСТВУЕТ by design — вместо него wl_label из Δ.
#[derive(Clone, Debug)]
pub struct PayConfirm {
    pub recipient: CanonAddress,
    pub wl_label: Option<String>,
    pub amount: Amount,
    pub gas_est: Amount,
    pub agent_id: String,
    pub remaining_window: Amount,
    pub intent_hash: Hash32,
    /// бейдж «ПЕРВЫЙ ПЛАТЁЖ» (RISK-M9-4)
    pub first_payment_to_recipient: bool,
}

/// Криптодоказательство решения (RISK-M9-2). Не булев флаг.
#[derive(Clone, Debug)]
pub struct AuthProof {
    pub sig: P256Sig,
    pub intent_hash: Hash32,
}

#[derive(Debug)]
pub enum HumanDecision {
    Approved(AuthProof),
    Denied,
    Timeout,
}

/// Платформенный слой показа диалога. Возвращает лишь намерение владельца;
/// криптографию делает confirm_payment через vault.
pub trait Presenter: Send + Sync {
    /// Показать диалог, вернуть выбор владельца (или None при закрытии/таймауте UI).
    /// elapsed_budget — сколько времени осталось (для отображения таймера).
    fn present(&self, req: &PayConfirm, budget: Duration) -> PresenterChoice;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PresenterChoice {
    Approve,
    Deny,
    NoResponse,
}

/// Полный цикл подтверждения платежа.
/// deadline рассчитан в M6; монотонные часы (RISK-X-2 / RISK-M9-5).
pub fn confirm_payment(
    req: &PayConfirm,
    presenter: &dyn Presenter,
    vault: &dyn Vault,
    deadline: Instant,
) -> HumanDecision {
    let now = Instant::now();
    if now >= deadline {
        return HumanDecision::Timeout;
    }
    let budget = deadline.saturating_duration_since(now);
    match presenter.present(req, budget) {
        PresenterChoice::Deny => HumanDecision::Denied,
        PresenterChoice::NoResponse => HumanDecision::Timeout,
        PresenterChoice::Approve => {
            // RISK-M9-5: поздний тап после дедлайна не превращается в Approved
            if Instant::now() >= deadline {
                return HumanDecision::Timeout;
            }
            // RISK-M9-2: Approved = подпись owner-ключом над intent_hash.
            // owner_sign сам требует свежей платформенной аутентификации (RISK-M4-3).
            match vault.owner_sign(DomainTag::MuHuman, &req.intent_hash) {
                Ok(sig) => HumanDecision::Approved(AuthProof { sig, intent_hash: req.intent_hash }),
                Err(VaultErr::UserAuthRequired) | Err(VaultErr::UserAuthFailed) => HumanDecision::Denied,
                Err(_) => HumanDecision::Denied,
            }
        }
    }
}

/// Верификация auth_proof на стороне конвейера (RISK-M9-2):
/// даже полностью скомпрометированный M9 не породит валидный Approved.
pub fn verify_auth_proof(proof: &AuthProof, expected_intent: &Hash32, owner_pubkey: &[u8]) -> bool {
    if proof.intent_hash != *expected_intent {
        return false;
    }
    let tagged = tagged_digest(DomainTag::MuHuman, expected_intent);
    let Ok(vk) = P256Verifying::from_sec1_bytes(owner_pubkey) else { return false; };
    let Ok(sig) = P256Signature::from_slice(&proof.sig.0) else { return false; };
    vk.verify_prehash(&tagged.0, &sig).is_ok()
}

/// Строка диалога: label ВСЕГДА сопровождается усечённым адресом (RISK-M9-3).
pub fn render_recipient_line(req: &PayConfirm) -> String {
    match &req.wl_label {
        Some(l) => format!("{} ({})", l, req.recipient.redacted()),
        None => req.recipient.redacted(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mu_vault::backend::SoftVault;

    struct FixedPresenter(PresenterChoice);
    impl Presenter for FixedPresenter {
        fn present(&self, _r: &PayConfirm, _b: Duration) -> PresenterChoice { self.0 }
    }

    fn req() -> PayConfirm {
        PayConfirm {
            recipient: CanonAddress::canon("0xabcdef0123456789abcdef0123456789abcdef01", 8453).unwrap(),
            wl_label: Some("API Service".into()),
            amount: Amount::from_minor(5_000_000),
            gas_est: Amount::from_minor(1000),
            agent_id: "agent-1".into(),
            remaining_window: Amount::from_minor(10_000_000),
            intent_hash: Hash32([0xCC; 32]),
            first_payment_to_recipient: true,
        }
    }
    fn owner_pubkey(v: &SoftVault) -> Vec<u8> {
        // SoftVault owner key = [2;32]; получаем pubkey из p256
        use p256::ecdsa::SigningKey;
        let sk = SigningKey::from_bytes((&[2u8; 32]).into()).unwrap();
        sk.verifying_key().to_encoded_point(true).as_bytes().to_vec()
    }

    #[test]
    fn approved_proof_verifies() {
        // RISK-M9-2 happy path
        let v = SoftVault::for_test([1; 32], [2; 32], [3; 32]);
        let r = req();
        let d = confirm_payment(&r, &FixedPresenter(PresenterChoice::Approve), &v,
                                Instant::now() + Duration::from_secs(60));
        let HumanDecision::Approved(proof) = d else { panic!("expected Approved") };
        assert!(verify_auth_proof(&proof, &r.intent_hash, &owner_pubkey(&v)));
    }

    #[test]
    fn garbage_proof_rejected() {
        // RISK-M9-2: мусорный proof (мок скомпрометированного M9) не проходит верификацию
        let v = SoftVault::for_test([1; 32], [2; 32], [3; 32]);
        let fake = AuthProof { sig: mu_vault::P256Sig([0u8; 64]), intent_hash: Hash32([0xCC; 32]) };
        assert!(!verify_auth_proof(&fake, &Hash32([0xCC; 32]), &owner_pubkey(&v)));
    }

    #[test]
    fn proof_bound_to_intent_hash() {
        // подпись под intent A не проходит для intent B (RISK-M9-1 связка)
        let v = SoftVault::for_test([1; 32], [2; 32], [3; 32]);
        let r = req();
        let d = confirm_payment(&r, &FixedPresenter(PresenterChoice::Approve), &v,
                                Instant::now() + Duration::from_secs(60));
        let HumanDecision::Approved(proof) = d else { panic!() };
        let other = Hash32([0xDD; 32]);
        assert!(!verify_auth_proof(&proof, &other, &owner_pubkey(&v)));
    }

    #[test]
    fn no_biometry_means_denied_not_approved() {
        // RISK-M4-3 → M9: отказ биометрии не может дать Approved
        let mut v = SoftVault::for_test([1; 32], [2; 32], [3; 32]);
        v.set_owner_auth(false);
        let d = confirm_payment(&req(), &FixedPresenter(PresenterChoice::Approve), &v,
                                Instant::now() + Duration::from_secs(60));
        assert!(matches!(d, HumanDecision::Denied));
    }

    #[test]
    fn expired_deadline_is_timeout() {
        // RISK-M9-5
        let v = SoftVault::for_test([1; 32], [2; 32], [3; 32]);
        let past = Instant::now() - Duration::from_secs(1);
        let d = confirm_payment(&req(), &FixedPresenter(PresenterChoice::Approve), &v, past);
        assert!(matches!(d, HumanDecision::Timeout));
    }

    #[test]
    fn label_always_with_address() {
        // RISK-M9-3: label не может скрыть адрес
        let line = render_recipient_line(&req());
        assert!(line.contains("API Service"));
        assert!(line.contains("0xabcdef")); // усечённый адрес присутствует
    }
}
