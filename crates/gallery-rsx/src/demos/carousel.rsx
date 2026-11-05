use rsx::prelude::*;

use crate::ui::carousel::Carousel;
use crate::ui::theme::theme;

pub fn demo() -> Element {
    let t = theme();
    <div class="px-12">
        <Carousel
            class="w-[320px]"
            items={(1..=5).map(|n| {
                // `p-1` around each slide keeps the card's shadow and corners visible.
                // TODO(card): `<Card><CardContent class="flex h-[248px] items-center justify-center text-[36px] leading-[40px] font-semibold">{n}</CardContent></Card>` once `card` is ported.
                <div class="p-1">
                    <div class="flex flex-col py-6 border rounded-xl" bg={t.get().card} text_color={t.get().card_foreground} border_color={t.get().border}>
                        <div class="flex px-6 h-[248px] items-center justify-center text-[36px] leading-[40px] font-semibold">{n.to_string()}</div>
                    </div>
                </div>
            }).collect::<Vec<_>>()}
        />
    </div>
}
