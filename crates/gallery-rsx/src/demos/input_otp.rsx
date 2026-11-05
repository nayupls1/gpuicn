use rsx::prelude::*;

use crate::ui::input_otp::InputOtp;
use crate::ui::sonner::toast;

pub fn demo() -> Element {
    <InputOtp length={6} groups={[3, 3]} onComplete={move |code: &String| {
        toast(format!("Code entered: {code}"));
    }} />
}
