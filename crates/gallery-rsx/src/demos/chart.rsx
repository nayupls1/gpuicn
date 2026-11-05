use rsx::prelude::*;

use crate::ui::chart::{AreaChart, BarChart, ChartSeries, ChartSlice, LineChart, PieChart};
use crate::ui::icon::{Icon, IconName};
use crate::ui::theme::theme;

const MONTHS: [&str; 6] = ["January", "February", "March", "April", "May", "June"];
const DESKTOP: [f64; 6] = [186., 305., 237., 73., 209., 214.];
const MOBILE: [f64; 6] = [80., 200., 120., 190., 130., 140.];

/// A chart in a card with a title, a description and a trend footer.
fn card(title: &'static str, chart: Element) -> Element {
    let t = theme();
    <div
        class="flex flex-col gap-6 py-6 w-[400px] border"
        corner_radius={t.get().radius_xl()}
        border_color={t.get().border}
        bg={t.get().card}
        text_color={t.get().card_foreground}
    >
        {/* TODO(card): `<Card class="w-[400px]"><CardHeader><CardTitle>{title}</CardTitle>
            <CardDescription>January - June 2024</CardDescription></CardHeader>
            <CardContent>{chart}</CardContent><CardFooter class="flex-col items-start gap-1 text-sm leading-[20px]">..</CardFooter></Card>`
            once `card` is ported. */}
        <div class="flex flex-col gap-1.5 px-6">
            <div class="text-base font-semibold leading-none">{title}</div>
            <div class="text-sm leading-[1.25rem]" text_color={t.get().muted_foreground}>January - June 2024</div>
        </div>
        <div class="px-6">{chart}</div>
        <div class="flex flex-col items-start gap-1 px-6 text-sm leading-[20px]">
            <div class="flex items-center gap-2 font-medium">
                Trending up by 5.2% this month
                <Icon name={IconName::TrendingUp} />
            </div>
            <div text_color={t.get().muted_foreground}>Showing total visitors for the last 6 months</div>
        </div>
    </div>
}

pub fn demo() -> Element {
    <div class="w-[816px] flex flex-wrap gap-4">
        {card("Bar Chart - Multiple", <BarChart
            labels={MONTHS}
            series={vec![ChartSeries::new("Desktop").data(DESKTOP), ChartSeries::new("Mobile").data(MOBILE)]}
        />)}
        {card("Area Chart - Stacked", <AreaChart
            labels={MONTHS}
            stacked
            series={vec![ChartSeries::new("Desktop").data(DESKTOP), ChartSeries::new("Mobile").data(MOBILE)]}
        />)}
        {card("Line Chart - Dots", <LineChart labels={MONTHS} dots series={vec![ChartSeries::new("Desktop").data(DESKTOP)]} />)}
        {card("Pie Chart - Donut with Text", <PieChart
            slices={vec![
                ChartSlice::new("Chrome", 275.),
                ChartSlice::new("Safari", 200.),
                ChartSlice::new("Firefox", 187.),
                ChartSlice::new("Edge", 173.),
                ChartSlice::new("Other", 90.),
            ]}
            donut={0.6f32}
            size={220.0f32}
            center_label={("925".to_string(), "Visitors".to_string())}
        />)}
    </div>
}
