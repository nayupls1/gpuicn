use rsx::prelude::*;

use crate::ui::button::Button;
use crate::ui::field::{Field, FieldGroup, FieldSeparator, FieldSet};
use crate::ui::input::Input;
use crate::ui::textarea::Textarea;

pub fn demo() -> Element {
    <div class="w-[420px]">
        <FieldSet legend="Payment Method" description="All transactions are secure and encrypted">
            <FieldGroup>
                <Field label="Name on Card"><Input placeholder="Evil Rabbit" /></Field>
                <Field label="Card Number" description="Enter your 16-digit card number">
                    <Input placeholder="1234 5678 9012 3456" />
                </Field>
                <div class="flex items-center gap-4 flex-nowrap">
                    <Field label="Month" class="flex-1 min-w-0">
                        {/* TODO(native_select): `<NativeSelect placeholder="MM" options=[01, 02, 03, 12] />` once `native_select` is ported. */}
                        <select class="w-full h-9"><option value="">MM</option><option>01</option><option>02</option><option>03</option><option>12</option></select>
                    </Field>
                    <Field label="Year" class="flex-1 min-w-0">
                        {/* TODO(native_select): `<NativeSelect placeholder="YYYY" options=[2026, 2027, 2028] />` once `native_select` is ported. */}
                        <select class="w-full h-9"><option value="">YYYY</option><option>2026</option><option>2027</option><option>2028</option></select>
                    </Field>
                    <Field label="CVV" class="flex-1 min-w-0"><Input placeholder="123" /></Field>
                </div>
                <FieldSeparator />
                <FieldSet legend="Billing Address" description="The billing address associated with your payment method">
                    <Field horizontal label="Same as shipping address">
                        {/* TODO(checkbox): `<Checkbox default_checked />` once `checkbox` is ported. */}
                        <input type="checkbox" checked />
                    </Field>
                </FieldSet>
                <Field label="Comments"><Textarea placeholder="Add any additional comments" /></Field>
            </FieldGroup>
        </FieldSet>
        <div class="flex items-center gap-2 mt-6">
            <Button label="Submit" />
            <Button outline label="Cancel" />
        </div>
    </div>
}
