//! Beautiful charts: bar, line, area and pie — drawn as inline `<svg>` and
//! colored by the theme's `chart_1..chart_5` tokens.
//!
//! ```ignore
//! <BarChart
//!     labels={["January", "February", "March", "April", "May", "June"]}
//!     series={vec![
//!         ChartSeries::new("Desktop").data([186., 305., 237., 73., 209., 214.]),
//!         ChartSeries::new("Mobile").data([80., 200., 120., 190., 130., 140.]),
//!     ]}
//! />
//!
//! <LineChart labels={..} series={..} dots />
//! <AreaChart labels={..} series={..} stacked />
//! <PieChart slices={vec![ChartSlice::new("Chrome", 275.), ChartSlice::new("Safari", 200.)]} donut={0.6f32} />
//! ```

use std::rc::Rc;

use rsx::prelude::*;

use super::theme::{theme, ColorExt};

/// One data series (a bar color, a line…).
#[derive(Clone, Debug, PartialEq)]
pub struct ChartSeries {
    label: SharedString,
    color: Option<Color>,
    data: Vec<f64>,
}

impl ChartSeries {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self { label: label.into(), color: None, data: Vec::new() }
    }

    /// Override the theme chart color.
    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn data(mut self, data: impl IntoIterator<Item = f64>) -> Self {
        self.data = data.into_iter().collect();
        self
    }
}

/// X-axis category labels: `labels={["Jan", "Feb"]}`, or any list of strings.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChartLabels(pub Vec<SharedString>);

impl<S: Into<SharedString>, I: IntoIterator<Item = S>> From<I> for ChartLabels {
    fn from(labels: I) -> Self {
        ChartLabels(labels.into_iter().map(Into::into).collect())
    }
}

/// One pie slice (gpuicn's `.slice(label, value)` / `.slice_with_color(..)`).
#[derive(Clone, Debug, PartialEq)]
pub struct ChartSlice {
    label: SharedString,
    value: f64,
    color: Option<Color>,
}

impl ChartSlice {
    pub fn new(label: impl Into<SharedString>, value: f64) -> Self {
        Self { label: label.into(), value, color: None }
    }

    pub fn with_color(label: impl Into<SharedString>, value: f64, color: impl Into<Color>) -> Self {
        Self { label: label.into(), value, color: Some(color.into()) }
    }

