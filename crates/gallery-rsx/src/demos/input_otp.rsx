use rsx::prelude::*;

use crate::ui::input_otp::InputOtp;
use crate::ui::theme::theme;

pub fn demo() -> Element {
    let t = theme();
    let entered = signal(None::<String>);
    <div class="flex flex-col gap-4">
        <InputOtp length={6usize} groups={[3, 3]} onComplete={move |code: &String| entered.set(Some(code.clone()))} />
        {/* TODO(sonner): `toast(format!("Code entered: {code}"))` instead of this line once `sonner` is ported. */}
        {move || entered.get().map(|code| <div class="text-sm" text_color={t.get().muted_foreground}>Code entered: {code}</div>)}
    </div>
}
