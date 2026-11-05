use rsx::prelude::*;

use crate::ui::card::{Card, CardContent};
use crate::ui::carousel::Carousel;

pub fn demo() -> Element {
    <div class="px-12">
        <Carousel
            class="w-[320px]"
            items={(1..=5).map(|n| {
                // `p-1` around each slide keeps the card's shadow and corners visible.
                <div class="p-1">
                    <Card>
                        <CardContent class="flex h-[248px] items-center justify-center text-[36px] leading-[40px] font-semibold">
                            {n.to_string()}
                        </CardContent>
                    </Card>
                </div>
            }).collect::<Vec<_>>()}
        />
    </div>
}
