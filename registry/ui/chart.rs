//! Beautiful charts: bar, line, area and pie — drawn with gpui paths and
//! colored by the theme's `chart_1..chart_5` tokens.
//!
//! ```ignore
//! BarChart::new("visitors")
//!     .labels(["January", "February", "March", "April", "May", "June"])
//!     .series(ChartSeries::new("Desktop").data([186., 305., 237., 73., 209., 214.]))
//!     .series(ChartSeries::new("Mobile").data([80., 200., 120., 190., 130., 140.]))
//!
//! LineChart::new("trend").labels(..).series(..).dots(true)
//! AreaChart::new("traffic").labels(..).series(..).stacked()
//! PieChart::new("browsers").slice("Chrome", 275.).slice("Safari", 200.).donut(0.6)
//! ```

use gpui::{
    App, Bounds, ElementId, FontWeight, Hsla, IntoElement, PathBuilder, Pixels, Point, RenderOnce,
    SharedString, StyleRefinement, Window, canvas, div, fill, point, prelude::*, px, relative,
};

use super::theme::ActiveTheme;
use super::utils::{StyledExt, on_mouse_leave_window};

/// One data series (a bar color, a line…).
#[derive(Clone)]
pub struct ChartSeries {
    label: SharedString,
    color: Option<Hsla>,
    data: Vec<f64>,
}

impl ChartSeries {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self { label: label.into(), color: None, data: Vec::new() }
    }

    /// Override the theme chart color.
    pub fn color(mut self, color: impl Into<Hsla>) -> Self {
        self.color = Some(color.into());
        self
    }

    pub fn data(mut self, data: impl IntoIterator<Item = f64>) -> Self {
        self.data = data.into_iter().collect();
        self
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Bar,
    Line,
    Area,
}

#[derive(Default)]
struct HoverState {
    plot: Bounds<Pixels>,
    hovered: Option<usize>,
}

/// A "nice" axis maximum (1, 2, 2.5, 5 × 10ⁿ) above `value`.
pub fn nice_max(value: f64) -> f64 {
    if value <= 0. {
        return 1.;
    }
    let exp = value.log10().floor();
    let base = 10f64.powf(exp);
    for step in [1., 2., 2.5, 5., 10.] {
        if step * base >= value {
            return step * base;
        }
    }
    10. * base
}

macro_rules! cartesian_chart {
    ($name:ident, $kind:expr, $doc:literal) => {
        #[doc = $doc]
        #[derive(IntoElement)]
        pub struct $name {
            chart: Cartesian,
        }

        impl $name {
            pub fn new(id: impl Into<ElementId>) -> Self {
                Self { chart: Cartesian::new(id.into(), $kind) }
            }

            /// X-axis category labels.
            pub fn labels(mut self, labels: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
                self.chart.labels = labels.into_iter().map(Into::into).collect();
                self
            }

            pub fn series(mut self, series: ChartSeries) -> Self {
                self.chart.series.push(series);
                self
            }

            /// Height of the plot area (default 200px).
            pub fn height(mut self, height: impl Into<Pixels>) -> Self {
                self.chart.height = height.into();
                self
            }

            /// Stack series on top of each other.
            pub fn stacked(mut self) -> Self {
                self.chart.stacked = true;
                self
            }

            pub fn legend(mut self, legend: bool) -> Self {
                self.chart.legend = legend;
                self
            }

            pub fn grid(mut self, grid: bool) -> Self {
                self.chart.grid = grid;
                self
            }

            /// Draw dots on line/area points.
            pub fn dots(mut self, dots: bool) -> Self {
                self.chart.dots = dots;
                self
            }

            /// Show only every n-th x label (e.g. abbreviate long series).
            pub fn label_every(mut self, n: usize) -> Self {
                self.chart.label_every = n.max(1);
                self
            }
        }

        impl gpui::Styled for $name {
            fn style(&mut self) -> &mut StyleRefinement {
                &mut self.chart.style
            }
        }

        impl RenderOnce for $name {
            fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
                self.chart.render(window, cx)
            }
        }
    };
}

