use pumpkin_world::item::Item;

#[allow(dead_code)]
pub struct PlayerInventory {
    // Main Inventory + Hotbar
    items: [Option<Item>; 36],
    armor: [Option<Item>; 4],
    offhand: Option<Item>,
    // current selected slot in hortbar
    selected: i16,
}

impl Default for PlayerInventory {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerInventory {
    pub fn new() -> Self {
        Self {
            items: [None; 36],
            armor: [None; 4],
            offhand: None,
            // TODO: What when player spawns in with an different index ?
            selected: 0,
        }
    }

    pub fn set_slot(_slot: u32, _item: Item) {}

    pub fn set_selected(&mut self, slot: i16) {
        assert!((0..9).contains(&slot));
        self.selected = slot;
    }
}
