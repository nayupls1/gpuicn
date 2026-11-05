use rsx::prelude::*;

use crate::ui::button::Button;
use crate::ui::checkbox::Checkbox;
use crate::ui::field::{Field, FieldGroup, FieldSeparator, FieldSet};
use crate::ui::input::Input;
use crate::ui::native_select::{option, NativeSelect};
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
                        <NativeSelect placeholder="MM" options={["01", "02", "03", "12"].map(|m| option(m, m)).to_vec()} />
                    </Field>
                    <Field label="Year" class="flex-1 min-w-0">
                        <NativeSelect placeholder="YYYY" options={["2026", "2027", "2028"].map(|y| option(y, y)).to_vec()} />
                    </Field>
                    <Field label="CVV" class="flex-1 min-w-0"><Input placeholder="123" /></Field>
                </div>
                <FieldSeparator />
                <FieldSet legend="Billing Address" description="The billing address associated with your payment method">
                    <Field horizontal label="Same as shipping address">
                        <Checkbox default_checked />
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