cartesian_chart!(BarChart, Kind::Bar, "A bar chart (grouped or stacked).");
cartesian_chart!(LineChart, Kind::Line, "A smooth line chart.");
cartesian_chart!(AreaChart, Kind::Area, "A filled area chart.");

struct Cartesian {
    id: ElementId,
    kind: Kind,
    labels: Vec<SharedString>,
    series: Vec<ChartSeries>,
    height: Pixels,
    stacked: bool,
    legend: bool,
    grid: bool,
    dots: bool,
    label_every: usize,
    style: StyleRefinement,
}

impl Cartesian {
    fn new(id: ElementId, kind: Kind) -> Self {
        Self {
            id,
            kind,
            labels: Vec::new(),
            series: Vec::new(),
            height: px(200.),
            stacked: false,
            legend: true,
            grid: true,
            dots: false,
            label_every: 1,
            style: StyleRefinement::default(),
        }
    }

    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state(self.id.clone(), cx, |_, _| HoverState::default());
        let hovered = state.read(cx).hovered;
        let theme = cx.theme();
        let palette = theme.charts();
        let colors: Vec<Hsla> = self
            .series
            .iter()
            .enumerate()
            .map(|(ix, s)| s.color.unwrap_or(palette[ix % palette.len()]))
            .collect();
        let points = self.labels.len().max(self.series.iter().map(|s| s.data.len()).max().unwrap_or(0));
        let value = |s: usize, i: usize| self.series[s].data.get(i).copied().unwrap_or(0.);
        let max = if self.stacked {
            (0..points).map(|i| (0..self.series.len()).map(|s| value(s, i)).sum::<f64>()).fold(0., f64::max)
        } else {
            self.series.iter().flat_map(|s| s.data.iter().copied()).fold(0., f64::max)
        };
        let max = nice_max(max);
        let series: Vec<Vec<f64>> = self.series.iter().map(|s| (0..points).map(|i| s.data.get(i).copied().unwrap_or(0.)).collect()).collect();
        let (kind, stacked, grid, dots) = (self.kind, self.stacked, self.grid, self.dots);
        let grid_color = theme.border;
        let hover_bg = theme.muted.opacity(0.5);

