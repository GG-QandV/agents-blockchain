// C1 UI-логика. Vanilla JS без внешних пакетов (N-02: zero runtime-CDN; отклонение
// от «TS» спеки — без сборщика, аудит по одному файлу; типы — JSDoc).
// ВСЯ валидация — на Rust-стороне (mu-policy) через invoke; UI только рендерит.
const { invoke } = window.__TAURI__.core;

/** @type {{daily_limit:string, confirm_threshold:string, whitelist:Array<{address_hex:string,redacted:string,label:string}>}} */
let form = { daily_limit: "0", confirm_threshold: "0", whitelist: [] };
let baseHash = "0".repeat(64);
let pendingAddr = null; // адрес, ожидающий подтверждения в диалоге S3

const $ = (id) => document.getElementById(id);

// ── identicon (~40 LOC): детерминированный 8×8 из hex-адреса (RISK-M9-3 визуальная сверка) ──
function identicon(canvas, hex) {
  const ctx = canvas.getContext("2d");
  let seed = 0;
  for (const c of hex) seed = (seed * 31 + c.charCodeAt(0)) >>> 0;
  const rnd = () => (seed = (seed * 1103515245 + 12345) >>> 0) / 4294967296;
  const hue = Math.floor(rnd() * 360);
  ctx.fillStyle = "#0d1013"; ctx.fillRect(0, 0, 8, 8);
  ctx.fillStyle = `hsl(${hue} 70% 55%)`;
  for (let y = 0; y < 8; y++)
    for (let x = 0; x < 4; x++)
      if (rnd() > 0.5) { ctx.fillRect(x, y, 1, 1); ctx.fillRect(7 - x, y, 1, 1); }
}

// ── рендер ────────────────────────────────────────────────────────────────
function renderWl() {
  const tb = $("wl").querySelector("tbody");
  tb.innerHTML = "";
  form.whitelist.forEach((e, i) => {
    const tr = document.createElement("tr");
    const icn = document.createElement("canvas");
    icn.className = "icn"; icn.width = 8; icn.height = 8;
    icn.style.width = "24px"; icn.style.height = "24px";
    identicon(icn, e.address_hex);
    const td0 = document.createElement("td"); td0.appendChild(icn);
    const td1 = document.createElement("td"); td1.className = "mono";
    td1.textContent = e.redacted; // label ВСЕГДА рядом с адресом (RISK-M9-3)
    const td2 = document.createElement("td"); td2.textContent = e.label;
    const td3 = document.createElement("td");
    const rm = document.createElement("button"); rm.className = "ghost"; rm.textContent = "✕";
    rm.onclick = () => { form.whitelist.splice(i, 1); refresh(); };
    td3.appendChild(rm);
    tr.append(td0, td1, td2, td3);
    tb.appendChild(tr);
  });
}

function renderReport(rep) {
  $("rules-msgs").innerHTML = rep.errors.map(e => `<div class="msg-err">✗ ${e}</div>`).join("");
  $("warns").innerHTML = rep.warnings.map(w => `<div class="msg-warn">⚠ ${w}</div>`).join("");
  $("btn-propose").disabled = rep.errors.length > 0; // S4: экспорт только при 0 ошибок (F-03)
  $("limit").classList.toggle("bad", rep.errors.some(e => e.startsWith("ELim")));
  $("threshold").classList.toggle("bad", rep.errors.some(e => e.startsWith("EThr")));
}

async function refresh() {
  renderWl();
  try {
    const rep = await invoke("validate_form", { delta: form });
    renderReport(rep);
  } catch (e) {
    $("rules-msgs").innerHTML = `<div class="msg-err">✗ ${e}</div>`;
    $("btn-propose").disabled = true;
  }
}

// ── события ───────────────────────────────────────────────────────────────
$("limit").addEventListener("input", () => { form.daily_limit = $("limit").value; refresh(); });
$("threshold").addEventListener("input", () => { form.confirm_threshold = $("threshold").value; refresh(); });

$("btn-add").addEventListener("click", async () => {
  const input = $("new-addr").value.trim();
  const r = await invoke("check_address", { input });
  if (!r.ok) { $("addr-preview").innerHTML = `<span class="msg-err">✗ ${r.error}</span>`; return; }
  // S3: диалог сверки — укрупнённый redacted + identicon
  pendingAddr = r;
  $("dlg-addr").textContent = r.redacted;
  $("dlg-warn").textContent = r.eip55_warning ? "⚠ Адрес без EIP-55 чексуммы (W-ADR-01) — сверьте вручную" : "";
  identicon($("dlg-icn"), r.hex);
  $("dlg-confirm").showModal();
});
$("dlg-no").addEventListener("click", () => { pendingAddr = null; $("dlg-confirm").close(); });
$("dlg-yes").addEventListener("click", () => {
  form.whitelist.push({ address_hex: pendingAddr.hex, redacted: pendingAddr.redacted,
                        label: $("new-label").value.trim() || "(без label)" });
  pendingAddr = null; $("new-addr").value = ""; $("new-label").value = "";
  $("addr-preview").innerHTML = "";
  $("dlg-confirm").close();
  refresh();
});

$("btn-save").addEventListener("click", async () => {
  try { $("result").textContent = await invoke("save_form", { delta: form, baseHash }); }
  catch (e) { $("result").innerHTML = `<span class="msg-err">${e}</span>`; }
});
$("btn-propose").addEventListener("click", async () => {
  try {
    await invoke("save_form", { delta: form, baseHash });
    const resp = await invoke("propose"); // S5: ответ демона (Applied/Rejected{Stale,…})
    $("result").textContent = `демон: ${resp}`;
    if (resp.includes("Stale")) load(); // §10: авто-перезагрузка при TOCTOU
  } catch (e) { $("result").innerHTML = `<span class="msg-err">${e}</span>`; }
});

// ── загрузка (S1) ────────────────────────────────────────────────────────
async function load() {
  const st = await invoke("get_state");
  form = st.delta;
  baseHash = st.delta_hash;
  $("dhash").textContent = st.delta_hash.slice(0, 16) + "…";
  $("limit").value = st.delta.daily_limit;
  $("threshold").value = st.delta.confirm_threshold;
  const b = $("daemon-badge");
  b.textContent = st.daemon ? "демон: online" : "демон: offline (черновик)";
  b.classList.toggle("on", st.daemon);
  renderReport(st.report);
  renderWl();
}
load();
