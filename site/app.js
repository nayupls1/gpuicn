// Site behavior: component grid, search, detail dialog, theme playground.

const $ = (selector, root = document) => root.querySelector(selector);
const $$ = (selector, root = document) => [...root.querySelectorAll(selector)];

// ---------- syntax highlighting ----------
const RUST_KEYWORDS = new Set("as async await break const continue crate else enum false fn for if impl in let loop match mod move mut pub ref return self Self static struct super trait true type use where while dyn".split(" "));

function escapeHtml(text) {
  return text.replace(/&/g, "&amp;").replace(/</g, "&lt;").replace(/>/g, "&gt;");
}

function highlight(code, lang = "rust") {
  const pattern = lang === "sh"
    ? /(#.*$)|("(?:\\.|[^"\\])*")|(\b[\w-]+\b)|([\s\S])/gm
    : /(\/\/.*$)|("(?:\\.|[^"\\])*")|(\b\d[\d_]*(?:\.\d*)?\b)|([A-Za-z_]\w*)|([\s\S])/gm;
  let out = "";
  let first = true;
  for (const m of code.matchAll(pattern)) {
    const [text, comment, string] = m;
    const wrap = (cls) => `<span class="${cls}">${escapeHtml(text)}</span>`;
    if (comment) out += wrap("tk-com");
    else if (string) out += wrap("tk-str");
    else if (lang === "sh") {
      if (m[3] && (first || text === "gpuicn" || text === "cargo")) out += wrap("tk-fn");
      else if (m[3] && text.startsWith("-")) out += wrap("tk-kw");
      else out += escapeHtml(text);
      if (text === "\n") first = true;
      else if (m[3]) first = false;
    } else if (m[3]) out += wrap("tk-num");
    else if (m[4]) {
      const next = code[m.index + text.length];
      if (RUST_KEYWORDS.has(text)) out += wrap("tk-kw");
      else if (next === "(" || next === "!") out += wrap("tk-fn");
      else if (/^[A-Z]/.test(text)) out += wrap("tk-ty");
      else out += escapeHtml(text);
    } else out += escapeHtml(text);
  }
  return out;
}

function renderCode(pre, code) {
  pre.innerHTML = highlight(code, pre.dataset.lang) + `<button class="btn outline icon sm copy-btn" aria-label="Copy">${I("copy")}</button>`;
  $(".copy-btn", pre).addEventListener("click", (event) => copy(code, event.currentTarget));
}

// ---------- clipboard ----------
function copy(text, button) {
  navigator.clipboard?.writeText(text);
  if (!button) return;
  const before = button.innerHTML;
  button.innerHTML = I("check");
  setTimeout(() => (button.innerHTML = before), 1200);
}

document.addEventListener("click", (event) => {
  const button = event.target.closest("[data-copy]");
  if (button) {
    event.stopPropagation();
    copy(button.dataset.copy, button);
  }
});

// ---------- interactive previews ----------
document.addEventListener("click", (event) => {
  const toggle = event.target.closest("[data-toggle]");
  if (toggle) toggle.classList.toggle("on");
  const trigger = event.target.closest(".acc-trigger");
  if (trigger) trigger.parentElement.classList.toggle("on");
  const tab = event.target.closest(".tab");
  if (tab) $$(".tab", tab.parentElement).forEach((t) => t.classList.toggle("on", t === tab));
  const radio = event.target.closest("[data-radio] .hstack");
  if (radio) $$(".radio", radio.parentElement).forEach((r) => r.classList.toggle("on", radio.contains(r)));
});

// Scale a preview down until it fits its stage.
function fit(stage) {
  const inner = stage.firstElementChild;
  if (!inner) return;
  inner.style.transform = "";
  const pad = 24;
  const scale = Math.min(1, (stage.clientWidth - pad) / inner.offsetWidth, (stage.clientHeight - pad) / inner.offsetHeight);
  if (scale < 1) inner.style.transform = `scale(${scale})`;
}
const resizeObserver = new ResizeObserver((entries) => entries.forEach((e) => fit(e.target)));

function stage(el, component) {
  el.innerHTML = `<div class="autofit">${component.preview}</div>`;
  resizeObserver.observe(el);
  requestAnimationFrame(() => fit(el));
}

