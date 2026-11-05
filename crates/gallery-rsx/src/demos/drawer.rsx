use rsx::prelude::*;

use crate::ui::button::{Button, ButtonSize};
use crate::ui::dialog::DialogHandle;
use crate::ui::drawer::Drawer;
use crate::ui::icon::IconName;
use crate::ui::theme::{theme, ColorExt};

fn goal_picker() -> Element {
    let t = theme();
    let goal = signal(350i32);
    let step = move |icon: IconName, delta: i32| {
        <Button
            outline
            size={ButtonSize::IconSm}
            class="rounded-full"
            icon={icon}
            disabled={if delta < 0 { goal.get() <= 200 } else { goal.get() >= 400 }}
            onClick={move |_| goal.update(|g| *g = (*g + delta).clamp(200, 400))}
        />
    };
    let data = [400, 300, 200, 300, 200, 278, 189, 239, 300, 200, 278, 189, 349];
    <div class="flex flex-col items-center gap-6 pb-2">
        <div class="flex items-center flex-nowrap gap-6">
            {step(IconName::Minus, -10)}
            <div class="flex flex-col items-center w-[160px]">
                <div class="text-[72px] leading-[72px] font-bold">{goal}</div>
                <div class="text-xs leading-[16px]" text_color={t.get().muted_foreground}>CALORIES/DAY</div>
            </div>
            {step(IconName::Plus, 10)}
        </div>
        <div class="flex items-end gap-1 h-[120px] w-[320px]">
            {data.map(|v| <div
                class="flex-1"
                h={px(v as f32 * 0.3)}
                class="rounded-t-[4px]"
                bg={t.get().foreground.opacity(0.9)}
            />)}
        </div>
    </div>
}

pub fn demo() -> Element {
    <Drawer
        trigger={<Button outline label="Open Drawer" />}
        title="Move Goal"
        description="Set your daily activity goal."
        content={|_| goal_picker()}
        footer={|drawer: DialogHandle| <div class="flex flex-col gap-2">
            <Button label="Submit" class="w-full" />
            <Button outline label="Cancel" class="w-full" onClick={move |_| drawer.close()} />
        </div>}
    />
}
