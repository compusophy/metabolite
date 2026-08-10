#!/usr/bin/env bash
# Build the serverless observatory: the whole world compiled to wasm and
# inlined into one dist/index.html (deployable as a static page anywhere).
# The dashboard is unchanged — a shim reroutes its fetch() calls onto the
# wasm ABI, so one HTML file observes either a server world or an in-page
# world. No bindgen, no deps, litelite-shell style.
set -euo pipefail
cd "$(dirname "$0")/.."

cargo build --release --lib --target wasm32-unknown-unknown
mkdir -p dist
python - <<'EOF'
import base64, pathlib

wasm = pathlib.Path("target/wasm32-unknown-unknown/release/metabolite.wasm").read_bytes()
b64 = base64.b64encode(wasm).decode()
html = pathlib.Path("web/index.html").read_text(encoding="utf-8")

shim = """<script>
(async () => {
  const bytes = Uint8Array.from(atob("%B64%"), c => c.charCodeAt(0));
  const { instance } = await WebAssembly.instantiate(bytes, {});
  const E = instance.exports;
  const mem = () => new Uint8Array(E.memory.buffer);
  const dec = new TextDecoder(), enc = new TextEncoder();
  const seed = BigInt(new URLSearchParams(location.search).get('seed') || 1618);
  E.mb_new(seed);
  let paused = false, tps = 8;
  window.__prof = { ticks: 0, worst: 0, errs: [] };
  window.addEventListener('error', e => __prof.errs.push('err:' + e.message));
  window.addEventListener('unhandledrejection', e => __prof.errs.push('rej:' + e.reason));
  setInterval(() => {
    if (paused) return;
    const t0 = performance.now();
    try { E.mb_tick(Math.max(1, Math.round(tps / 8))); __prof.ticks++; }
    catch (e) { __prof.errs.push('tick:' + e); }
    const dt = performance.now() - t0;
    if (dt > __prof.worst) __prof.worst = dt;
  }, 125);
  const grab = len => dec.decode(mem().slice(E.mb_out_ptr(), E.mb_out_ptr() + len));
  const R = s => ({ ok: true, json: async () => JSON.parse(s), text: async () => s });
  window.fetch = async (url, opts) => {
    url = String(url);
    const q = new URL(url, location.origin).searchParams;
    if (url.startsWith('/state')) return R(grab(E.mb_state(paused ? 1 : 0, tps)));
    if (url.startsWith('/agent')) return R(grab(E.mb_agent(+q.get('id'))));
    if (url.startsWith('/physics')) return R(grab(E.mb_physics()));
    if (url.startsWith('/inject')) {
      const b = enc.encode((opts && opts.body) || '');
      mem().set(b, E.mb_in_alloc(b.length));
      return R(grab(E.mb_inject(b.length)));
    }
    if (url.startsWith('/control')) {
      if (q.has('pause')) paused = q.get('pause') === '1';
      if (q.has('tps')) tps = Math.min(1000, Math.max(1, +q.get('tps')));
      return R('{"ok":true}');
    }
    return R('{}');
  };
})();
</script>
"""
shim = shim.replace("%B64%", b64)
at = html.index("<script>")
out = html[:at] + shim + html[at:]
pathlib.Path("dist/index.html").write_text(out, encoding="utf-8")
print(f"dist/index.html: {len(out)//1024} KB (wasm {len(wasm)//1024} KB)")
EOF
