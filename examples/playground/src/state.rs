use leptos::prelude::{StoredValue, Update, Write, WriteValue};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use reactive_stores::Store;
use reactive_stores_macro::Store as StoreMacro;

use crate::{
    item::{Item, Random},
    store,
};

#[derive(StoreMacro, Debug)]
pub struct State {
    #[store(key: usize = |item| item.id)]
    items: Vec<Item>,
    #[store(skip)]
    rng: StoredValue<StdRng>,
}

impl State {
    pub fn new(seed: u64) -> Self {
        const INITIAL_ITEM_COUNT: usize = 24;
        let mut rng = StdRng::seed_from_u64(seed);
        Self {
            items: (0..INITIAL_ITEM_COUNT)
                .map(|_| Item::random(&mut rng))
                .collect(),
            rng: StoredValue::new(rng),
        }
    }

    fn new_item(&self) -> Item {
        let mut rng = self.rng.write_value();
        Item::random(&mut *rng)
    }
}

pub trait Utils
where
    Self: store::Utils<State>,
{
    fn reset_rng(&self, seed: u64) {
        *self.write_untracked().rng.write_value() = StdRng::seed_from_u64(seed);
    }

    fn shuffle_items(&self) {
        let rng = self.read().rng;
        self.items().write().shuffle(&mut *rng.write_value());
    }

    fn add_item_at_start(&self) {
        let item = self.write_untracked().new_item();
        self.items().write().insert(0, item);
    }

    fn add_item_at_end(&self) {
        let item = self.write_untracked().new_item();
        self.items().write().push(item);
    }

    fn add_item_at_random(&self) {
        let state = self.read();
        let item = state.new_item();
        let items_len = state.items.len();
        let index = state.rng.write_value().random_range(0..=items_len);
        drop(state);
        self.items().update(|items| items.insert(index, item));
    }

    fn remove_item_by_id(&self, id: usize) {
        self.items().update(|items| {
            items.retain(|item| item.id != id);
        });
    }
}

impl Utils for Store<State> {}