// ---------- grid ----------
let filter = { query: "", category: "All" };
let visible = COMPONENTS;

function matches(component) {
  const q = filter.query.trim().toLowerCase();
  const inCategory = filter.category === "All" || component.category === filter.category;
  const inQuery = !q || [component.title, component.name, component.description].some((s) => s.toLowerCase().includes(q));
  return inCategory && inQuery;
}

function renderChips() {
  const chips = $("#chips");
  chips.innerHTML = ["All", ...CATEGORIES]
    .map((c) => `<button class="chip ${c === filter.category ? "on" : ""}" data-category="${c}">${c}</button>`)
    .join("");
  $$(".chip", chips).forEach((chip) =>
    chip.addEventListener("click", () => {
      filter.category = chip.dataset.category;
      renderChips();
      renderGrid();
    }),
  );
}

function renderGrid() {
  const grid = $("#grid");
  visible = COMPONENTS.filter(matches);
  $("#count").textContent = `${visible.length} of ${COMPONENTS.length}`;
  if (!visible.length) {
    grid.innerHTML = `<div class="empty-results" style="grid-column:1/-1">No components match “${escapeHtml(filter.query)}”.</div>`;
    return;
  }
  grid.innerHTML = visible
    .map(
      (c, ix) => `<article class="tile" data-ix="${ix}" tabindex="0">
        <div class="stage"></div>
        <div class="tile-foot">
          <div class="tile-title">${c.title}<span class="tile-cat">${c.category}</span></div>
          <div class="tile-desc">${c.description}</div>
        </div>
      </article>`,
    )
    .join("");
  $$(".tile", grid).forEach((tile) => {
    const component = visible[+tile.dataset.ix];
    stage($(".stage", tile), component);
    tile.addEventListener("click", () => openDetail(+tile.dataset.ix));
    tile.addEventListener("keydown", (e) => e.key === "Enter" && openDetail(+tile.dataset.ix));
  });
}

$("#search").addEventListener("input", (event) => {
  filter.query = event.target.value;
  renderGrid();
});

// ---------- detail dialog ----------
let current = -1;

function openDetail(ix) {
  current = (ix + visible.length) % visible.length;
  const c = visible[current];
  $("#d-title").textContent = c.title;
  $("#d-desc").textContent = c.description;
  const command = `gpuicn add ${c.name}`;
  $("#d-cmd").innerHTML = `<span class="prompt">$</span> ${command}<button class="btn ghost icon sm" data-copy="${command}" aria-label="Copy">${I("copy")}</button>`;
  renderCode($("#d-code"), c.code);
  $("#overlay").classList.add("open");
  document.body.style.overflow = "hidden";
  stage($("#d-stage"), c);
  history.replaceState(null, "", `#${c.name}`);
}

function closeDetail() {
  $("#overlay").classList.remove("open");
  document.body.style.overflow = "";
  current = -1;
  history.replaceState(null, "", "#components");
}

$("#overlay").addEventListener("click", (event) => event.target.id === "overlay" && closeDetail());
$("#d-close").addEventListener("click", closeDetail);
document.addEventListener("keydown", (event) => {
  if (current >= 0) {
    if (event.key === "Escape") closeDetail();
    if (event.key === "ArrowRight") openDetail(current + 1);
    if (event.key === "ArrowLeft") openDetail(current - 1);
  } else if (event.key === "/" && document.activeElement.tagName !== "INPUT") {
    event.preventDefault();
    $("#search").focus();
  }
});

// ---------- theme ----------
function setDark(dark) {
  document.documentElement.classList.toggle("dark", dark);
  localStorage.setItem("gpuicn-theme", dark ? "dark" : "light");
  $("#theme-toggle").innerHTML = I(dark ? "sun" : "moon");
  applyPalette();
}
$("#theme-toggle").addEventListener("click", () => setDark(!document.documentElement.classList.contains("dark")));