        let plot = {
            let state = state.clone();
            let colors = colors.clone();
            canvas(
                move |bounds, _, cx| state.update(cx, |s, _| s.plot = bounds),
                move |bounds, _, window, _| {
                    let n = points.max(1);
                    let step = bounds.size.width / n as f32;
                    let y_for = |v: f64| bounds.bottom() - bounds.size.height * (v / max) as f32;

                    if grid {
                        for line in 0..=4 {
                            let y = bounds.top() + bounds.size.height * (line as f32 / 4.);
                            let mut builder = PathBuilder::stroke(px(1.)).dash_array(&[px(3.), px(3.)]);
                            builder.move_to(point(bounds.left(), y));
                            builder.line_to(point(bounds.right(), y));
                            if let Ok(path) = builder.build() {
                                window.paint_path(path, grid_color);
                            }
                        }
                    }
                    if let Some(ix) = hovered.filter(|_| kind == Kind::Bar) {
                        window.paint_quad(fill(
                            Bounds::new(point(bounds.left() + step * ix as f32, bounds.top()), gpui::size(step, bounds.size.height)),
                            hover_bg,
                        ));
                    }

                    match kind {
                        Kind::Bar => {
                            let groups = if stacked { 1 } else { series.len().max(1) };
                            let inner = step * 0.8;
                            let bar_w = (inner / groups as f32 - px(2.)).max(px(1.));
                            for i in 0..points {
                                let mut base = 0.;
                                for (s, data) in series.iter().enumerate() {
                                    let v = data[i];
                                    let slot = if stacked { 0 } else { s };
                                    let x = bounds.left() + step * i as f32 + step * 0.1 + (bar_w + px(2.)) * slot as f32;
                                    let (top, bottom) = if stacked { (y_for(base + v), y_for(base)) } else { (y_for(v), y_for(0.)) };
                                    let radius = if !stacked || s + 1 == series.len() { px(4.) } else { px(0.) };
                                    window.paint_quad(
                                        fill(Bounds::from_corners(point(x, top), point(x + bar_w, bottom)), colors[s]).corner_radii(
                                            gpui::Corners { top_left: radius, top_right: radius, bottom_left: px(0.), bottom_right: px(0.) },
                                        ),
                                    );
                                    if stacked {
                                        base += v;
                                    }
                                }
                            }
                        }
                        Kind::Line | Kind::Area => {
                            let mut baseline = vec![0f64; points];
                            let mut layers: Vec<(Vec<Point<Pixels>>, Vec<Point<Pixels>>)> = Vec::new();
                            for data in &series {
                                let tops: Vec<f64> = (0..points).map(|i| if stacked { baseline[i] + data[i] } else { data[i] }).collect();
                                let pts: Vec<Point<Pixels>> = (0..points)
                                    .map(|i| point(bounds.left() + step * (i as f32 + 0.5), y_for(tops[i])))
                                    .collect();
                                let base: Vec<Point<Pixels>> = (0..points)
                                    .map(|i| point(bounds.left() + step * (i as f32 + 0.5), y_for(if stacked { baseline[i] } else { 0. })))
                                    .collect();
                                if stacked {
                                    baseline = tops;
                                }
                                layers.push((pts, base));
                            }
                            for (s, (pts, base)) in layers.iter().enumerate().rev() {
                                if pts.is_empty() {
                                    continue;
                                }
                                if kind == Kind::Area {
                                    let mut builder = PathBuilder::fill();
                                    builder.move_to(base[0]);
                                    builder.line_to(pts[0]);
                                    smooth(&mut builder, pts);
                                    for p in base.iter().rev() {
                                        builder.line_to(*p);
                                    }
                                    builder.close();
                                    if let Ok(path) = builder.build() {
                                        window.paint_path(path, colors[s].opacity(0.4));
                                    }
                                }
                                let mut builder = PathBuilder::stroke(px(2.));
                                builder.move_to(pts[0]);
                                smooth(&mut builder, pts);
                                if let Ok(path) = builder.build() {
                                    window.paint_path(path, colors[s]);
                                }
                                if dots || hovered.is_some() {
                                    for (i, p) in pts.iter().enumerate() {
                                        if dots || hovered == Some(i) {
                                            let r = px(3.);
                                            window.paint_quad(
                                                fill(Bounds::from_corners(*p - point(r, r), *p + point(r, r)), colors[s]).corner_radii(r),
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
            )
            .size_full()
        };

        let tooltip = hovered.filter(|ix| *ix < points).map(|ix| {
            let left = relative((ix as f32 + 0.5) / points.max(1) as f32);
            div()
                .absolute()
                .top_0()
                .left(left)
                .ml_2()
                .min_w_32()
                .flex()
                .flex_col()
                .gap_1p5()
                .px_2p5()
                .py_1p5()
                .rounded(theme.radius_lg())
                .border_1()
                .border_color(theme.border.opacity(0.5))
                .bg(theme.background)
                .text_xs()
                .when(theme.shadows, |this| this.shadow_xl())
                .children(self.labels.get(ix).map(|l| div().font_weight(FontWeight::MEDIUM).child(l.clone())))
                .children(self.series.iter().enumerate().map(|(s, series)| {
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(div().size_2p5().rounded(px(2.)).bg(colors[s]))
                        .child(div().flex_1().text_color(theme.muted_foreground).child(series.label.clone()))
                        .child(
                            div()
                                .font_family(theme.mono_font_family.clone())
                                .font_weight(FontWeight::MEDIUM)
                                .child(format_value(series.data.get(ix).copied().unwrap_or(0.))),
                        )
                }))
        });

        let move_state = state.clone();
        let leave_state = state.clone();
        let exit_state = state.clone();
        let label_every = self.label_every;
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap_2()
            .w_full()
            .refine_style(&self.style)
            .child(
                div()
                    .id("chart-plot")
                    .relative()
                    .w_full()
                    .h(self.height)
                    .child(plot)
                    .children(tooltip)
                    .child(on_mouse_leave_window(move |_, cx| {
                        exit_state.update(cx, |s, cx| {
                            s.hovered = None;
                            cx.notify();
                        })
                    }))
                    .on_mouse_move(move |event, _, cx| {
                        move_state.update(cx, |s, cx| {
                            let plot = s.plot;
                            let ix = if plot.contains(&event.position) && points > 0 {
                                Some((((event.position.x - plot.left()) / plot.size.width) * points as f32).floor() as usize)
                            } else {
                                None
                            };
                            let ix = ix.map(|ix| ix.min(points - 1));
                            if s.hovered != ix {
                                s.hovered = ix;
                                cx.notify();
                            }
                        })
                    })
                    .on_hover(move |hovered, _, cx| {
                        if !hovered {
                            leave_state.update(cx, |s, cx| {
                                s.hovered = None;
                                cx.notify();
                            })
                        }
                    }),
            )
            .child(div().flex().w_full().text_xs().text_color(theme.muted_foreground).children(
                self.labels.iter().enumerate().map(|(ix, label)| {
                    let text: SharedString = if ix % label_every == 0 { label.chars().take(3).collect::<String>().into() } else { "".into() };
                    div().flex_1().text_center().overflow_hidden().child(text)
                }),
            ))
            .when(self.legend && self.series.len() > 1, |this| {
                this.child(ChartLegend::new(self.series.iter().zip(colors.iter()).map(|(s, c)| (s.label.clone(), *c))))
            })
    }
}

/// Catmull-Rom spline through `points` (assumes the path is at `points[0]`).
fn smooth(builder: &mut PathBuilder, points: &[Point<Pixels>]) {
    for i in 0..points.len().saturating_sub(1) {
        let p0 = if i == 0 { points[0] } else { points[i - 1] };
        let (p1, p2) = (points[i], points[i + 1]);
        let p3 = points.get(i + 2).copied().unwrap_or(p2);
        let c1 = p1 + (p2 - p0) * (1. / 6.);
        let c2 = p2 - (p3 - p1) * (1. / 6.);
        builder.cubic_bezier_to(p2, c1, c2);
    }
}

fn format_value(value: f64) -> String {
    if value.fract() == 0. { format!("{}", value as i64) } else { format!("{value:.2}") }
}

/// Colored squares with labels under a chart (`ChartLegend`).
#[derive(IntoElement)]
pub struct ChartLegend {
    items: Vec<(SharedString, Hsla)>,
}

impl ChartLegend {
    pub fn new(items: impl IntoIterator<Item = (SharedString, Hsla)>) -> Self {
        Self { items: items.into_iter().collect() }
    }
}

impl RenderOnce for ChartLegend {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div().flex().items_center().justify_center().gap_4().pt_3().text_xs().children(self.items.into_iter().map(
            |(label, color)| div().flex().items_center().gap_1p5().child(div().size_2().rounded(px(2.)).bg(color)).child(label),
        ))
    }
}

/// A pie or donut chart.
#[derive(IntoElement)]
pub struct PieChart {
    id: ElementId,
    slices: Vec<(SharedString, f64, Option<Hsla>)>,
    size: Pixels,
    inner_radius: f32,
    center_label: Option<(SharedString, SharedString)>,
    legend: bool,
    style: StyleRefinement,
}

crate::impl_styled!(PieChart);

impl PieChart {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            slices: Vec::new(),
            size: px(250.),
            inner_radius: 0.,
            center_label: None,
            legend: true,
            style: StyleRefinement::default(),
        }
    }

    pub fn slice(mut self, label: impl Into<SharedString>, value: f64) -> Self {
        self.slices.push((label.into(), value, None));
        self
    }

    pub fn slice_with_color(mut self, label: impl Into<SharedString>, value: f64, color: impl Into<Hsla>) -> Self {
        self.slices.push((label.into(), value, Some(color.into())));
        self
    }

    /// Diameter (default 250px).
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into();
        self
    }

    /// Inner radius as a fraction of the outer radius (0 = pie).
    pub fn donut(mut self, inner_radius: f32) -> Self {
        self.inner_radius = inner_radius.clamp(0., 0.95);
        self
    }

    /// Big number + caption in the middle of a donut.
    pub fn center_label(mut self, value: impl Into<SharedString>, caption: impl Into<SharedString>) -> Self {
        self.center_label = Some((value.into(), caption.into()));
        self
    }

    pub fn legend(mut self, legend: bool) -> Self {
        self.legend = legend;
        self
    }
}

impl RenderOnce for PieChart {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let palette = theme.charts();
        let colors: Vec<Hsla> = self.slices.iter().enumerate().map(|(ix, s)| s.2.unwrap_or(palette[ix % palette.len()])).collect();
        let values: Vec<f64> = self.slices.iter().map(|s| s.1.max(0.)).collect();
        let total: f64 = values.iter().sum::<f64>().max(f64::EPSILON);
        let inner = self.inner_radius;
        let background = theme.background;
        let paint_colors = colors.clone();

        let pie = canvas(
            |_, _, _| {},
            move |bounds, _, window, _| {
                let center = bounds.center();
                let r = bounds.size.width.min(bounds.size.height) / 2.;
                let ri = r * inner;
                let at = |radius: Pixels, angle: f64| {
                    point(center.x + radius * angle.sin() as f32, center.y - radius * angle.cos() as f32)
                };
                let mut start = 0f64;
                for (ix, value) in values.iter().enumerate() {
                    let sweep = value / total * std::f64::consts::TAU;
                    if sweep <= 0. {
                        continue;
                    }
                    let end = start + sweep.min(std::f64::consts::TAU - 1e-4);
                    let large = sweep > std::f64::consts::PI;
                    let mut builder = PathBuilder::fill();
                    if ri > px(0.) {
                        builder.move_to(at(ri, start));
                        builder.line_to(at(r, start));
                        builder.arc_to(point(r, r), px(0.), large, true, at(r, end));
                        builder.line_to(at(ri, end));
                        builder.arc_to(point(ri, ri), px(0.), large, false, at(ri, start));
                    } else {
                        builder.move_to(center);
                        builder.line_to(at(r, start));
                        builder.arc_to(point(r, r), px(0.), large, true, at(r, end));
                    }
                    builder.close();
                    if let Ok(path) = builder.build() {
                        window.paint_path(path, paint_colors[ix]);
                    }
                    // Thin separators between slices.
                    let mut sep = PathBuilder::stroke(px(1.));
                    sep.move_to(at(ri, start));
                    sep.line_to(at(r, start));
                    if let Ok(path) = sep.build() {
                        window.paint_path(path, background);
                    }
                    start += sweep;
                }
            },
        )
        .size_full();

        div()
            .id(self.id)
            .flex()
            .flex_col()
            .items_center()
            .refine_style(&self.style)
            .child(
                div()
                    .relative()
                    .size(self.size)
                    .child(pie)
                    .children(self.center_label.map(|(value, caption)| {
                        div()
                            .absolute()
                            .inset_0()
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .child(div().text_3xl().font_weight(FontWeight::BOLD).child(value))
                            .child(div().text_sm().text_color(theme.muted_foreground).child(caption))
                    })),
            )
            .when(self.legend, |this| {
                this.child(ChartLegend::new(self.slices.iter().zip(colors.iter()).map(|(s, c)| (s.0.clone(), *c))))
            })
    }
}
