use rsx::prelude::*;

use crate::ui::breadcrumb::{Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem};

pub fn demo() -> Element {
    <Breadcrumb>
        <BreadcrumbItem label="Home" onClick={move |_| {}} />
        <BreadcrumbEllipsis />
        <BreadcrumbItem label="Components" onClick={move |_| {}} />
        <BreadcrumbItem label="Breadcrumb" current />
    </Breadcrumb>
}
