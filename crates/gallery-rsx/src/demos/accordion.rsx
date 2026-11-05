use rsx::prelude::*;

use crate::ui::accordion::{Accordion, AccordionItem};

pub fn demo() -> Element {
    <div class="h-[260px]">
        <Accordion default_value={vec!["item-1".into()]} class="w-[480px]">
            <AccordionItem value="item-1" title="Product Information">
                <div class="flex flex-col gap-4">
                    <div>Our flagship product combines cutting-edge technology with sleek design. Built with premium materials, it offers unparalleled performance and reliability.</div>
                    <div>Key features include advanced processing capabilities, and an intuitive user interface designed for both beginners and experts.</div>
                </div>
            </AccordionItem>
            <AccordionItem value="item-2" title="Shipping Details">
                <div>We offer worldwide shipping through trusted courier partners. Standard delivery takes 3-5 business days.</div>
            </AccordionItem>
            <AccordionItem value="item-3" title="Return Policy">
                <div>We stand behind our products with a comprehensive 30-day return policy.</div>
            </AccordionItem>
        </Accordion>
    </div>
}