    /// Override the theme chart color.
    pub fn color(mut self, color: impl Into<Color>) -> Self {
        self.color = Some(color.into());
        self
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Bar,
    Line,
    Area,
}

/// Number of horizontal grid intervals.
const GRID_LINES: usize = 4;

/// Plot width used before the plot has been laid out (and on targets that
/// don't report layout, like HTML): a chart in a 400px card.
const FALLBACK_WIDTH: f32 = 352.0;

/// A "nice" axis maximum at or above `value`: [`GRID_LINES`] × a round tick
/// step (1, 1.5, 2, 2.5, 3, 4, 5, 6, 8 × 10ⁿ), so the data fills most of the
/// plot the way Recharts' automatic domain does.
pub fn nice_max(value: f64) -> f64 {
    if value <= 0. {
        return 1.;
    }
    let raw = value / GRID_LINES as f64;
    let base = 10f64.powf(raw.log10().floor());
    let step = [1., 1.5, 2., 2.5, 3., 4., 5., 6., 8., 10.]
        .into_iter()
        .map(|m| m * base)
        .find(|step| *step >= raw)
        .unwrap_or(10. * base);
    step * GRID_LINES as f64
}

/// A color as an SVG paint (`#rrggbbaa`).
fn hex(c: Color) -> String {
    let (r, g, b, a) = c.to_rgba8();
    if a == 255 { format!("#{r:02x}{g:02x}{b:02x}") } else { format!("#{r:02x}{g:02x}{b:02x}{a:02x}") }
}

/// A rectangle with only its top corners rounded (a bar), as path data.
fn bar_path(x: f32, top: f32, w: f32, bottom: f32, radius: f32) -> String {
    let h = (bottom - top).max(0.0);
    let r = radius.min(w / 2.0).min(h).max(0.0);
    if r <= 0.0 {
        return format!("M{x},{bottom}V{top}H{}V{bottom}Z", x + w);
    }
    format!(
        "M{x},{bottom}V{}A{r},{r} 0 0 1 {},{top}H{}A{r},{r} 0 0 1 {},{}V{bottom}Z",
        top + r,
        x + r,
        x + w - r,
        x + w,
        top + r
    )
}

/// Monotone cubic interpolation (Fritsch–Carlson, like Recharts'
/// `type="monotone"`) through `points`, appended to path data that is
/// already at `points[0]`: smooth, but never overshoots the data, so a low
/// point doesn't dip below its value. Works for points in either x direction.
fn smooth(d: &mut String, points: &[(f32, f32)]) {
    use std::fmt::Write;
    let n = points.len();
    if n < 2 {
        return;
    }
    let (xs, ys): (Vec<f32>, Vec<f32>) = points.iter().copied().unzip();
    let secants: Vec<f32> = (0..n - 1)
        .map(|i| {
            let dx = xs[i + 1] - xs[i];
            if dx == 0. { 0. } else { (ys[i + 1] - ys[i]) / dx }
        })
        .collect();
    let mut tangents = vec![0f32; n];
    tangents[0] = secants[0];
    tangents[n - 1] = secants[n - 2];
    for i in 1..n - 1 {
        tangents[i] = if secants[i - 1] * secants[i] <= 0. { 0. } else { (secants[i - 1] + secants[i]) / 2. };
    }
    for i in 0..n - 1 {
        if secants[i] == 0. {
            tangents[i] = 0.;
            tangents[i + 1] = 0.;
            continue;
        }
        let (a, b) = (tangents[i] / secants[i], tangents[i + 1] / secants[i]);
        let s = a * a + b * b;
        if s > 9. {
            let tau = 3. / s.sqrt();
            tangents[i] = tau * a * secants[i];
            tangents[i + 1] = tau * b * secants[i];
        }
    }
    for i in 0..n - 1 {
        let dx = (xs[i + 1] - xs[i]) / 3.;
        let (c1x, c1y) = (xs[i] + dx, ys[i] + tangents[i] * dx);
        let (c2x, c2y) = (xs[i + 1] - dx, ys[i + 1] - tangents[i + 1] * dx);
        let _ = write!(d, " C{c1x},{c1y} {c2x},{c2y} {},{}", xs[i + 1], ys[i + 1]);
    }
}

fn format_value(value: f64) -> String {
    if value.fract() == 0. { format!("{}", value as i64) } else { format!("{value:.2}") }
}

/// The plot of a cartesian chart, `width` × `height` px.
#[allow(clippy::too_many_arguments)]
fn plot_svg(
    kind: Kind,
    series: &[Vec<f64>],
    colors: &[Color],
    max: f64,
    width: f32,
    height: f32,
    stacked: bool,
    grid: bool,
    dots: bool,
    hovered: Option<usize>,
    grid_color: Color,
    hover_bg: Color,
    fill_opacity: f32,
) -> Element {
    let points = series.iter().map(|s| s.len()).max().unwrap_or(0);
    let n = points.max(1);
    let step = width / n as f32;
    let y_for = |v: f64| height - height * (v / max) as f32;
    let mut shapes: Vec<Element> = Vec::new();

    if grid {
        // `<CartesianGrid vertical={false} />`: solid horizontal lines.
        for line in 0..=GRID_LINES {
            let y = height * (line as f32 / GRID_LINES as f32);
            shapes.push(<line x1="0" y1={y} x2={width} y2={y} stroke={hex(grid_color)} stroke-width="1" />);
        }
    }
    if let Some(ix) = hovered.filter(|_| kind == Kind::Bar) {
        shapes.push(<rect x={step * ix as f32} y="0" width={step} height={height} fill={hex(hover_bg)} />);
    }

    match kind {
        Kind::Bar => {
            let groups = if stacked { 1 } else { series.len().max(1) };
            let inner = step * 0.8;
            let bar_w = (inner / groups as f32 - 2.0).max(1.0);
            for i in 0..points {
                let mut base = 0.;
                for (s, data) in series.iter().enumerate() {
                    let v = data[i];
                    let slot = if stacked { 0 } else { s };
                    let x = step * i as f32 + step * 0.1 + (bar_w + 2.0) * slot as f32;
                    let (top, bottom) = if stacked { (y_for(base + v), y_for(base)) } else { (y_for(v), y_for(0.)) };
                    let radius = if !stacked || s + 1 == series.len() { 4.0 } else { 0.0 };
                    shapes.push(<path d={bar_path(x, top, bar_w, bottom, radius)} fill={hex(colors[s])} />);
                    if stacked {
                        base += v;
                    }
                }
            }
        }
        Kind::Line | Kind::Area => {
            let mut baseline = vec![0f64; points];
            let mut layers: Vec<(Vec<(f32, f32)>, Vec<(f32, f32)>)> = Vec::new();
            for data in series {
                let tops: Vec<f64> = (0..points).map(|i| if stacked { baseline[i] + data[i] } else { data[i] }).collect();
                let x = |i: usize| step * (i as f32 + 0.5);
                let pts: Vec<(f32, f32)> = (0..points).map(|i| (x(i), y_for(tops[i]))).collect();
                let base: Vec<(f32, f32)> =
                    (0..points).map(|i| (x(i), y_for(if stacked { baseline[i] } else { 0. }))).collect();
                if stacked {
                    baseline = tops;
                }
                layers.push((pts, base));
            }
            for (s, (pts, base)) in layers.iter().enumerate().rev() {
                if pts.is_empty() {
                    continue;
                }
                let color = hex(colors[s]);
                if kind == Kind::Area {
                    let mut d = format!("M{},{} L{},{}", base[0].0, base[0].1, pts[0].0, pts[0].1);
                    smooth(&mut d, pts);
                    // Follow the (smoothed) layer below exactly, so stacked
                    // fills meet without seams or overlaps.
                    let back: Vec<(f32, f32)> = base.iter().rev().copied().collect();
                    d.push_str(&format!(" L{},{}", back[0].0, back[0].1));
                    smooth(&mut d, &back);
                    d.push_str(" Z");
                    // `fillOpacity={0.4}`; a touch lighter on dark backgrounds,
                    // where saturated fills read heavier.
                    shapes.push(<path d={d} fill={color.clone()} fill-opacity={fill_opacity} />);
                }
                let mut d = format!("M{},{}", pts[0].0, pts[0].1);
                smooth(&mut d, pts);
                shapes.push(<path d={d} fill="none" stroke={color.clone()} stroke-width="2" />);
                if dots || hovered.is_some() {
                    for (i, p) in pts.iter().enumerate() {
                        if dots || hovered == Some(i) {
                            let r = if hovered == Some(i) { 5.0 } else { 4.0 };
                            shapes.push(<circle cx={p.0} cy={p.1} r={r} fill={color.clone()} />);
                        }
                    }
                }
            }
        }
    }

    <svg class="absolute top-0 left-0" width={width} height={height} viewBox={format!("0 0 {width} {height}")}>
        {...shapes}
    </svg>
}

/// The shared body of [`BarChart`], [`LineChart`] and [`AreaChart`].
#[allow(clippy::too_many_arguments)]
fn cartesian(
    kind: Kind,
    labels: Option<Accessor<ChartLabels>>,
    series: Option<Accessor<Vec<ChartSeries>>>,
    height: f32,
    stacked: bool,
    legend: bool,
    grid: bool,
    dots: bool,
    label_every: usize,
    attrs: Attributes,
) -> Element {
    let t = theme();
    let hovered = signal(None::<usize>);
    let plot = NodeRef::new();
    let label_every = label_every.max(1);
    let attrs = StoredValue::new(attrs);

    (move || {
        let theme = t.get();
        let labels = labels.map(|l| l.get()).unwrap_or_default().0;
        let all = series.map(|s| s.get()).unwrap_or_default();
        let palette = theme.charts();
        let colors: Vec<Color> =
            all.iter().enumerate().map(|(ix, s)| s.color.unwrap_or(palette[ix % palette.len()])).collect();
        let points = labels.len().max(all.iter().map(|s| s.data.len()).max().unwrap_or(0));
        let value = |s: usize, i: usize| all[s].data.get(i).copied().unwrap_or(0.);
        let max = if stacked {
            (0..points).map(|i| (0..all.len()).map(|s| value(s, i)).sum::<f64>()).fold(0., f64::max)
        } else {
            all.iter().flat_map(|s| s.data.iter().copied()).fold(0., f64::max)
        };
        let max = nice_max(max);
        let data: Rc<Vec<Vec<f64>>> = Rc::new((0..all.len()).map(|s| (0..points).map(|i| value(s, i)).collect()).collect());
        let grid_color = theme.border;
        let fill_opacity = if theme.is_dark() { 0.3 } else { 0.4 };
        let hover_bg = theme.muted.opacity(0.5);
        let shared = Rc::new((labels.clone(), all.clone(), colors.clone()));

        let svg = {
            let (data, colors) = (data.clone(), colors.clone());
            move || {
                let width = plot.bounds().map(|b| b.size.width).filter(|w| *w > 0.0).unwrap_or(FALLBACK_WIDTH);
                plot_svg(kind, &data, &colors, max, width, height, stacked, grid, dots, hovered.get(), grid_color, hover_bg, fill_opacity)
            }
        };

        let tooltip = {
            let theme = theme.clone();
            let shared = shared.clone();
            move || {
                let (labels, all, colors) = &*shared;
                hovered.get().filter(|ix| *ix < points).map(|ix| {
                    let left = relative((ix as f32 + 0.5) / points.max(1) as f32);
                    // rsx#7: no `shadow-xl`. rsx#8: the value isn't in the mono font.
                    <div
                        class="absolute top-0 ml-2 min-w-32 flex flex-col gap-1.5 px-2.5 py-1.5 border text-xs leading-none"
                        left={left}
                        corner_radius={theme.radius_lg()}
                        border_color={theme.border.opacity(0.5)}
                        bg={theme.background}
                    >
                        {labels.get(ix).map(|l| <div class="font-medium">{l.clone()}</div>)}
                        {...all.iter().enumerate().map(|(s, series)| <div class="flex items-center gap-2">
                            <div class="size-2.5 flex-none rounded-[2px]" bg={colors[s]} />
                            <div class="flex-1 min-w-0" text_color={theme.muted_foreground}>{series.label.clone()}</div>
                            <div class="font-medium">{format_value(series.data.get(ix).copied().unwrap_or(0.))}</div>
                        </div>)}
                    </div>
                })
            }
        };

        let legend_items: Vec<(SharedString, Color)> = all.iter().zip(colors.iter()).map(|(s, c)| (s.label.clone(), *c)).collect();
        <div class="flex flex-col gap-2 w-full" {...attrs.with_value(|a| a.clone())}>
            <div
                ref={plot}
                class="relative w-full"
                h={px(height)}
                onMouseMove={move |e: &rsx::MouseMoveEvent| {
                    let Some(b) = plot.bounds() else { return };
                    let inside = e.position.x >= b.origin.x
                        && e.position.x < b.origin.x + b.size.width
                        && e.position.y >= b.origin.y
                        && e.position.y < b.origin.y + b.size.height;
                    let ix = (inside && points > 0 && b.size.width > 0.0)
                        .then(|| (((e.position.x - b.origin.x) / b.size.width) * points as f32).floor() as usize)
                        .map(|ix| ix.min(points - 1));
                    if hovered.get_untracked() != ix {
                        hovered.set(ix);
                    }
                }}
                onHover={move |h: &bool| if !*h { hovered.set(None) }}
            >
                {svg}
                {tooltip}
            </div>
            <div class="flex w-full text-xs leading-none" text_color={theme.muted_foreground}>
                {...labels.iter().enumerate().map(|(ix, label)| {
                    let text: String = if ix % label_every == 0 { label.chars().take(3).collect() } else { String::new() };
                    <div class="flex-1 min-w-0 text-center overflow-hidden">{text}</div>
                })}
            </div>
            {(legend && all.len() > 1).then(|| <ChartLegend items={legend_items} />)}
        </div>
    })
    .into_element()
}

macro_rules! cartesian_chart {
    ($name:ident, $kind:expr, $doc:literal) => {
        #[doc = $doc]
        #[component]
        pub fn $name(
            /// X-axis category labels.
            labels: Option<Accessor<ChartLabels>>,
            series: Option<Accessor<Vec<ChartSeries>>>,
            /// Height of the plot area (default 200px).
            #[prop(default = 200.0)]
            height: f32,
            /// Stack series on top of each other.
            stacked: bool,
            #[prop(default = true)]
            legend: bool,
            #[prop(default = true)]
            grid: bool,
            /// Draw dots on line/area points.
            dots: bool,
            /// Show only every n-th x label (e.g. abbreviate long series).
            #[prop(default = 1)]
            label_every: usize,
            attrs: Attributes,
        ) -> impl IntoElement {
            cartesian($kind, labels, series, height, stacked, legend, grid, dots, label_every, attrs)
        }
    };
}

cartesian_chart!(BarChart, Kind::Bar, "A bar chart (grouped or stacked).");
cartesian_chart!(LineChart, Kind::Line, "A smooth line chart.");
cartesian_chart!(AreaChart, Kind::Area, "A filled area chart.");

/// Colored squares with labels under a chart (`ChartLegend`).
#[component]
pub fn ChartLegend(items: Vec<(SharedString, Color)>, attrs: Attributes) -> impl IntoElement {
    <div class="flex items-center justify-center gap-4 pt-3 text-xs leading-none" {...attrs}>
        {...items.into_iter().map(|(label, color)| <div class="flex items-center gap-1.5">
            <div class="size-2 flex-none rounded-[2px]" bg={color} />
            {label}
        </div>)}
    </div>
}

/// A pie or donut chart.
#[component]
pub fn PieChart(
    slices: Option<Accessor<Vec<ChartSlice>>>,
    /// Diameter (default 250px).
    #[prop(default = 250.0)]
    size: f32,
    /// Inner radius as a fraction of the outer radius (0 = pie).
    #[prop(optional)]
    donut: f32,
    /// Big number + caption in the middle of a donut.
    center_label: Option<(String, String)>,
    #[prop(default = true)]
    legend: bool,
    attrs: Attributes,
) -> impl IntoElement {
    let t = theme();
    let inner = donut.clamp(0., 0.95);
    let center_label = StoredValue::new(center_label);
    move || {
        let theme = t.get();
        let slices = slices.map(|s| s.get()).unwrap_or_default();
        let palette = theme.charts();
        let colors: Vec<Color> =
            slices.iter().enumerate().map(|(ix, s)| s.color.unwrap_or(palette[ix % palette.len()])).collect();
        let values: Vec<f64> = slices.iter().map(|s| s.value.max(0.)).collect();
        let total: f64 = values.iter().sum::<f64>().max(f64::EPSILON);

        let r = size / 2.0;
        let ri = r * inner;
        let at = |radius: f32, angle: f64| (r + radius * angle.sin() as f32, r - radius * angle.cos() as f32);
        let mut shapes: Vec<Element> = Vec::new();
        let mut start = 0f64;
        let mut boundaries = Vec::new();
        for (ix, value) in values.iter().enumerate() {
            let sweep = value / total * std::f64::consts::TAU;
            if sweep <= 0. {
                continue;
            }
            let end = start + sweep.min(std::f64::consts::TAU - 1e-4);
            let large = (sweep > std::f64::consts::PI) as u8;
            let (s0, s1) = (at(r, start), at(r, end));
            let d = if ri > 0.0 {
                let (i0, i1) = (at(ri, start), at(ri, end));
                format!(
                    "M{},{} L{},{} A{r},{r} 0 {large} 1 {},{} L{},{} A{ri},{ri} 0 {large} 0 {},{} Z",
                    i0.0, i0.1, s0.0, s0.1, s1.0, s1.1, i1.0, i1.1, i0.0, i0.1
                )
            } else {
                format!("M{r},{r} L{},{} A{r},{r} 0 {large} 1 {},{} Z", s0.0, s0.1, s1.0, s1.1)
            };
            shapes.push(<path d={d} fill={hex(colors[ix])} />);
            boundaries.push(start);
            start += sweep;
        }
        // Separators go on top of every slice (Recharts' background stroke),
        // so each boundary looks the same.
        if boundaries.len() > 1 {
            for angle in boundaries {
                let (a, b) = (at(ri.max(0.0), angle), at(r + 1.0, angle));
                shapes.push(<line x1={a.0} y1={a.1} x2={b.0} y2={b.1} stroke={hex(theme.background)} stroke-width="2" />);
            }
        }

        let legend_items: Vec<(SharedString, Color)> = slices.iter().zip(colors.iter()).map(|(s, c)| (s.label.clone(), *c)).collect();
        <div class="flex flex-col items-center" {...&attrs}>
            <div class="relative flex-none" w={px(size)} h={px(size)}>
                <svg class="absolute top-0 left-0" width={size} height={size} viewBox={format!("0 0 {size} {size}")}>
                    {...shapes}
                </svg>
                {center_label.get_value().map(|(value, caption)| <div class="absolute inset-0 flex flex-col items-center justify-center">
                    <div class="text-3xl leading-[2.25rem] font-bold">{value}</div>
                    <div class="text-sm leading-[1.25rem]" text_color={theme.muted_foreground}>{caption}</div>
                </div>)}
            </div>
            {legend.then(|| <ChartLegend items={legend_items} />)}
        </div>
    }
}
