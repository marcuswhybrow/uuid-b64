use reactive_stores::{PatchField, StoreField};

use crate::UuidB64;

impl PatchField for UuidB64 {
    fn patch_field(
            &mut self,
            new: Self,
            path: &reactive_stores::StorePath,
            notify: &mut dyn FnMut(&reactive_stores::StorePath),
        ) {
        if new != *self {
            *self = new;
            notify(path);
        }
    }
}