// shadcn theme presets: [light primary, light fg, dark primary, dark fg].
const PALETTES = {
  neutral: ["0.205 0 0", "0.985 0 0", "0.922 0 0", "0.205 0 0"],
  blue: ["0.488 0.243 264.376", "0.97 0.014 254.604", "0.546 0.245 262.881", "0.97 0.014 254.604"],
  green: ["0.648 0.2 131.684", "0.986 0.031 120.757", "0.648 0.2 131.684", "0.986 0.031 120.757"],
  rose: ["0.586 0.253 17.585", "0.969 0.015 12.422", "0.645 0.246 16.439", "0.969 0.015 12.422"],
  orange: ["0.646 0.222 41.116", "0.98 0.016 73.684", "0.705 0.213 47.604", "0.98 0.016 73.684"],
  violet: ["0.541 0.281 293.009", "0.969 0.016 293.756", "0.606 0.25 292.717", "0.969 0.016 293.756"],
};
let palette = "neutral";
let radius = 0.625;

function applyPalette() {
  const dark = document.documentElement.classList.contains("dark");
  const [lp, lf, dp, df] = PALETTES[palette];
  const style = document.documentElement.style;
  if (palette === "neutral") {
    ["--primary", "--primary-foreground", "--ring"].forEach((v) => style.removeProperty(v));
  } else {
    style.setProperty("--primary", `oklch(${dark ? dp : lp})`);
    style.setProperty("--primary-foreground", `oklch(${dark ? df : lf})`);
    style.setProperty("--ring", `oklch(${dark ? dp : lp})`);
  }
  style.setProperty("--radius", `${radius}rem`);
  renderThemeCode(dark ? dp : lp, dark ? df : lf);
  renderTokens();
}

function renderThemeCode(primary, foreground) {
  const rust = (oklch) => oklch.split(" ").map((n) => (n.includes(".") ? n : `${n}.0`)).join(", ");
  const dark = document.documentElement.classList.contains("dark");
  const code = `// src/components/ui/theme.rs
impl ThemeColors {
    pub fn ${dark ? "dark" : "light"}() -> Self {
        Self {
            // --primary: oklch(${primary});
            primary: oklch(${rust(primary)}),
            primary_foreground: oklch(${rust(foreground)}),
            background: ${dark ? "oklch(0.145, 0.0, 0.0)" : "oklch(1.0, 0.0, 0.0)"},
            border: ${dark ? "oklcha(1.0, 0.0, 0.0, 0.1)" : "oklch(0.922, 0.0, 0.0)"},
            // …every other shadcn token
        }
    }
}

impl Theme {
    pub fn new(mode: ThemeMode) -> Self {
        Self {
            radius: px(${Math.round(radius * 16)}.),
            font_family: ".SystemUIFont".into(),
            shadows: true,
            // …
        }
    }
}

// At runtime
Theme::change(ThemeMode::Dark, cx);`;
  renderCode($("#theme-code"), code);
}

function renderTokens() {
  const names = ["background", "foreground", "primary", "secondary", "muted", "accent", "destructive", "border", "chart-1", "chart-2", "chart-3", "chart-4"];
  $("#tokens").innerHTML = names.map((n) => `<span class="token"><i style="background:var(--${n})"></i>${n}</span>`).join("");
}

$("#swatches").innerHTML = Object.entries(PALETTES)
  .map(([name, [p]]) => `<button class="swatch ${name === palette ? "on" : ""}" title="${name}" data-palette="${name}" style="background:oklch(${p})"></button>`)
  .join("");
$$(".swatch").forEach((swatch) =>
  swatch.addEventListener("click", () => {
    palette = swatch.dataset.palette;
    $$(".swatch").forEach((s) => s.classList.toggle("on", s === swatch));
    applyPalette();
  }),
);
$("#radius").addEventListener("input", (event) => {
  radius = +event.target.value;
  $("#radius-value").textContent = `${radius}rem`;
  applyPalette();
});

// ---------- boot ----------
$$("[data-icon]").forEach((el) => (el.tagName === "SPAN" ? (el.outerHTML = I(el.dataset.icon)) : (el.innerHTML = I(el.dataset.icon))));
$$("[data-copy]").forEach((el) => (el.innerHTML = I("copy")));
$("#d-close").innerHTML = I("x");
$$("pre.code:not(#theme-code):not(#d-code)").forEach((pre) => renderCode(pre, pre.textContent));
$("#count-pill").textContent = `${COMPONENTS.length} components`;
renderChips();
renderGrid();
setDark(document.documentElement.classList.contains("dark"));

const deepLink = location.hash.slice(1);
const linked = COMPONENTS.findIndex((c) => c.name === deepLink);
if (linked >= 0) openDetail(linked);
